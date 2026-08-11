//! The model API: CRUD over elements and relationships, with ownership-tree
//! invariants enforced (`REQ-MM-002`…`007`, `INV-MM-001/004/005`).
//!
//! [`Model`] wraps a [`smith_store::Store`] and holds the single write
//! connection (`REQ-PERS-012`). Every mutation runs in one `SQLite` transaction
//! (`REQ-PERS-013`): the element/relationship row and the closure rows commit
//! together, or neither does — on failure neither the projection nor the store
//! changes. All fallible operations return [`Result`]; no panic crosses the
//! boundary (`REQ-ARCH-017`, `REQ-ARCH-018`).

use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension};
use smith_core::{Comment, CommentError, ElementId, MetaclassKind, MetaclassScope, Visibility};
use smith_store::closure;

use crate::error::Error;

/// The `.smith` project model: a store plus the model API (`REQ-PERS-001`).
///
/// Created from a path via [`Model::open`]. Mutations go through the single
/// write connection (`REQ-PERS-012`) and commit in one transaction
/// (`REQ-PERS-013`). Reads query the live connection directly. The in-memory
/// petgraph projection ([`crate::projection::Projection`]) hydrates on open and
/// is updated write-through after every successful commit — they update
/// together or not at all (`REQ-ARCH-007`).
#[derive(Debug)]
pub struct Model {
    store: smith_store::Store,
    projection: crate::projection::Projection,
}

/// A snapshot of an element row, for reads (`REQ-PERS-001`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementView {
    /// Immutable identity.
    pub id: ElementId,
    /// Metaclass kind.
    pub kind: MetaclassKind,
    /// Owning namespace id (`None` only for the project root).
    pub owner: Option<ElementId>,
    /// Human-readable name (optional, not unique).
    pub name: Option<String>,
    /// UML visibility.
    pub visibility: Visibility,
}

/// A snapshot of a relationship row, for reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipView {
    /// Edge identity.
    pub id: ElementId,
    /// Relationship kind string.
    pub kind: String,
    /// Source element id.
    pub source: ElementId,
    /// Target element id.
    pub target: ElementId,
    /// Owning namespace id.
    pub owner: ElementId,
}

/// A snapshot of a comment row, for reads (`REQ-MM-011`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentView {
    /// Comment identity.
    pub id: ElementId,
    /// Owning element id.
    pub owner: ElementId,
    /// Non-empty comment body.
    pub body: String,
}

/// A request to create an element (the kind-agnostic common fields).
#[derive(Debug, Clone)]
pub struct CreateElement {
    /// Fresh identity minted by the caller.
    pub id: ElementId,
    /// Metaclass (must be a top-level ownable kind).
    pub kind: MetaclassKind,
    /// Optional name.
    pub name: Option<String>,
    /// Owning namespace (`None` only for the project root).
    pub owner: Option<ElementId>,
    /// UML visibility (defaults to public).
    pub visibility: Visibility,
}

/// A request to create a relationship.
#[derive(Debug, Clone)]
pub struct CreateRelationship {
    /// Fresh identity minted by the caller.
    pub id: ElementId,
    /// Relationship kind string (e.g. `"Association"`, `"Generalization"`).
    pub kind: String,
    /// Source element (must exist).
    pub source: ElementId,
    /// Target element (must exist).
    pub target: ElementId,
    /// Owning namespace (must exist).
    pub owner: ElementId,
}

/// RFC 3339 timestamp used for `created_at`/`updated_at`.
///
/// A fixed value keeps tests deterministic; a real clock would inject here.
const NOW: &str = "2026-01-01T00:00:00Z";

impl Model {
    /// Open (or create) a `.smith` project file.
    ///
    /// Wraps [`smith_store::open`] and hydrates the in-memory petgraph
    /// projection from `SQLite` (`REQ-PERS-002`). Store errors surface as
    /// [`Error::Store`]; corrupt rows surface as [`Error::CorruptStore`].
    ///
    /// # Errors
    ///
    /// - [`Error::Store`] when the underlying file cannot be opened or
    ///   migrated.
    /// - [`Error::CorruptStore`] when a row holds an unparseable id or
    ///   unknown kind during hydration.
    pub fn open(path: &Path) -> Result<Self, Error> {
        let store = smith_store::open(path).map_err(|detail| Error::Store {
            path: path.to_path_buf(),
            detail,
        })?;
        let projection = crate::projection::Projection::hydrate(store.connection())?;
        Ok(Self { store, projection })
    }

