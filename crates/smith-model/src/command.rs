//! The undo/redo command stack (`REQ-PERS-014`…`016`).
//!
//! Each command records `do` and `undo` deltas. Undo replays the inverse delta
//! through the same model API, re-validating invariants; an undo that would
//! violate an invariant is rejected with a clear error (never a panic —
//! `REQ-ARCH-018`). Redo replays the forward delta.
//!
//! The stack is bounded (default depth 200; oldest evicted), project-wide, and
//! shared by UI and MCP callers. Each command is tagged with an [`Actor`]
//! (`ui` or `session:<id>`). A [`BatchCommand`] wraps N sub-commands so they
//! undo/redo as one unit.
//!
//! Undo is NOT SQLite-level: every command (including undo itself) commits its
//! own transaction through `smith-store` (`REQ-PERS-013`). Undo is a forward
//! mutation, persisted the same way as any other.

use smith_core::ElementId;

use crate::error::Error;
use crate::model::{CreateElement, CreateRelationship, ElementView, Model, RelationshipView};

/// The default bounded depth of the undo/redo stack (`REQ-PERS-015`).
pub const DEFAULT_DEPTH: usize = 200;

/// Who initiated a command (`REQ-PERS-015`).
///
/// `Ui` is a keyboard/menu action; `Session("<id>")` is an MCP caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Actor {
    /// A UI action (Cmd/Ctrl+Z).
    Ui,
    /// An MCP session, identified by its session id.
    Session(String),
}

impl Actor {
    /// The stable string form (`"ui"` or `"session:<id>"`).
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Ui => "ui".to_string(),
            Self::Session(id) => format!("session:{id}"),
        }
    }
}

impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}

/// A single undoable/redoable command (`REQ-PERS-014`).
///
/// Records `do` and `undo` deltas. Both are replayed through the [`Model`] API
/// so invariants are re-validated on undo as well as do (`REQ-PERS-016`).
pub trait Command: Send {
    /// Apply the forward delta (re-played on redo).
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the model API rejects the operation (invariant
    /// violation, store failure). Never panics (`REQ-ARCH-018`).
    fn redo(&mut self, model: &Model) -> Result<(), Error>;

    /// Apply the inverse delta (re-played on undo).
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when replaying the inverse delta would violate an
    /// invariant (`REQ-PERS-016`). The command stays on the undo stack when
    /// this returns `Err`; the model is unchanged.
    fn undo(&mut self, model: &Model) -> Result<(), Error>;

    /// A short, human-readable label (for UI/menus and MCP descriptions).
    fn label(&self) -> &'static str;

    /// The actor who initiated this command (`REQ-PERS-015`).
    fn actor(&self) -> Actor;
}

// --- element commands -------------------------------------------------------

/// Create an element; undo deletes it.
pub struct CreateElementCommand {
    req: CreateElement,
    actor: Actor,
}

impl CreateElementCommand {
    /// Create a command tagged `Actor::Ui`.
    #[must_use]
    pub fn new(req: CreateElement) -> Self {
        Self {
            req,
            actor: Actor::Ui,
        }
    }

    /// Create a command with an explicit actor tag.
    #[must_use]
    pub fn with_actor(req: CreateElement, actor: Actor) -> Self {
        Self { req, actor }
    }
}

impl Command for CreateElementCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        let view: ElementView = if self.req.owner.is_none() {
            model.create_root(self.req.id, self.req.name.clone())?
        } else {
            model.create_element(&self.req)?
        };
        self.req.name = view.name;
        Ok(())
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        model.delete_element(self.req.id)
    }

    fn label(&self) -> &'static str {
        "create element"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

/// Delete an element; undo re-creates it (same id, kind, owner, name,
/// visibility).
pub struct DeleteElementCommand {
    id: ElementId,
    /// The pre-deletion snapshot, captured on the first `redo` so the undo
    /// delta can re-create the element with its exact prior fields.
    snapshot: Option<ElementView>,
    actor: Actor,
}

impl DeleteElementCommand {
    /// Create a command tagged `Actor::Ui`.
    #[must_use]
    pub fn new(id: ElementId) -> Self {
        Self {
            id,
            snapshot: None,
            actor: Actor::Ui,
        }
    }

