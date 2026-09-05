//! Commands, undo and redo, per document 26.
//!
//! Document 26: "All persistent project edits enter through a command interface. A command
//! validates intent against the current document revision, applies one atomic logical change,
//! returns diagnostics, and supplies sufficient inverse data to restore the exact prior model
//! state."
//!
//! [`Document`] owns the project and is the only thing that can change it. Every mutating
//! method returns `Result<&Record, Diagnostic>`; a rejected command changes nothing at all,
//! which document 26 states as "A rejected command does not change document revision, dirty
//! state, undo stack or caches".
//!
//! Not done here, and deliberately: cache invalidation domains (document 27, no cache exists
//! yet — B-07), and persisting history across a reopen, which document 26 explicitly does not
//! ask for ("undo/redo after project reopen is empty").

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::model::{
    Asset, Composition, Id, Interp, Keyframe, Layer, MatteReference, Project, Prop, Value,
};

/// One user action. Document 26 requires a stable command ID and a human-readable label on
/// every history record; both are derived from the variant rather than passed in, so a caller
/// cannot mislabel history.
#[derive(Clone, PartialEq, Debug)]
pub enum Command {
    AddAsset {
        asset: Asset,
    },
    AddLayer {
        composition: Id,
        layer: Box<Layer>,
        index: usize,
    },
    RemoveLayer {
        composition: Id,
        layer_id: Id,
    },
    RenameLayer {
        composition: Id,
        layer_id: Id,
        name: String,
    },
    SetLayerEnabled {
        composition: Id,
        layer_id: Id,
        value: bool,
    },
    SetLayerLocked {
        composition: Id,
        layer_id: Id,
        value: bool,
    },
    ReorderLayer {
        composition: Id,
        layer_id: Id,
        to_index: usize,
    },
    SetPropertyBase {
        composition: Id,
        layer_id: Id,
        prop: Prop,
        value: Value,
    },
    SetKeyframe {
        composition: Id,
        layer_id: Id,
        prop: Prop,
        frame: i32,
        value: Value,
        interp: Interp,
    },
    RemoveKeyframe {
        composition: Id,
        layer_id: Id,
        prop: Prop,
        frame: i32,
    },
    SetMatte {
        composition: Id,
        layer_id: Id,
        matte: Option<Id>,
    },
}