    /// The live `SQLite` connection (single writer; `REQ-PERS-012`).
    #[must_use]
    pub fn connection(&self) -> &Connection {
        self.store.connection()
    }

    /// The schema version this file was migrated to on open.
    #[must_use]
    pub fn schema_version(&self) -> u32 {
        self.store.schema_version()
    }

    /// The in-memory petgraph projection (`REQ-PERS-002`).
    ///
    /// The projection is a rebuildable cache; `SQLite` is the source of truth.
    /// Updated write-through after every successful mutation commit.
    #[must_use]
    pub fn projection(&self) -> &crate::projection::Projection {
        &self.projection
    }

    /// Rebuild the projection from `SQLite` (`REQ-PERS-002`).
    ///
    /// Discards the current graph state and re-hydrates from the store. `SQLite`
    /// wins on divergence. No projection-derived state survives a rebuild
    /// (`REQ-PERS-003`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Sqlite`] on store failure, or [`Error::CorruptStore`]
    /// when a row holds an unparseable id or unknown kind.
    pub fn rebuild_projection(&mut self) -> Result<(), Error> {
        let conn = self.store.connection();
        self.projection.rebuild(conn)
    }

    // --- project root -------------------------------------------------------

    /// Create the project root: a `Package` with `isModel = true`, no owner
    /// (`REQ-MM-006`). Exactly one per project; a second is rejected. The
    /// element row (with `isModel` already in its data blob) and the closure
    /// rows commit in a single transaction (`REQ-PERS-013`): no intermediate
    /// committed state can durably persist a root without `isModel = true`.
    ///
    /// # Errors
    ///
    /// - [`Error::RootAlreadyExists`] when a root already exists.
    /// - [`Error::Store`] / [`Error::Sqlite`] on store failure.
    pub fn create_root(
        &mut self,
        id: ElementId,
        name: Option<String>,
    ) -> Result<ElementView, Error> {
        if let Some(existing) = self.root_id()? {
            return Err(Error::RootAlreadyExists { existing });
        }
        let req = CreateElement {
            id,
            kind: MetaclassKind::Package,
            name,
            owner: None,
            visibility: Visibility::Public,
        };
        // The root is a Package with isModel = true (REQ-MM-006). Insert the
        // element row (data already carrying isModel) and the closure rows in
        // ONE transaction (REQ-PERS-013): no intermediate committed state can
        // durably persist a root without isModel = true.
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        insert_element_row(&tx, &req, r#"{"isModel":true}"#)?;
        closure::insert_on_create(&tx, &req.id.as_uuid().to_string(), None)?;
        tx.commit()?;
        // Write-through: the transaction committed, so update the projection.
        // If this had failed, neither the store nor the projection would have
        // changed (they update together or not at all — REQ-ARCH-007).
        let view = self.get_element(req.id)?;
        self.projection.add_element(&view);
        Ok(view)
    }

    /// The project root element, if one exists (`REQ-MM-006`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Sqlite`] on store failure, or [`Error::CorruptStore`]
    /// when a root row holds an unparseable id or unknown kind.
    pub fn root(&self) -> Result<Option<ElementView>, Error> {
        let conn = self.connection();
        let row = conn
            .query_row(
                "SELECT id, kind, owner_id, name, visibility \
                 FROM elements WHERE owner_id IS NULL LIMIT 1",
                [],
                read_element_row,
            )
            .optional()?;
        row.map(row_to_element_view).transpose()
    }

    fn root_id(&self) -> Result<Option<ElementId>, Error> {
        let conn = self.connection();
        let id: Option<String> = conn
            .query_row(
                "SELECT id FROM elements WHERE owner_id IS NULL LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        id.as_deref().map(parse_id).transpose()
    }

    // --- element CRUD -------------------------------------------------------

    /// Create an element (`REQ-MM-005`: adding to a namespace sets `owner`).
    ///
    /// The element row and closure rows commit in one transaction
    /// (`REQ-PERS-013`). The kind must be a top-level ownable metaclass
    /// (`REQ-MM-015`).
    ///
    /// # Errors
    ///
    /// - [`Error::NotTopLevelKind`] when the kind is a sub-element kind.
    /// - [`Error::ElementNotFound`] when the owner does not exist.
    /// - [`Error::RootAlreadyExists`] when creating a second root.
    /// - [`Error::Sqlite`] on store failure.
    pub fn create_element(&mut self, req: &CreateElement) -> Result<ElementView, Error> {
        if req.kind.scope() != MetaclassScope::TopLevel {
            return Err(Error::NotTopLevelKind {
                kind: req.kind.as_str().to_string(),
            });
        }
        if req.owner.is_none() && self.root_id()?.is_some() {
            return Err(Error::RootAlreadyExists {
                existing: self.root_id()?.unwrap_or(ElementId::new()),
            });
        }
        // Validate the owner exists (if given).
        if let Some(owner) = req.owner {
            if !self.element_exists(owner)? {
                return Err(Error::ElementNotFound { id: owner });
            }
        }
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        insert_element_row(&tx, req, "{}")?;
        closure::insert_on_create(
            &tx,
            &req.id.as_uuid().to_string(),
            req.owner.map(|o| o.as_uuid().to_string()).as_deref(),
        )?;
        tx.commit()?;
        // Write-through: the transaction committed, so update the projection.
        let view = self.get_element(req.id)?;
        self.projection.add_element(&view);
        Ok(view)
    }

    /// Get an element by id (`REQ-PERS-001`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::ElementNotFound`] when no element has this id,
    /// [`Error::Sqlite`] on store failure, or [`Error::CorruptStore`] when
    /// the row holds an unparseable id or unknown kind.
    pub fn get_element(&self, id: ElementId) -> Result<ElementView, Error> {
        let conn = self.connection();
        let row = conn
            .query_row(
                "SELECT id, kind, owner_id, name, visibility FROM elements WHERE id = ?1",
                params![id.as_uuid().to_string()],
                read_element_row,
            )
            .optional()?;
        row.map(row_to_element_view)
            .transpose()?
            .ok_or(Error::ElementNotFound { id })
    }

    /// Rename an element (`name` is mutable, not unique; `REQ-MM-004`).
    /// Renaming updates `qualifiedName` transparently (it is derived, not
    /// stored — `REQ-MM-003`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::ElementNotFound`] or [`Error::Sqlite`].
    pub fn rename_element(
        &mut self,
        id: ElementId,
        name: Option<&str>,
    ) -> Result<ElementView, Error> {
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        let changed = tx.execute(
            "UPDATE elements SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![name, NOW, id.as_uuid().to_string()],
        )?;
        if changed == 0 {
            return Err(Error::ElementNotFound { id });
        }
        tx.commit()?;
        // Write-through: update the projection node's name.
        self.projection.rename_element(id, name);
        self.get_element(id)
    }

    /// Reparent an element (`REQ-MM-005`: sets the new `owner`).
    ///
    /// Reparenting into the element's own subtree is rejected to keep the
    /// ownership tree acyclic (`INV-MM-001`). The closure table is updated
    /// in the same transaction (`REQ-PERS-008`).
    ///
    /// Pass `new_owner = None` to orphan to root (only valid if the element
    /// is not already the project root — the root has no owner).
    ///
    /// # Errors
    ///
    /// - [`Error::ElementNotFound`] when the element or new owner does not
    ///   exist.
    /// - [`Error::ReparentIntoOwnSubtree`] when the new owner is in the
    ///   element's subtree.
    /// - [`Error::Sqlite`] on store failure.
    pub fn reparent_element(
        &mut self,
        id: ElementId,
        new_owner: Option<ElementId>,
    ) -> Result<ElementView, Error> {
        let id_str = id.as_uuid().to_string();
        // Validate the element exists.
        if !self.element_exists(id)? {
            return Err(Error::ElementNotFound { id });
        }
        // Validate the new owner exists and is not in the element's subtree.
        if let Some(owner) = new_owner {
            if !self.element_exists(owner)? {
                return Err(Error::ElementNotFound { id: owner });
            }
            let owner_str = owner.as_uuid().to_string();
            let conn = self.connection();
            // If the new owner is a descendant of (or equal to) the element,
            // reparenting would form a cycle (INV-MM-001).
            if closure::depth(conn, &id_str, &owner_str)?.is_some() {
                return Err(Error::ReparentIntoOwnSubtree {
                    element: id,
                    new_owner: owner,
                });
            }
        }
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "UPDATE elements SET owner_id = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_owner.map(|o| o.as_uuid().to_string()), NOW, id_str,],
        )?;
        closure::update_on_reparent(
            &tx,
            &id_str,
            new_owner.map(|o| o.as_uuid().to_string()).as_deref(),
        )?;
        tx.commit()?;
        // Write-through: update the projection's ownership edges.
        self.projection.reparent_element(id, new_owner);
        self.get_element(id)
    }

    /// Delete an element (`REQ-MM-005`: removing from a namespace unsets
    /// `owner`).
    ///
    /// Deleting an element that owns children is rejected
    /// (store-level `ON DELETE RESTRICT` on `elements.owner_id`). Deleting
    /// an element referenced by a relationship is rejected (store-level
    /// `ON DELETE RESTRICT` on `relationships.source_id`/`target_id`/\
    /// `owner_id`); the pre-check covers all three so the surface stays the
    /// typed [`Error::ElementReferencedByRelationship`] rather than a raw FK
    /// violation. The closure table is cleaned by `ON DELETE CASCADE`.
    ///
    /// # Errors
    ///
    /// - [`Error::ElementNotFound`] when the element does not exist.
    /// - [`Error::ElementHasChildren`] when the element owns children.
    /// - [`Error::ElementReferencedByRelationship`] when a relationship
    ///   references it.
    /// - [`Error::Sqlite`] on store failure.
    pub fn delete_element(&mut self, id: ElementId) -> Result<(), Error> {
        let id_str = id.as_uuid().to_string();
        if !self.element_exists(id)? {
            return Err(Error::ElementNotFound { id });
        }
        let conn = self.connection();
        // Pre-check children (ON DELETE RESTRICT on owner_id).
        let child_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM elements WHERE owner_id = ?1",
            params![id_str],
            |row| row.get(0),
        )?;
        if child_count > 0 {
            return Err(Error::ElementHasChildren {
                element: id,
                count: usize::try_from(child_count).unwrap_or(0),
            });
        }
        // Pre-check relationship references (ON DELETE RESTRICT on
        // source_id, target_id, AND owner_id). Checking owner_id here keeps
        // the surface consistent: without it, deleting an element that owns
        // a relationship would pass both pre-checks and then fail inside the
        // transaction with a raw FK violation.
        let rel_ref: Option<String> = conn
            .query_row(
                "SELECT id FROM relationships \
                 WHERE source_id = ?1 OR target_id = ?1 OR owner_id = ?1 \
                 LIMIT 1",
                params![id_str],
                |row| row.get(0),
            )
            .optional()?;
        if let Some(rel_id) = rel_ref {
            return Err(Error::ElementReferencedByRelationship {
                element: id,
                relationship: parse_id(&rel_id)?,
            });
        }
        let tx = conn.unchecked_transaction()?;
        tx.execute("DELETE FROM elements WHERE id = ?1", params![id_str])?;
        tx.commit()?;
        // Write-through: remove the node and all its edges from the projection.
        self.projection.remove_element(id);
        Ok(())
    }