    /// Create a command with an explicit actor tag.
    #[must_use]
    pub fn with_actor(id: ElementId, actor: Actor) -> Self {
        Self {
            id,
            snapshot: None,
            actor,
        }
    }
}

impl Command for DeleteElementCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        // Capture the element's state before deleting, so the undo delta can
        // re-create it with the same fields. This is the `undo` delta.
        self.snapshot = Some(model.get_element(self.id)?);
        model.delete_element(self.id)
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        let view = self.snapshot.clone().ok_or(Error::CannotUndoRedoBeforeDo {
            reason: "delete command has no snapshot; redo was never run",
        })?;
        let req = CreateElement {
            id: view.id,
            kind: view.kind,
            name: view.name,
            owner: view.owner,
            visibility: view.visibility,
        };
        model.create_element(&req)?;
        Ok(())
    }

    fn label(&self) -> &'static str {
        "delete element"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

/// Rename an element; undo restores the prior name.
pub struct RenameElementCommand {
    id: ElementId,
    new_name: Option<String>,
    /// The pre-rename name, captured on the first `redo` so the undo delta can
    /// restore it.
    prior_name: Option<String>,
    actor: Actor,
}

impl RenameElementCommand {
    /// Create a command tagged `Actor::Ui`.
    #[must_use]
    pub fn new(id: ElementId, new_name: Option<String>) -> Self {
        Self {
            id,
            new_name,
            prior_name: None,
            actor: Actor::Ui,
        }
    }

    /// Create a command with an explicit actor tag.
    #[must_use]
    pub fn with_actor(id: ElementId, new_name: Option<String>, actor: Actor) -> Self {
        Self {
            id,
            new_name,
            prior_name: None,
            actor,
        }
    }
}

impl Command for RenameElementCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        self.prior_name = model.get_element(self.id)?.name;
        model.rename_element(self.id, self.new_name.as_deref())?;
        Ok(())
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        let prior = self
            .prior_name
            .clone()
            .ok_or(Error::CannotUndoRedoBeforeDo {
                reason: "rename command has no prior name; redo was never run",
            })?;
        model.rename_element(self.id, Some(prior.as_str()))?;
        Ok(())
    }

    fn label(&self) -> &'static str {
        "rename element"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

/// Reparent an element; undo restores the prior owner.
pub struct ReparentElementCommand {
    id: ElementId,
    new_owner: Option<ElementId>,
    /// The pre-reparent owner, captured on the first `redo`.
    prior_owner: Option<ElementId>,
    actor: Actor,
}

impl ReparentElementCommand {
    /// Create a command tagged `Actor::Ui`.
    #[must_use]
    pub fn new(id: ElementId, new_owner: Option<ElementId>) -> Self {
        Self {
            id,
            new_owner,
            prior_owner: None,
            actor: Actor::Ui,
        }
    }

    /// Create a command with an explicit actor tag.
    #[must_use]
    pub fn with_actor(id: ElementId, new_owner: Option<ElementId>, actor: Actor) -> Self {
        Self {
            id,
            new_owner,
            prior_owner: None,
            actor,
        }
    }
}

impl Command for ReparentElementCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        self.prior_owner = model.get_element(self.id)?.owner;
        model.reparent_element(self.id, self.new_owner)?;
        Ok(())
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        let prior = self.prior_owner;
        model.reparent_element(self.id, prior)?;
        Ok(())
    }

    fn label(&self) -> &'static str {
        "reparent element"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

// --- relationship commands --------------------------------------------------

/// Create a relationship; undo deletes it.
pub struct CreateRelationshipCommand {
    req: CreateRelationship,
    actor: Actor,
}

impl CreateRelationshipCommand {
    /// Create a command tagged `Actor::Ui`.
    #[must_use]
    pub fn new(req: CreateRelationship) -> Self {
        Self {
            req,
            actor: Actor::Ui,
        }
    }

    /// Create a command with an explicit actor tag.
    #[must_use]
    pub fn with_actor(req: CreateRelationship, actor: Actor) -> Self {
        Self { req, actor }
    }
}