impl Command {
    /// The stable machine identifier document 26 requires on each history record.
    pub fn command_id(&self) -> &'static str {
        match self {
            Command::AddAsset { .. } => "ADD_ASSET",
            Command::AddLayer { .. } => "ADD_LAYER",
            Command::RemoveLayer { .. } => "REMOVE_LAYER",
            Command::RenameLayer { .. } => "RENAME_LAYER",
            Command::SetLayerEnabled { .. } => "SET_LAYER_ENABLED",
            Command::SetLayerLocked { .. } => "SET_LAYER_LOCKED",
            Command::ReorderLayer { .. } => "REORDER_LAYER",
            Command::SetPropertyBase { .. } => "SET_PROPERTY",
            Command::SetKeyframe { .. } => "SET_KEYFRAME",
            Command::RemoveKeyframe { .. } => "REMOVE_KEYFRAME",
            Command::SetMatte { .. } => "SET_MATTE",
        }
    }

    /// The label a user would see in a history panel, in their words rather than the model's.
    pub fn label(&self) -> String {
        match self {
            Command::AddAsset { asset } => format!("Import {}", asset.name),
            Command::AddLayer { layer, .. } => format!("Add layer {}", layer.name),
            Command::RemoveLayer { layer_id, .. } => format!("Delete layer {layer_id}"),
            Command::RenameLayer { name, .. } => format!("Rename layer to {name}"),
            Command::SetLayerEnabled { value, .. } => {
                format!("{} layer", if *value { "Show" } else { "Hide" })
            }
            Command::SetLayerLocked { value, .. } => {
                format!("{} layer", if *value { "Lock" } else { "Unlock" })
            }
            Command::ReorderLayer { to_index, .. } => format!("Move layer to position {to_index}"),
            Command::SetPropertyBase { prop, value, .. } => format!("Set {prop} to {value}"),
            Command::SetKeyframe {
                prop, frame, value, ..
            } => format!("Keyframe {prop} at frame {frame} to {value}"),
            Command::RemoveKeyframe { prop, frame, .. } => {
                format!("Remove {prop} keyframe at frame {frame}")
            }
            Command::SetMatte { matte, .. } => match matte {
                Some(id) => format!("Set matte to {id}"),
                None => "Clear matte".to_string(),
            },
        }
    }

    fn composition(&self) -> Option<&Id> {
        match self {
            Command::AddAsset { .. } => None,
            Command::AddLayer { composition, .. }
            | Command::RemoveLayer { composition, .. }
            | Command::RenameLayer { composition, .. }
            | Command::SetLayerEnabled { composition, .. }
            | Command::SetLayerLocked { composition, .. }
            | Command::ReorderLayer { composition, .. }
            | Command::SetPropertyBase { composition, .. }
            | Command::SetKeyframe { composition, .. }
            | Command::RemoveKeyframe { composition, .. }
            | Command::SetMatte { composition, .. } => Some(composition),
        }
    }

    /// The stable IDs a history record must name as affected.
    pub fn affected(&self) -> Vec<Id> {
        let mut ids: Vec<Id> = self.composition().cloned().into_iter().collect();
        match self {
            Command::AddAsset { asset } => ids.push(asset.id.clone()),
            Command::AddLayer { layer, .. } => ids.push(layer.id.clone()),
            Command::RemoveLayer { layer_id, .. }
            | Command::RenameLayer { layer_id, .. }
            | Command::SetLayerEnabled { layer_id, .. }
            | Command::SetLayerLocked { layer_id, .. }
            | Command::ReorderLayer { layer_id, .. }
            | Command::SetPropertyBase { layer_id, .. }
            | Command::SetKeyframe { layer_id, .. }
            | Command::RemoveKeyframe { layer_id, .. } => ids.push(layer_id.clone()),
            Command::SetMatte {
                layer_id, matte, ..
            } => {
                ids.push(layer_id.clone());
                ids.extend(matte.clone());
            }
        }
        ids
    }

    /// True for commands a locked layer must refuse.
    ///
    /// Unlocking is not one of them: a lock the user cannot undo would be a trap.
    fn blocked_by_lock(&self) -> bool {
        !matches!(
            self,
            Command::SetLayerLocked { .. } | Command::AddAsset { .. } | Command::AddLayer { .. }
        )
    }

    fn layer_id(&self) -> Option<&Id> {
        match self {
            Command::RemoveLayer { layer_id, .. }
            | Command::RenameLayer { layer_id, .. }
            | Command::SetLayerEnabled { layer_id, .. }
            | Command::SetLayerLocked { layer_id, .. }
            | Command::ReorderLayer { layer_id, .. }
            | Command::SetPropertyBase { layer_id, .. }
            | Command::SetKeyframe { layer_id, .. }
            | Command::RemoveKeyframe { layer_id, .. }
            | Command::SetMatte { layer_id, .. } => Some(layer_id),
            _ => None,
        }
    }
}

/// One entry in the undo or redo stack.
///
/// Document 26 lists what it must hold: "command ID, human-readable label, affected stable
/// IDs, before/after values or a reversible operation payload, source document revision and
/// timestamp for diagnostics. It must not retain live UI objects or unsafe raw pointers."
///
/// The reversible payload here is the whole project as it stood before the change.
///
/// ponytail: whole-project snapshot per record. The model holds no pixels — document 26 says
/// "Large media bytes are not duplicated in history", and media lives in the asset record, not
/// the project graph — so a snapshot is a few kilobytes and drags coalesce into one record.
/// Narrow it to per-layer inverses if a project ever grows enough for this to show up.
#[derive(Clone, Debug)]
pub struct Record {
    pub command_id: &'static str,
    pub label: String,
    pub affected: Vec<Id>,
    pub source_revision: u64,
    /// The commands as applied, in order. More than one only for a transaction.
    pub commands: Vec<Command>,
    before: Project,
}

