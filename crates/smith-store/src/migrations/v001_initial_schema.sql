-- V001 — initial Smith schema (`docs/architecture/03-persistence.md` §SQLite
-- schema, REQ-PERS-004). Types/constraints finalized by the implementer:
-- timestamps are RFC 3339 TEXT, JSON TEXT columns carry a json_valid CHECK
-- plus a validation trigger (never trusted raw, REQ-PERS-005), and the
-- shape_views element index doubles as the coverage index (REQ-PERS-006).

-- Elements (nodes) — single table, kind-discriminated.
CREATE TABLE elements (
  id          TEXT PRIMARY KEY,            -- ElementId (UUID v7 string)
  kind        TEXT NOT NULL,               -- metaclass: 'Class', 'Package', 'Requirement', ...
  name        TEXT,                        -- nullable (NamedElement.name)
  owner_id    TEXT,                        -- adjacency-list parent (Namespace); NULL for root
  visibility  TEXT NOT NULL DEFAULT 'public'
              CHECK (visibility IN ('public', 'private', 'protected', 'package')),
  data        TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(data)),
  created_at  TEXT NOT NULL,
  updated_at  TEXT NOT NULL,
  FOREIGN KEY (owner_id) REFERENCES elements(id) ON DELETE RESTRICT
);
CREATE INDEX idx_elements_owner ON elements(owner_id);
CREATE INDEX idx_elements_kind  ON elements(kind);

-- Closure table for the ownership tree (Package/Namespace owns members).
CREATE TABLE ownership_closure (
  ancestor_id   TEXT NOT NULL,
  descendant_id TEXT NOT NULL,
  depth         INTEGER NOT NULL CHECK (depth >= 0),  -- 0 = self, 1 = direct child, ...
  PRIMARY KEY (ancestor_id, descendant_id),
  FOREIGN KEY (ancestor_id)   REFERENCES elements(id) ON DELETE CASCADE,
  FOREIGN KEY (descendant_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_closure_anc ON ownership_closure(ancestor_id, depth);
CREATE INDEX idx_closure_dec ON ownership_closure(descendant_id);

-- Relationships (edges).
CREATE TABLE relationships (
  id          TEXT PRIMARY KEY,            -- RelationshipId
  kind        TEXT NOT NULL,               -- 'Association', 'Generalization', 'Dependency', ...
  stereotype  TEXT,                        -- for trace relations: 'satisfy', 'verify', ...
  source_id   TEXT NOT NULL,
  target_id   TEXT NOT NULL,
  owner_id    TEXT NOT NULL,               -- owning namespace
  data        TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(data)),
  created_at  TEXT NOT NULL,
  updated_at  TEXT NOT NULL,
  FOREIGN KEY (source_id) REFERENCES elements(id) ON DELETE RESTRICT,
  FOREIGN KEY (target_id) REFERENCES elements(id) ON DELETE RESTRICT,
  FOREIGN KEY (owner_id)  REFERENCES elements(id) ON DELETE RESTRICT
);
CREATE INDEX idx_rel_source ON relationships(source_id);
CREATE INDEX idx_rel_target ON relationships(target_id);
CREATE INDEX idx_rel_kind   ON relationships(kind);

-- Stereotype applications.
CREATE TABLE applied_stereotypes (
  id            TEXT PRIMARY KEY,
  element_id    TEXT NOT NULL,
  stereotype_id TEXT NOT NULL,             -- references the Stereotype element
  tag_values    TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(tag_values)),
  FOREIGN KEY (element_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_stereo_element ON applied_stereotypes(element_id);

-- Comments (owned by an element).
CREATE TABLE comments (
  id           TEXT PRIMARY KEY,
  owner_id     TEXT NOT NULL,              -- the element owning this comment
  body         TEXT NOT NULL,
  annotated    TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(annotated)),
  FOREIGN KEY (owner_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_comments_owner ON comments(owner_id);

-- Diagrams (views) and view references (DI shapes/edges).
CREATE TABLE diagrams (
  id           TEXT PRIMARY KEY,
  name         TEXT NOT NULL,
  kind         TEXT NOT NULL,              -- DiagramKind
  owner_id     TEXT NOT NULL,              -- owning package
  canvas_state TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(canvas_state)),
  FOREIGN KEY (owner_id) REFERENCES elements(id) ON DELETE CASCADE
);

CREATE TABLE shape_views (
  id               TEXT PRIMARY KEY,       -- ViewRefId (per-diagram)
  diagram_id       TEXT NOT NULL,
  model_element_id TEXT NOT NULL,
  bounds           TEXT NOT NULL CHECK (json_valid(bounds)),
  hints            TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(hints)),
  container_shape  TEXT,                   -- parent shape (nested)
  FOREIGN KEY (diagram_id)       REFERENCES diagrams(id) ON DELETE CASCADE,
  FOREIGN KEY (model_element_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_shape_diagram ON shape_views(diagram_id);
-- Coverage index (REQ-PERS-006 / REQ-DI-015): covered set = SELECT
-- model_element_id FROM shape_views; uncovered = set difference vs elements.
CREATE INDEX idx_shape_element ON shape_views(model_element_id);

CREATE TABLE edge_views (
  id                TEXT PRIMARY KEY,
  diagram_id        TEXT NOT NULL,
  model_relation_id TEXT NOT NULL,
  source_shape_id   TEXT NOT NULL,
  target_shape_id   TEXT NOT NULL,
  bend_points       TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(bend_points)),
  label_positions   TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(label_positions)),
  routing_kind      TEXT NOT NULL DEFAULT 'orthogonal'
                    CHECK (routing_kind IN
                           ('orthogonal', 'straight', 'curved', 'fixedBendPoints')),
  FOREIGN KEY (diagram_id)          REFERENCES diagrams(id)      ON DELETE CASCADE,
  FOREIGN KEY (model_relation_id)   REFERENCES relationships(id) ON DELETE CASCADE,
  FOREIGN KEY (source_shape_id)     REFERENCES shape_views(id)   ON DELETE CASCADE,
  FOREIGN KEY (target_shape_id)     REFERENCES shape_views(id)   ON DELETE CASCADE
);
CREATE INDEX idx_edge_diagram ON edge_views(diagram_id);

-- FTS5 full-text search index (population is a smith-model concern).
CREATE VIRTUAL TABLE elements_fts USING fts5(
  element_id UNINDEXED,
  name, qualified_name, kind, requirement_id, test_case_id, comment_body,
  tokenize = 'unicode61'
);

-- UI state (per-project panel layout, expanded nodes, etc.).
CREATE TABLE ui_state (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- JSON blobs are validated on write, never trusted raw (REQ-PERS-005): the
-- json_valid CHECK constraints above apply to bound parameters and literals
-- alike (verified by store_insert_rejects_invalid_json_blobs), and serde
-- validates the shape on both write and read (smith-store::data).