impl Command for CreateRelationshipCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        let view: RelationshipView = model.create_relationship(&self.req)?;
        // Normalize the kind to the stored form in case the caller passed a
        // differently-cased string.
        self.req.kind = view.kind;
        Ok(())
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        model.delete_relationship(self.req.id)
    }

    fn label(&self) -> &'static str {
        "create relationship"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

/// Delete a relationship; undo re-creates it (same id, kind, endpoints, owner).
pub struct DeleteRelationshipCommand {
    id: ElementId,
    /// The pre-deletion snapshot, captured on the first `redo`.
    snapshot: Option<RelationshipView>,
    actor: Actor,
}

impl DeleteRelationshipCommand {
    /// Create a command tagged `Actor::Ui`.
    #[must_use]
    pub fn new(id: ElementId) -> Self {
        Self {
            id,
            snapshot: None,
            actor: Actor::Ui,
        }
    }

    /// Create a command with an explicit actor tag.
    #[must_use]
    pub fn with_actor(id: ElementId, actor: Actor) -> Self {
        Self {
            id,
            snapshot: None,
            actor,
        }
    }
}

impl Command for DeleteRelationshipCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        self.snapshot = Some(model.get_relationship(self.id)?);
        model.delete_relationship(self.id)
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        let view = self.snapshot.clone().ok_or(Error::CannotUndoRedoBeforeDo {
            reason: "delete-relationship command has no snapshot; redo was never run",
        })?;
        let req = CreateRelationship {
            id: view.id,
            kind: view.kind,
            source: view.source,
            target: view.target,
            owner: view.owner,
        };
        model.create_relationship(&req)?;
        Ok(())
    }

    fn label(&self) -> &'static str {
        "delete relationship"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

// --- batch ------------------------------------------------------------------

/// A batch wraps N sub-commands so they undo/redo as one unit
/// (`REQ-PERS-015`).
///
/// `redo` applies all sub-commands in order; `undo` reverses them in reverse
/// order. If any sub-command's `redo` fails mid-batch, the already-applied
/// sub-commands are rolled back (undone) in reverse order before returning the
/// error — the batch is atomic. The same applies to `undo`.
pub struct BatchCommand {
    sub: Vec<Box<dyn Command>>,
    actor: Actor,
}

impl BatchCommand {
    /// Create a batch from sub-commands, all tagged with `actor`.
    #[must_use]
    pub fn new(sub: Vec<Box<dyn Command>>, actor: Actor) -> Self {
        Self { sub, actor }
    }
}

impl Command for BatchCommand {
    fn redo(&mut self, model: &Model) -> Result<(), Error> {
        // Apply each sub-command in order. If one fails, roll back the
        // already-applied ones (in reverse) so the batch is atomic.
        for i in 0..self.sub.len() {
            if let Err(err) = self.sub[i].redo(model) {
                // Roll back [0, i) in reverse order.
                for j in (0..i).rev() {
                    // Rollback failure is a data-integrity problem; surface it
                    // by replacing the original error so the caller sees the
                    // real cause of inconsistency.
                    if self.sub[j].undo(model).is_err() {
                        return Err(Error::BatchRollbackFailed {
                            reason: format!(
                                "batch redo failed at index {i} ({err}) and rollback of index \
                                 {j} also failed; model may be inconsistent"
                            ),
                        });
                    }
                }
                return Err(err);
            }
        }
        Ok(())
    }

    fn undo(&mut self, model: &Model) -> Result<(), Error> {
        // Undo each sub-command in reverse order. If one fails, the batch is
        // left partially undone — surface the error. (We do NOT roll forward
        // the remaining undone sub-commands, because that would re-apply
        // deltas the caller asked to undo.)
        for cmd in self.sub.iter_mut().rev() {
            cmd.undo(model)?;
        }
        Ok(())
    }

    fn label(&self) -> &'static str {
        "batch"
    }

    fn actor(&self) -> Actor {
        self.actor.clone()
    }
}

// --- the stack --------------------------------------------------------------