    // --- relationship CRUD --------------------------------------------------

    /// Create a relationship (`INV-MM-004`: endpoints must exist).
    ///
    /// # Errors
    ///
    /// - [`Error::RelationshipEndpointNotFound`] when source or target does
    ///   not exist.
    /// - [`Error::ElementNotFound`] when the owner does not exist.
    /// - [`Error::Sqlite`] on store failure.
    pub fn create_relationship(
        &mut self,
        req: &CreateRelationship,
    ) -> Result<RelationshipView, Error> {
        if !self.element_exists(req.source)? {
            return Err(Error::RelationshipEndpointNotFound {
                kind: req.kind.clone(),
                endpoint: req.source,
            });
        }
        if !self.element_exists(req.target)? {
            return Err(Error::RelationshipEndpointNotFound {
                kind: req.kind.clone(),
                endpoint: req.target,
            });
        }
        if !self.element_exists(req.owner)? {
            return Err(Error::ElementNotFound { id: req.owner });
        }
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO relationships (id, kind, source_id, target_id, owner_id, data, \
             created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, '{}', ?6, ?6)",
            params![
                req.id.as_uuid().to_string(),
                &req.kind,
                req.source.as_uuid().to_string(),
                req.target.as_uuid().to_string(),
                req.owner.as_uuid().to_string(),
                NOW,
            ],
        )?;
        tx.commit()?;
        // Write-through: add the relationship edge to the projection.
        let view = self.get_relationship(req.id)?;
        self.projection.add_relationship(&view);
        Ok(view)
    }

    /// Get a relationship by id.
    ///
    /// # Errors
    ///
    /// Returns [`Error::RelationshipNotFound`], [`Error::Sqlite`] on store
    /// failure, or [`Error::CorruptStore`] when a relationship row holds an
    /// unparseable id.
    pub fn get_relationship(&self, id: ElementId) -> Result<RelationshipView, Error> {
        let conn = self.connection();
        let row = conn
            .query_row(
                "SELECT id, kind, source_id, target_id, owner_id FROM relationships WHERE id = ?1",
                params![id.as_uuid().to_string()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                    ))
                },
            )
            .optional()?;
        let (id_str, kind, source, target, owner) =
            row.ok_or(Error::RelationshipNotFound { id })?;
        Ok(RelationshipView {
            id: parse_id(&id_str)?,
            kind,
            source: parse_id(&source)?,
            target: parse_id(&target)?,
            owner: parse_id(&owner)?,
        })
    }

    /// Delete a relationship.
    ///
    /// # Errors
    ///
    /// Returns [`Error::RelationshipNotFound`] or [`Error::Sqlite`].
    pub fn delete_relationship(&mut self, id: ElementId) -> Result<(), Error> {
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        let changed = tx.execute(
            "DELETE FROM relationships WHERE id = ?1",
            params![id.as_uuid().to_string()],
        )?;
        if changed == 0 {
            return Err(Error::RelationshipNotFound { id });
        }
        tx.commit()?;
        // Write-through: remove the relationship edge from the projection.
        self.projection.remove_relationship(id);
        Ok(())
    }

    // --- comments -----------------------------------------------------------

    /// Create a comment on an element (`REQ-MM-011`: non-empty body).
    ///
    /// Deleting the owning element deletes the comment (store-level
    /// `ON DELETE CASCADE` on `comments.owner_id`).
    ///
    /// # Errors
    ///
    /// - [`Error::EmptyCommentBody`] when the body is empty.
    /// - [`Error::ElementNotFound`] when the owner does not exist.
    /// - [`Error::Sqlite`] on store failure.
    pub fn create_comment(
        &mut self,
        id: ElementId,
        owner: ElementId,
        body: impl Into<String>,
    ) -> Result<CommentView, Error> {
        let comment = Comment::new(body).map_err(|e| comment_err(&e))?;
        if !self.element_exists(owner)? {
            return Err(Error::ElementNotFound { id: owner });
        }
        let conn = self.connection();
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO comments (id, owner_id, body, annotated) \
             VALUES (?1, ?2, ?3, '[]')",
            params![
                id.as_uuid().to_string(),
                owner.as_uuid().to_string(),
                comment.body,
            ],
        )?;
        tx.commit()?;
        self.get_comment(id)
    }

    /// Get a comment by id (`REQ-MM-011`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::ElementNotFound`], [`Error::Sqlite`] on store
    /// failure, or [`Error::CorruptStore`] when a comment row holds an
    /// unparseable id.
    pub fn get_comment(&self, id: ElementId) -> Result<CommentView, Error> {
        let conn = self.connection();
        let row = conn
            .query_row(
                "SELECT id, owner_id, body FROM comments WHERE id = ?1",
                params![id.as_uuid().to_string()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .optional()?;
        let (id_str, owner, body) = row.ok_or(Error::ElementNotFound { id })?;
        Ok(CommentView {
            id: parse_id(&id_str)?,
            owner: parse_id(&owner)?,
            body,
        })
    }

    // --- derived reads ------------------------------------------------------

    /// The `qualifiedName` of an element, derived on demand from the
    /// ownership chain (`REQ-MM-003`, `INV-MM-005`).
    ///
    /// Walks the ownership chain via the closure table's `ancestors` query,
    /// assembling the `/`-delimited path from root to the element. The path
    /// is assembled from owner names; the element's own name is the last
    /// segment. An element with no name contributes an empty segment.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ElementNotFound`] or [`Error::Sqlite`].
    pub fn qualified_name(&self, id: ElementId) -> Result<String, Error> {
        if !self.element_exists(id)? {
            return Err(Error::ElementNotFound { id });
        }
        let id_str = id.as_uuid().to_string();
        let conn = self.connection();
        // Ancestors nearest-first: [self, owner, ..., root].
        let chain = closure::ancestors(conn, &id_str)?;
        // Build names for each ancestor id, root-first.
        let mut names: Vec<String> = Vec::with_capacity(chain.len());
        for row in chain.iter().rev() {
            let name: Option<String> = conn.query_row(
                "SELECT name FROM elements WHERE id = ?1",
                params![row.id],
                |r| r.get(0),
            )?;
            names.push(name.unwrap_or_default());
        }
        Ok(names.join("/"))
    }

    // --- helpers ------------------------------------------------------------

    /// Whether an element with this id exists.
    fn element_exists(&self, id: ElementId) -> Result<bool, Error> {
        let conn = self.connection();
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM elements WHERE id = ?1)",
            params![id.as_uuid().to_string()],
            |row| row.get(0),
        )?;
        Ok(exists)
    }
}