/// A drag in progress. Document 26: "release creates one history record from original to
/// final value."
struct Drag {
    before: Project,
    commands: Vec<Command>,
    updates: usize,
}

/// The project plus its history. The only way to change a project.
pub struct Document {
    project: Project,
    revision: u64,
    /// The state at open, or at the last successful save. Dirty is measured against this.
    baseline: Project,
    undo: Vec<Record>,
    redo: Vec<Record>,
    drag: Option<Drag>,
}

impl Document {
    pub fn new(project: Project) -> Self {
        Document {
            baseline: project.clone(),
            project,
            revision: 0,
            undo: Vec::new(),
            redo: Vec::new(),
            drag: None,
        }
    }

    pub fn project(&self) -> &Project {
        &self.project
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn undo_depth(&self) -> usize {
        self.undo.len()
    }
    pub fn redo_depth(&self) -> usize {
        self.redo.len()
    }
    pub fn undo_labels(&self) -> Vec<&str> {
        self.undo.iter().map(|r| r.label.as_str()).collect()
    }

    /// Document 26: "If the current document state becomes byte/semantic-equivalent to the
    /// last successful save revision, dirty becomes false even if history contains later
    /// redoable commands."
    ///
    /// So dirty is a comparison, not a flag. Undoing back to the saved state clears it.
    pub fn is_dirty(&self) -> bool {
        self.project != self.baseline
    }

    /// Document 26: "Save serializes a stable snapshot and records the saved semantic
    /// revision." Saving does not create an undo item.
    pub fn mark_saved(&mut self) {
        self.baseline = self.project.clone();
    }

    /// Apply one command as one history record.
    pub fn apply(&mut self, command: Command) -> Result<&Record, Diagnostic> {
        self.apply_all(vec![command])
    }

    /// Apply several commands as one all-or-nothing transaction with one history record.
    ///
    /// Document 26: "Operations such as importing media plus creating a layer ... must either
    /// succeed as one transaction or change nothing." The work happens on a clone, so a
    /// failure on the third command cannot leave the first two applied.
    pub fn apply_all(&mut self, commands: Vec<Command>) -> Result<&Record, Diagnostic> {
        if commands.is_empty() {
            return Err(reject("A command transaction contained no commands.", ""));
        }
        if self.drag.is_some() {
            return Err(reject(
                "A drag is in progress, so this edit cannot be applied yet.",
                "Document 26: an interaction transaction must be committed or cancelled before \
                 another command runs.",
            ));
        }
        let before = self.project.clone();
        let mut working = self.project.clone();
        for command in &commands {
            apply_to(&mut working, command)?;
        }
        self.project = working;
        self.revision += 1;
        self.redo.clear();
        let first = &commands[0];
        let record = Record {
            command_id: first.command_id(),
            label: if commands.len() == 1 {
                first.label()
            } else {
                format!("{} and {} more", first.label(), commands.len() - 1)
            },
            affected: commands.iter().flat_map(Command::affected).collect(),
            source_revision: self.revision - 1,
            commands,
            before,
        };
        self.undo.push(record);
        Ok(self.undo.last().expect("just pushed"))
    }

    /// Begin an interaction transaction: a drag whose intermediate values must not each become
    /// an undo item.
    pub fn begin_drag(&mut self) -> Result<(), Diagnostic> {
        if self.drag.is_some() {
            return Err(reject(
                "A drag is already in progress.",
                "Document 26: interaction transactions do not nest.",
            ));
        }
        self.drag = Some(Drag {
            before: self.project.clone(),
            commands: Vec::new(),
            updates: 0,
        });
        Ok(())
    }

    /// One intermediate value of a drag. Document 26: "Intermediate previews may update a
    /// transient working value" — the model changes, history does not.
    pub fn update_drag(&mut self, command: Command) -> Result<(), Diagnostic> {
        let Some(drag) = self.drag.as_mut() else {
            return Err(reject(
                "No drag is in progress.",
                "update_drag was called without begin_drag.",
            ));
        };
        // Validate against a clone so a rejected intermediate leaves the working value alone.
        let mut working = self.project.clone();
        apply_to(&mut working, &command)?;
        self.project = working;
        drag.updates += 1;
        drag.commands.clear();
        drag.commands.push(command);
        Ok(())
    }

    /// Release. One history record, from the value before the drag to the final value.
    ///
    /// Returns `None` if the drag never moved anything, which is not an edit and must not
    /// enter history.
    pub fn end_drag(&mut self) -> Option<&Record> {
        let drag = self.drag.take()?;
        if drag.commands.is_empty() || drag.before == self.project {
            self.project = drag.before;
            return None;
        }
        self.revision += 1;
        self.redo.clear();
        let first = drag.commands[0].clone();
        self.undo.push(Record {
            command_id: first.command_id(),
            label: first.label(),
            affected: first.affected(),
            source_revision: self.revision - 1,
            commands: drag.commands,
            before: drag.before,
        });
        self.undo.last()
    }

    /// Abandon a drag and restore the value it started from.
    pub fn cancel_drag(&mut self) {
        if let Some(drag) = self.drag.take() {
            self.project = drag.before;
        }
    }

    pub fn drag_in_progress(&self) -> bool {
        self.drag.is_some()
    }

    /// Document 26: "Undo applies the stored inverse as one transaction and moves the item to
    /// redo."
    pub fn undo(&mut self) -> Option<&Record> {
        let mut record = self.undo.pop()?;
        std::mem::swap(&mut self.project, &mut record.before);
        self.revision += 1;
        self.redo.push(record);
        self.redo.last()
    }

    /// Document 26: "Redo reapplies the original semantic command against the restored state."
    ///
    /// Reapplied, not restored from a stored after-image, which is the wording the document
    /// uses and the stricter of the two: it fails loudly if a command is not deterministic.
    pub fn redo(&mut self) -> Option<&Record> {
        let record = self.redo.pop()?;
        let before = self.project.clone();
        let mut working = self.project.clone();
        for command in &record.commands {
            if apply_to(&mut working, command).is_err() {
                // Cannot happen for a command that succeeded once against this same state.
                self.redo.push(record);
                return None;
            }
        }
        self.project = working;
        self.revision += 1;
        self.undo.push(Record { before, ..record });
        self.undo.last()
    }
}

fn reject(message: &str, detail: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::CommandInvalidValue,
        Severity::Error,
        message,
        detail,
    )
}

