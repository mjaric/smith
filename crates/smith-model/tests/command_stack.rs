//! `smith-model` acceptance tests for issue #7 — the undo/redo command stack.
//!
//! One test per acceptance criterion, named per the issue. Style: deterministic
//! tests return [`TestResult`] and use the `ensure` macros — repo lints deny
//! `expect`/`unwrap`/`panic`.
//!
//! Covers `REQ-PERS-014` (command pattern, do/undo deltas), `REQ-PERS-015`
//! (bounded depth, actor tags, batch), `REQ-PERS-016` (replays through the
//! model API, re-validates invariants), `REQ-ARCH-018` (never panics).

use smith_core::{ElementId, MetaclassKind, Visibility};
use smith_model::command::{
    Actor, BatchCommand, CommandStack, CreateElementCommand, DeleteElementCommand,
    RenameElementCommand, ReparentElementCommand,
};
use smith_model::{CreateElement, Error, Model};

/// Deterministic test outcome: helper failures propagate via `?` (no panics).
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Assertion that fails the test by returning `Err` (no panics).
macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            return Err(format!("assertion failed: `{}`", stringify!($cond)).into());
        }
    };
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(format!($($arg)*).into());
        }
    };
}

/// Equality assertion that fails the test by returning `Err` (no panics).
macro_rules! ensure_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            return Err(format!(
                "assertion failed: `{}` == `{}`\n  left: {:?}\n right: {:?}",
                stringify!($left),
                stringify!($right),
                $left,
                $right,
            )
            .into());
        }
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if $left != $right {
            return Err(format!(
                "assertion failed: `{}` == `{}`: {}\n  left: {:?}\n right: {:?}",
                stringify!($left),
                stringify!($right),
                format!($($arg)*),
                $left,
                $right,
            )
            .into());
        }
    };
}

// --- shared helpers ---------------------------------------------------------

/// Open a fresh temp-file model for a test.
fn fresh_model() -> Result<Model, Error> {
    let dir = tempfile::tempdir().map_err(|e| Error::Sqlite {
        detail: rusqlite::Error::ToSqlConversionFailure(Box::new(e)),
    })?;
    Model::open(&dir.path().join("test.smith"))
}

/// Mint a fresh element id.
fn new_id() -> ElementId {
    ElementId::new()
}

/// Create the project root, returning its id.
fn create_root(model: &mut Model, name: &str) -> Result<ElementId, Error> {
    let id = new_id();
    model.create_root(id, Some(name.to_string()))?;
    Ok(id)
}

/// Build a `CreateElement` request for a class under `owner`.
fn class_req(id: ElementId, owner: ElementId, name: &str) -> CreateElement {
    CreateElement {
        id,
        kind: MetaclassKind::Class,
        name: Some(name.to_string()),
        owner: Some(owner),
        visibility: Visibility::Public,
    }
}

/// Build a `CreateElement` request for a package under `owner`.
fn pkg_req(id: ElementId, owner: ElementId, name: &str) -> CreateElement {
    CreateElement {
        id,
        kind: MetaclassKind::Package,
        name: Some(name.to_string()),
        owner: Some(owner),
        visibility: Visibility::Public,
    }
}

// ===========================================================================
// REQ-PERS-014: command_records_do_and_undo_deltas_and_undo_reverses
// (create, rename, reparent, delete each reversed exactly)
// ===========================================================================