// --- free functions ---------------------------------------------------------

/// Insert one element row (the `elements` table insert, no closure).
///
/// `data` is the JSON blob written to the `data` column (validated by the
/// store's `json_valid` CHECK). Callers pass `'{}'` for a plain element and
/// `{"isModel":true}` for the project root (`REQ-MM-006`).
fn insert_element_row(
    tx: &rusqlite::Transaction<'_>,
    req: &CreateElement,
    data: &str,
) -> Result<(), Error> {
    tx.execute(
        "INSERT INTO elements (id, kind, name, owner_id, visibility, data, created_at, \
         updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
        params![
            req.id.as_uuid().to_string(),
            req.kind.as_str(),
            req.name.as_deref(),
            req.owner.map(|o| o.as_uuid().to_string()),
            visibility_str(req.visibility),
            data,
            NOW,
        ],
    )?;
    Ok(())
}

/// Map a [`Visibility`] to its stable string id (the column value).
fn visibility_str(v: Visibility) -> &'static str {
    match v {
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::Protected => "protected",
        Visibility::Package => "package",
    }
}

/// Map a [`CommentError`] to an [`Error`].
fn comment_err(err: &CommentError) -> Error {
    match err {
        CommentError::EmptyBody => Error::EmptyCommentBody,
    }
}