fn missing(message: String, detail: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::CommandTargetMissing,
        Severity::Error,
        message,
        detail,
    )
    .with_remediation("The edit was not applied. Nothing in the project changed.")
}

/// Validate and apply one command to a project. Every failure path returns before mutating.
fn apply_to(project: &mut Project, command: &Command) -> Result<(), Diagnostic> {
    if let Command::AddAsset { asset } = command {
        if project.assets.iter().any(|a| a.id == asset.id) {
            return Err(reject(
                &format!(
                    "An asset with the ID {} is already in the project.",
                    asset.id
                ),
                "Document 19: stable IDs are unique within a project.",
            ));
        }
        project.assets.push(asset.clone());
        return Ok(());
    }

    let comp_id = command
        .composition()
        .expect("only AddAsset has none")
        .clone();
    // Locked and existence checks read the composition before anything is mutated.
    {
        let comp = project.composition(&comp_id).ok_or_else(|| {
            missing(
                format!("The composition {comp_id} is not in this project."),
                format!(
                    "Command {} named a composition ID that does not exist.",
                    command.command_id()
                ),
            )
        })?;
        if let Some(layer_id) = command.layer_id() {
            let layer = comp.layer(layer_id).ok_or_else(|| {
                missing(
                    format!("The layer {layer_id} is not in this composition."),
                    format!(
                        "Command {} named a layer ID that does not exist.",
                        command.command_id()
                    ),
                )
            })?;
            if layer.locked && command.blocked_by_lock() {
                return Err(Diagnostic::new(
                    DiagnosticId::CommandLayerLocked,
                    Severity::Error,
                    format!(
                        "The layer \"{}\" is locked, so it was not changed.",
                        layer.name
                    ),
                    format!(
                        "Command {} was rejected by the lock on layer {layer_id}.",
                        command.command_id()
                    ),
                )
                .with_remediation("Unlock the layer to edit it."));
            }
        }
    }

    match command {
        Command::AddAsset { .. } => unreachable!("handled above"),
        Command::AddLayer { layer, index, .. } => {
            let asset_known = project.assets.iter().any(|a| a.id == layer.asset_id);
            let comp = comp_mut(project, &comp_id)?;
            if comp.layer(&layer.id).is_some() {
                return Err(reject(
                    &format!(
                        "A layer with the ID {} is already in this composition.",
                        layer.id
                    ),
                    "Document 19: stable IDs are unique.",
                ));
            }
            if !layer.timing_is_valid() {
                return Err(reject(
                    &format!(
                        "Layer \"{}\" would start at frame {} and end at frame {}, which is not a span.",
                        layer.name, layer.in_frame, layer.out_frame
                    ),
                    "Document 19 invariant: in_frame < out_frame.",
                ));
            }
            if !asset_known {
                return Err(missing(
                    format!(
                        "The media for layer \"{}\" is not in this project.",
                        layer.name
                    ),
                    format!(
                        "Layer {} refers to asset {}, which no asset record matches.",
                        layer.id, layer.asset_id
                    ),
                ));
            }
            if *index > comp.len() {
                return Err(reject(
                    &format!(
                        "Position {index} is past the end of a stack of {} layers.",
                        comp.len()
                    ),
                    "Layer order index out of range.",
                ));
            }
            comp.insert_layer((**layer).clone(), *index);
        }
        Command::RemoveLayer { layer_id, .. } => {
            comp_mut(project, &comp_id)?.remove_layer(layer_id);
        }
        Command::RenameLayer { layer_id, name, .. } => {
            if name.trim().is_empty() {
                return Err(reject(
                    "A layer name cannot be empty.",
                    "Document 19: display names are not identity, but they are still shown.",
                ));
            }
            layer_mut(project, &comp_id, layer_id)?.name = name.clone();
        }
        Command::SetLayerEnabled {
            layer_id, value, ..
        } => {
            layer_mut(project, &comp_id, layer_id)?.enabled = *value;
        }
        Command::SetLayerLocked {
            layer_id, value, ..
        } => {
            layer_mut(project, &comp_id, layer_id)?.locked = *value;
        }
        Command::ReorderLayer {
            layer_id, to_index, ..
        } => {
            let comp = comp_mut(project, &comp_id)?;
            if *to_index >= comp.len() {
                return Err(reject(
                    &format!(
                        "Position {to_index} is past the end of a stack of {} layers.",
                        comp.len()
                    ),
                    "Layer order index out of range.",
                ));
            }
            comp.move_layer(layer_id, *to_index);
        }
        Command::SetPropertyBase {
            layer_id,
            prop,
            value,
            ..
        } => {
            let value = check_value(*prop, *value)?;
            layer_mut(project, &comp_id, layer_id)?
                .transform
                .get_mut(*prop)
                .set_base(value);
        }
        Command::SetKeyframe {
            layer_id,
            prop,
            frame,
            value,
            interp,
            ..
        } => {
            let value = check_value(*prop, *value)?;
            layer_mut(project, &comp_id, layer_id)?
                .transform
                .get_mut(*prop)
                .set_keyframe(Keyframe {
                    frame: *frame,
                    value,
                    interp: *interp,
                });
        }
        Command::RemoveKeyframe {
            layer_id,
            prop,
            frame,
            ..
        } => {
            let removed = layer_mut(project, &comp_id, layer_id)?
                .transform
                .get_mut(*prop)
                .remove_keyframe(*frame);
            if removed.is_none() {
                return Err(missing(
                    format!("There is no {prop} keyframe at frame {frame} to remove."),
                    format!("Layer {layer_id} has no {prop} keyframe at frame {frame}."),
                ));
            }
        }
        Command::SetMatte {
            layer_id, matte, ..
        } => {
            if let Some(target) = matte {
                let comp = project.composition(&comp_id).expect("checked above");
                if comp.layer(target).is_none() {
                    return Err(Diagnostic::new(
                        DiagnosticId::MatteReferenceMissing,
                        Severity::Error,
                        format!("The layer chosen as a matte, {target}, is not in this composition."),
                        "Document 19: a matte dependency must refer to a layer in the same composition.".to_string(),
                    ));
                }
            }
            let matte_ref = matte.clone().map(|layer_id| MatteReference { layer_id });
            layer_mut(project, &comp_id, layer_id)?.matte = matte_ref;
            // Checked after the write, then rolled back by the caller's working clone if bad.
            let comp = project.composition(&comp_id).expect("checked above");
            if comp.matte_cycle_from(layer_id) {
                return Err(Diagnostic::new(
                    DiagnosticId::MatteCycle,
                    Severity::Error,
                    "That matte would make two layers depend on each other.".to_string(),
                    format!("Setting the matte of {layer_id} to {matte:?} closes a cycle in the matte graph."),
                )
                .with_remediation("Choose a layer that does not already use this one as its matte."));
            }
        }
    }
    Ok(())
}