#[test]
fn command_records_do_and_undo_deltas_and_undo_reverses() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let mut stack = CommandStack::new();

    // --- create: undo reverses (element is deleted) ---
    let a = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::new(class_req(a, root, "a"))),
    )?;
    let view = model.get_element(a)?;
    ensure_eq!(view.name.as_deref(), Some("a"));
    ensure_eq!(view.owner, Some(root));

    // Undo create → A is gone.
    stack.undo(&mut model)?;
    ensure!(
        model.get_element(a).is_err(),
        "undo of create must delete the element"
    );

    // Redo create → A is back with the same fields.
    stack.redo(&mut model)?;
    let view = model.get_element(a)?;
    ensure_eq!(view.name.as_deref(), Some("a"));
    ensure_eq!(view.owner, Some(root));

    // --- rename: undo reverses (name is restored) ---
    stack.execute(
        &mut model,
        Box::new(RenameElementCommand::new(a, Some("renamed".to_string()))),
    )?;
    ensure_eq!(model.get_element(a)?.name.as_deref(), Some("renamed"));

    // Undo rename → name is "a" again.
    stack.undo(&mut model)?;
    ensure_eq!(model.get_element(a)?.name.as_deref(), Some("a"));

    // Redo rename → name is "renamed" again.
    stack.redo(&mut model)?;
    ensure_eq!(model.get_element(a)?.name.as_deref(), Some("renamed"));

    // --- reparent: undo reverses (owner is restored) ---
    let b = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::new(pkg_req(b, root, "b"))),
    )?;
    stack.execute(&mut model, Box::new(ReparentElementCommand::new(a, Some(b))))?;
    ensure_eq!(model.get_element(a)?.owner, Some(b));

    // Undo reparent → A is back under root.
    stack.undo(&mut model)?;
    ensure_eq!(model.get_element(a)?.owner, Some(root));

    // Redo reparent → A is under B again.
    stack.redo(&mut model)?;
    ensure_eq!(model.get_element(a)?.owner, Some(b));

    // --- delete: undo reverses (element is re-created) ---
    // Reparent A back to root so it can be deleted cleanly (B is its sibling).
    stack.execute(&mut model, Box::new(ReparentElementCommand::new(a, Some(root))))?;
    // Delete B (no children, no relationships) to keep the tree clean.
    stack.execute(&mut model, Box::new(DeleteElementCommand::new(b)))?;

    // Delete A.
    stack.execute(&mut model, Box::new(DeleteElementCommand::new(a)))?;
    ensure!(
        model.get_element(a).is_err(),
        "delete must remove the element"
    );

    // Undo delete → A is re-created with the fields it had before deletion.
    stack.undo(&mut model)?;
    let view = model.get_element(a)?;
    ensure_eq!(view.name.as_deref(), Some("renamed"));
    ensure_eq!(view.kind, MetaclassKind::Class);
    ensure_eq!(view.owner, Some(root));
    ensure_eq!(view.visibility, Visibility::Public);

    Ok(())
}

// ===========================================================================
// REQ-PERS-015 (depth): stack_evicts_oldest_beyond_depth_200
// ===========================================================================

#[test]
fn stack_evicts_oldest_beyond_depth_200() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let mut stack = CommandStack::new();

    // Push the first command (will be evicted).
    let first = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::new(class_req(first, root, "first"))),
    )?;

    // Push 200 more (total 201).
    for _ in 0..200 {
        let id = new_id();
        stack.execute(
            &mut model,
            Box::new(CreateElementCommand::new(class_req(id, root, "x"))),
        )?;
    }

    // Stack depth is 200 (the first command was evicted).
    ensure_eq!(stack.len(), 200, "stack must cap at depth 200");

    // Undo all 200 remaining commands.
    for _ in 0..200 {
        stack.undo(&mut model)?;
    }
    ensure!(
        !stack.can_undo(),
        "undo stack must be empty after 200 undos"
    );

    // The first element still exists — its undo was evicted, so it cannot be
    // undone. This is the eviction contract: oldest beyond depth 200 is gone.
    ensure!(
        model.get_element(first).is_ok(),
        "evicted command's undo is gone; the first element must still exist"
    );

    Ok(())
}

// ===========================================================================
// REQ-PERS-015 (actor): commands_are_tagged_with_actor
// ===========================================================================

#[test]
fn commands_are_tagged_with_actor() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let mut stack = CommandStack::new();

    // A UI-issued command.
    let id_ui = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::with_actor(
            class_req(id_ui, root, "ui-created"),
            Actor::Ui,
        )),
    )?;

    // A session-issued command.
    let id_sess = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::with_actor(
            class_req(id_sess, root, "session-created"),
            Actor::Session("s1".to_string()),
        )),
    )?;

    // The undo stack preserves actor tags in push order.
    let actors = stack.undo_actors();
    ensure_eq!(actors.len(), 2);
    ensure_eq!(actors[0], Actor::Ui);
    ensure_eq!(actors[1], Actor::Session("s1".to_string()));

    // After undoing one, the redo stack still carries its actor tag.
    stack.undo(&mut model)?;
    let redo_actors = stack.redo_actors();
    ensure_eq!(redo_actors.len(), 1);
    ensure_eq!(redo_actors[0], Actor::Session("s1".to_string()));

    Ok(())
}

