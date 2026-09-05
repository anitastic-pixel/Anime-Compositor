# Undo, redo and command transaction model

Version 0.2 | 2026-09-04 | Proposed baseline

## Command contract

All persistent project edits enter through a command interface. A command validates intent against the current document revision, applies one atomic logical change, returns diagnostics, and supplies sufficient inverse data to restore the exact prior model state.

UI-only actions such as zoom, panel focus and playback do not enter project undo history. Saving does not create an undo item.

## History semantics

The document holds an undo stack and redo stack. A successful new command clears redo history. Undo applies the stored inverse as one transaction and moves the item to redo. Redo reapplies the original semantic command against the restored state.

Undo/redo must restore project dirty state correctly. If the current document state becomes byte/semantic-equivalent to the last successful save revision, dirty becomes false even if history contains later redoable commands.

## Command contents

Each history record contains: command ID, human-readable label, affected stable IDs, before/after values or a reversible operation payload, source document revision and timestamp for diagnostics. It must not retain live UI objects or unsafe raw pointers.

Large media bytes are not duplicated in history; commands store asset records/references sufficient to restore the project model. Cache contents are never authoritative history.

## Coalescing

Continuous numeric drags, viewer transforms and similar gestures are interaction transactions from 24. Intermediate previews may update a transient working value, but release creates one history record from original to final value. Text edits may coalesce while one field has focus; committing/focus exit ends the transaction.

Repeated discrete commands such as layer nudges are not coalesced unless an explicit future policy is tested.

## Multi-object transactions

Operations such as importing media plus creating a layer, deleting a matte with dependent-reference repair, or relinking several occurrences must either succeed as one transaction or change nothing. Partial success is not permitted unless the UI explicitly presents a batch result and each accepted item is a separate command.

## Validation and failure

A rejected command does not change document revision, dirty state, undo stack or caches. Diagnostics use 28. Commands validate cycles, missing IDs, locked state, type/range constraints and schema invariants before commit where feasible.

## Persistence interaction

Save serializes a stable snapshot and records the saved semantic revision. Undo after save is allowed and makes the project dirty if it differs from the saved snapshot. Saving while an interaction drag is active is disallowed or first commits/cancels that transaction deterministically.

Autosave does not clear user-facing dirty state and does not replace the canonical manual-save path.

## Cache interaction

Every committed command reports invalidation domains to 27. Undo/redo emit the same semantic invalidations as the corresponding model change; they never restore stale cached pixels from history.

## Required tests

- scalar property edit, undo, redo exact value;
- layer reorder restores exact order and references;
- deleting/recovering a matte preserves dependent records;
- import/create-layer transaction is all-or-nothing;
- drag of 100 intermediate values produces one undo item;
- rejected command produces no revision/history change;
- save -> edit -> undo back to saved state clears dirty;
- undo/redo after project reopen is empty unless a future explicit persisted-history feature is added.

Related documents: 03, 19, 24 and 27.