fn comp_mut<'a>(project: &'a mut Project, id: &Id) -> Result<&'a mut Composition, Diagnostic> {
    project.composition_mut(id).ok_or_else(|| {
        missing(
            format!("The composition {id} is not in this project."),
            String::new(),
        )
    })
}

fn layer_mut<'a>(
    project: &'a mut Project,
    comp_id: &Id,
    layer_id: &Id,
) -> Result<&'a mut Layer, Diagnostic> {
    comp_mut(project, comp_id)?
        .layer_mut(layer_id)
        .ok_or_else(|| {
            missing(
                format!("The layer {layer_id} is not in this composition."),
                String::new(),
            )
        })
}

/// Document 20: "Opacity is clamped to 0..1 at command validation. Scale may be negative to
/// permit mirroring." Clamped, not rejected: an artist dragging opacity past the end of its
/// slider means the end of the slider.
fn check_value(prop: Prop, value: Value) -> Result<Value, Diagnostic> {
    if value.kind() != prop.kind() {
        return Err(reject(
            &format!(
                "{prop} takes a {} value, not a {}.",
                prop.kind(),
                value.kind()
            ),
            "Document 19: anchor, position and scale are vec2; rotation and opacity are scalar.",
        ));
    }
    if !finite(value) {
        return Err(reject(
            &format!("{prop} cannot be set to {value}."),
            "Property values must be finite numbers.",
        ));
    }
    Ok(match (prop, value) {
        (Prop::Opacity, Value::Scalar(v)) => Value::Scalar(v.clamp(0.0, 1.0)),
        _ => value,
    })
}

fn finite(value: Value) -> bool {
    match value {
        Value::Scalar(v) => v.is_finite(),
        Value::Vec2(x, y) => x.is_finite() && y.is_finite(),
    }
}