// ===========================================================================
// REQ-PERS-015 (batch): batch_command_undoes_as_one_unit
// ===========================================================================

#[test]
fn batch_command_undoes_as_one_unit() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let mut stack = CommandStack::new();

    // Create three elements as one batch.
    let ids: Vec<ElementId> = (0..3).map(|_| new_id()).collect();
    let sub_commands: Vec<Box<dyn smith_model::command::Command>> = ids
        .iter()
        .map(|&id| {
            Box::new(CreateElementCommand::new(class_req(id, root, "batched")))
                as Box<dyn smith_model::command::Command>
        })
        .collect();

    stack.execute(&mut model, Box::new(BatchCommand::new(sub_commands, Actor::Ui)))?;

    // All three exist.
    for &id in &ids {
        ensure!(model.get_element(id).is_ok(), "batched element must exist");
    }

    // The batch is a single entry on the undo stack.
    ensure_eq!(stack.len(), 1, "batch must be one undo unit");

    // One undo reverses all three.
    stack.undo(&mut model)?;
    ensure!(!stack.can_undo(), "undo stack empty after batch undo");

    for &id in &ids {
        ensure!(
            model.get_element(id).is_err(),
            "all batched elements must be gone after one undo"
        );
    }

    // Redo re-creates all three.
    stack.redo(&mut model)?;
    for &id in &ids {
        ensure!(
            model.get_element(id).is_ok(),
            "all batched elements re-created after redo"
        );
    }

    Ok(())
}

// ===========================================================================
// REQ-PERS-016: undo_revalidates_invariants_and_rejects_violation_with_error
// ===========================================================================

#[test]
fn undo_revalidates_invariants_and_rejects_violation_with_error() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let mut stack = CommandStack::new();

    // Create A under root via the command stack (undo delta: delete A).
    let a = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::new(class_req(a, root, "a"))),
    )?;

    // Create B under A directly via the model API (bypassing the stack).
    // This changes the model underneath the stack.
    let b = new_id();
    model.create_element(&class_req(b, a, "b"))?;

    // Undo the create-A command (which would delete A).
    // A now has a child (B), so the undo must be rejected with
    // ElementHasChildren — invariants are re-validated through the model API.
    let err = stack.undo(&mut model).err().ok_or("undo should have failed")?;
    ensure!(
        matches!(err, Error::ElementHasChildren { element, count } if element == a && count == 1),
        "undo should reject with ElementHasChildren, got {err:?}"
    );

    // The command is still on the undo stack (undo was rejected, not applied).
    ensure_eq!(
        stack.len(),
        1,
        "command stays on undo stack after rejection"
    );

    // A still exists (undo was rejected, model unchanged).
    ensure!(
        model.get_element(a).is_ok(),
        "element must still exist after rejected undo"
    );

    Ok(())
}

// ===========================================================================
// REQ-PERS-016: redo_replays_forward_delta
// ===========================================================================

#[test]
fn redo_replays_forward_delta() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let mut stack = CommandStack::new();

    // Create A under root.
    let a = new_id();
    stack.execute(
        &mut model,
        Box::new(CreateElementCommand::new(class_req(a, root, "a"))),
    )?;
    ensure!(model.get_element(a).is_ok());

    // Undo: A is deleted (inverse delta replayed).
    stack.undo(&mut model)?;
    ensure!(
        model.get_element(a).is_err(),
        "undo must remove the element"
    );
    ensure!(!stack.can_undo());
    ensure!(stack.can_redo());

    // Redo: A is re-created (forward delta replayed).
    stack.redo(&mut model)?;
    ensure!(
        model.get_element(a).is_ok(),
        "redo must re-create the element"
    );
    ensure!(stack.can_undo());
    ensure!(!stack.can_redo());

    // The re-created element has the same fields.
    let view = model.get_element(a)?;
    ensure_eq!(view.name.as_deref(), Some("a"));
    ensure_eq!(view.owner, Some(root));
    ensure_eq!(view.kind, MetaclassKind::Class);
    ensure_eq!(view.visibility, Visibility::Public);

    Ok(())
}