/// Parse a UUID string from the store into an [`ElementId`].
///
/// Fails fast with [`Error::CorruptStore`] when the string is not a valid
/// UUID — a corrupt id in the store is a data-integrity failure that must be
/// surfaced, never silently substituted with a fresh random id
/// (`REQ-ARCH-017`/`REQ-ARCH-018`).
pub(crate) fn parse_id(s: &str) -> Result<ElementId, Error> {
    uuid::Uuid::parse_str(s)
        .map(ElementId::from)
        .map_err(|_| Error::CorruptStore {
            field: "id",
            detail: format!("not a valid UUID: {s:?}"),
        })
}

/// Resolve a metaclass kind string from the store into a [`MetaclassKind`].
///
/// Fails fast with [`Error::CorruptStore`] when the kind is unknown — an
/// unrecognized kind is corruption, not a defaultable `Package`.
pub(crate) fn kind_from_store(s: &str) -> Result<MetaclassKind, Error> {
    MetaclassKind::from_id(s).ok_or(Error::CorruptStore {
        field: "kind",
        detail: format!("unknown metaclass kind: {s:?}"),
    })
}

/// The raw string columns of an element row, as read from the store.
struct RawElementRow {
    id: String,
    kind: String,
    owner: Option<String>,
    name: Option<String>,
    visibility: String,
}

/// Read the five element columns from a row (the `SELECT` projection is
/// `id, kind, owner_id, name, visibility`).
fn read_element_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawElementRow> {
    Ok(RawElementRow {
        id: row.get(0)?,
        kind: row.get(1)?,
        owner: row.get(2)?,
        name: row.get(3)?,
        visibility: row.get(4)?,
    })
}

/// Build an [`ElementView`] from the raw string columns, failing fast on a
/// corrupt id or unknown kind instead of fabricating values.
fn row_to_element_view(raw: RawElementRow) -> Result<ElementView, Error> {
    let kind = kind_from_store(&raw.kind)?;
    let visibility = match raw.visibility.as_str() {
        "private" => Visibility::Private,
        "protected" => Visibility::Protected,
        "package" => Visibility::Package,
        _ => Visibility::Public,
    };
    Ok(ElementView {
        id: parse_id(&raw.id)?,
        kind,
        owner: raw.owner.as_deref().map(parse_id).transpose()?,
        name: raw.name,
        visibility,
    })
}