/// The bounded undo/redo command stack (`REQ-PERS-015`).
///
/// Project-wide (not per-diagram), shared by UI and MCP callers. Bounded to
/// [`DEFAULT_DEPTH`] (200) entries; the oldest is evicted when the stack is
/// full. Each entry is tagged with an [`Actor`].
pub struct CommandStack {
    undo: Vec<Box<dyn Command>>,
    redo: Vec<Box<dyn Command>>,
    depth: usize,
}

impl std::fmt::Debug for CommandStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandStack")
            .field("undo_len", &self.undo.len())
            .field("redo_len", &self.redo.len())
            .field("depth", &self.depth)
            .finish()
    }
}

impl CommandStack {
    /// Create a stack with the default depth (200).
    #[must_use]
    pub fn new() -> Self {
        Self::with_depth(DEFAULT_DEPTH)
    }

    /// Create a stack with a custom depth (primarily for tests).
    #[must_use]
    pub fn with_depth(depth: usize) -> Self {
        Self {
            undo: Vec::with_capacity(depth),
            redo: Vec::with_capacity(depth),
            depth,
        }
    }

    /// Number of commands on the undo stack (the bounded depth in use).
    #[must_use]
    pub fn len(&self) -> usize {
        self.undo.len()
    }

    /// Whether the undo stack is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.undo.is_empty()
    }

    /// Whether an undo is available.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    /// Whether a redo is available.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// The actor tags on the undo stack, in push order (oldest first).
    #[must_use]
    pub fn undo_actors(&self) -> Vec<Actor> {
        self.undo.iter().map(|c| c.actor()).collect()
    }

    /// The actor tags on the redo stack, in LIFO order (next redo first).
    #[must_use]
    pub fn redo_actors(&self) -> Vec<Actor> {
        self.redo.iter().map(|c| c.actor()).collect()
    }

    /// Execute a command (the `do` delta), pushing it onto the undo stack.
    ///
    /// Clears the redo stack: a new command after undos discards the redo
    /// branch (standard undo/redo semantics). If the stack is full, the oldest
    /// command is evicted (`REQ-PERS-015`).
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the command's `redo` fails (invariant violation,
    /// store failure). The command is NOT pushed onto the stack on failure.
    pub fn execute(&mut self, model: &Model, mut cmd: Box<dyn Command>) -> Result<(), Error> {
        cmd.redo(model)?;
        // A new command after undos discards the redo branch.
        self.redo.clear();
        // Evict the oldest if at capacity.
        if self.undo.len() >= self.depth {
            self.undo.remove(0);
        }
        self.undo.push(cmd);
        Ok(())
    }

    /// Undo the most recent command (replay the inverse delta).
    ///
    /// The command moves to the redo stack. If the inverse delta would violate
    /// an invariant, the command stays on the undo stack and the error is
    /// returned (`REQ-PERS-016`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::UndoStackEmpty`] when the undo stack is empty, or the
    /// command's [`Error`] when replaying the inverse delta fails.
    pub fn undo(&mut self, model: &Model) -> Result<(), Error> {
        let mut cmd = self.undo.pop().ok_or(Error::UndoStackEmpty)?;
        match cmd.undo(model) {
            Ok(()) => {
                self.redo.push(cmd);
                Ok(())
            }
            Err(err) => {
                // Undo rejected: the command stays on the undo stack.
                self.undo.push(cmd);
                Err(err)
            }
        }
    }

    /// Redo the most recently undone command (replay the forward delta).
    ///
    /// The command moves back to the undo stack. If the forward delta would
    /// violate an invariant, the command stays on the redo stack and the error
    /// is returned.
    ///
    /// # Errors
    ///
    /// Returns [`Error::RedoStackEmpty`] when the redo stack is empty, or the
    /// command's [`Error`] when replaying the forward delta fails.
    pub fn redo(&mut self, model: &Model) -> Result<(), Error> {
        let mut cmd = self.redo.pop().ok_or(Error::RedoStackEmpty)?;
        match cmd.redo(model) {
            Ok(()) => {
                if self.undo.len() >= self.depth {
                    self.undo.remove(0);
                }
                self.undo.push(cmd);
                Ok(())
            }
            Err(err) => {
                self.redo.push(cmd);
                Err(err)
            }
        }
    }
}

impl Default for CommandStack {
    fn default() -> Self {
        Self::new()
    }
}
