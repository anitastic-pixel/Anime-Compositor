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
use crate::time::{ExposureMap, ExposureSpan};

/// One user action. Document 26 requires a stable command ID and a human-readable label on
/// every history record; both are derived from the variant rather than passed in, so a caller
/// cannot mislabel history.
#[derive(Clone, PartialEq, Debug)]
pub enum Command {
    AddAsset {
        asset: Asset,
    },
    /// Point an existing asset record at different files. Document 02: "Undo restores the
    /// prior reference." The whole record is replaced, so the inverse is the record that was
    /// there; layer IDs, transforms and effects are untouched by construction, because this
    /// command cannot reach a layer at all.
    RelinkAsset {
        asset: Box<Asset>,
    },
    /// Document 24's `composition.create`. B-12d.
    ///
    /// W-01 lists "creates a composition" third of thirteen and no command reached it: the
    /// composition a person worked in was whichever one their project file already held, so a
    /// project with none could not be opened into anything. The whole record is the unit of
    /// change because document 19 line 52 makes size and duration a property of the composition
    /// rather than of a later edit, and because there is nothing else to change yet - a new
    /// composition has no layers, and every command that would give it one already exists.
    ///
    /// No inverse data is carried, for the reason every other command here carries none: the
    /// document stores the whole project as it was before the record.
    AddComposition {
        composition: Box<Composition>,
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
    /// Move a layer along the timeline. W-05.
    ///
    /// Document 20: "Moving a layer changes in_frame/out_frame; trimming and changing source
    /// offset are distinct commands." This is the move: the layer keeps its length and its
    /// source offset, so the same drawing sits under each frame of it as before, all of them
    /// shifted together. Document 24 named no ID for it and the core had no command, so until
    /// this every layer began where it was added.
    ShiftLayer {
        composition: Id,
        layer_id: Id,
        in_frame: i32,
    },
    /// Trim either end of a layer. W-05.
    ///
    /// The other half of document 20's sentence. The drawing under each frame that survives
    /// the trim stays the drawing that was under it, which is what a person pulling the end of
    /// a bar in After Effects or Premiere expects; that means the source offset moves with the
    /// in point, and this command does that arithmetic so that a caller cannot get it wrong.
    TrimLayer {
        composition: Id,
        layer_id: Id,
        in_frame: i32,
        out_frame: i32,
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
        /// D-42: whether the layer named in `matte` stops being drawn in its own right.
        /// Ignored when `matte` is `None`, since there is then no layer to keep out of the stack.
        matte_only: bool,
    },
    /// Document 24's `exposure.set_span`. B-12a.
    ///
    /// The whole ordered list is the unit of change, for the reason [`Command::SetMask`] gives.
    /// Document 20 requires "the unique ExposureSpan" covering a frame, which is a property of
    /// the list rather than of any one span in it: a command that added one span at a time
    /// would have to pass through overlapping states this build rejects to reach an ordering
    /// that is legal, and rejecting the step would make the destination unreachable.
    ///
    /// Document 24 named this ID and no command existed for it. This is the second such
    /// omission after `property.set_base`, and it is registered here rather than assumed.
    SetExposureSpans {
        composition: Id,
        layer_id: Id,
        spans: Vec<ExposureSpan>,
    },
    /// Set or clear a layer's polygon mask. B-06.
    ///
    /// The whole mask is the unit of change, not a vertex, because document 19 makes
    /// self-intersection a property of the polygon rather than of any one point in it: a vertex
    /// moved one at a time would have to pass through states this build rejects to get anywhere.
    /// Editing a mask is therefore a drag that commits one `SetMask`, which is the same shape
    /// document 26 already gives a transform drag.
    SetMask {
        composition: Id,
        layer_id: Id,
        mask: Option<crate::mask::PolygonMask>,
    },
    /// Document 24's `effect.add`. B-07.
    ///
    /// `index` is where in the stack it lands, because order changes the picture: a blur then a
    /// tint is not a tint then a blur. `None` appends, which is what an inspector's "add" does.
    AddEffect {
        composition: Id,
        layer_id: Id,
        effect: crate::effects::EffectInstance,
        index: Option<usize>,
    },
    /// Document 24's `effect.delete`. B-07.
    RemoveEffect {
        composition: Id,
        layer_id: Id,
        instance_id: Id,
    },
    /// Document 24's `effect.toggle_bypass`. B-07.
    ///
    /// Bypassing is not deleting and is not a fault: the record stays, the picture changes, and
    /// nothing is written to the frame log, because a person choosing to switch an effect off is
    /// not this build failing to draw one.
    SetEffectEnabled {
        composition: Id,
        layer_id: Id,
        instance_id: Id,
        enabled: bool,
    },
    /// Document 24's `effect.move`. W-03.
    ///
    /// The stack is evaluated in order and ADR-017 makes that order the picture: a blur then a
    /// tint is not a tint then a blur, which is the same sentence `AddEffect` is written with
    /// and the reason it takes an index. Until this command there was nowhere to send a stack
    /// that had been built in the wrong order except delete and add again, which loses the
    /// parameters with it.
    ReorderEffect {
        composition: Id,
        layer_id: Id,
        instance_id: Id,
        to_index: usize,
    },
    /// Change one effect's parameters. B-07.
    ///
    /// The whole parameter set is the unit of change, for the reason `SetMask` gives: a tint has
    /// a colour and an amount, and an inspector edit that committed one of them at a time would
    /// put half-changed states in the history for no gain.
    ///
    /// Document 24 lists add, delete and bypass and no parameter command; this is the fourth
    /// thing an inspector must be able to do, and it is registered here rather than assumed.
    SetEffectParameters {
        composition: Id,
        layer_id: Id,
        instance_id: Id,
        effect: crate::effects::Effect,
    },
}

impl Command {
    /// The stable machine identifier document 26 requires on each history record.
    pub fn command_id(&self) -> &'static str {
        match self {
            Command::AddAsset { .. } => "ADD_ASSET",
            Command::RelinkAsset { .. } => "RELINK_ASSET",
            Command::AddComposition { .. } => "ADD_COMPOSITION",
            Command::AddLayer { .. } => "ADD_LAYER",
            Command::RemoveLayer { .. } => "REMOVE_LAYER",
            Command::RenameLayer { .. } => "RENAME_LAYER",
            Command::SetLayerEnabled { .. } => "SET_LAYER_ENABLED",
            Command::SetLayerLocked { .. } => "SET_LAYER_LOCKED",
            Command::ReorderLayer { .. } => "REORDER_LAYER",
            Command::ShiftLayer { .. } => "SHIFT_LAYER",
            Command::TrimLayer { .. } => "TRIM_LAYER",
            Command::SetPropertyBase { .. } => "SET_PROPERTY",
            Command::SetKeyframe { .. } => "SET_KEYFRAME",
            Command::RemoveKeyframe { .. } => "REMOVE_KEYFRAME",
            Command::SetMatte { .. } => "SET_MATTE",
            Command::SetExposureSpans { .. } => "SET_EXPOSURE_SPANS",
            Command::SetMask { .. } => "SET_MASK",
            Command::AddEffect { .. } => "ADD_EFFECT",
            Command::RemoveEffect { .. } => "REMOVE_EFFECT",
            Command::ReorderEffect { .. } => "REORDER_EFFECT",
            Command::SetEffectEnabled { .. } => "SET_EFFECT_ENABLED",
            Command::SetEffectParameters { .. } => "SET_EFFECT_PARAMETERS",
        }
    }

    /// The label a user would see in a history panel, in their words rather than the model's.
    pub fn label(&self) -> String {
        match self {
            Command::AddAsset { asset } => format!("Import {}", asset.name),
            Command::RelinkAsset { asset } => format!("Relink {}", asset.name),
            Command::AddComposition { composition } => {
                format!("New composition {}", composition.name)
            }
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
            Command::ShiftLayer { in_frame, .. } => {
                format!("Move layer to start at frame {in_frame}")
            }
            Command::TrimLayer {
                in_frame,
                out_frame,
                ..
            } => format!("Trim layer to frames {in_frame} to {out_frame}"),
            Command::SetPropertyBase { prop, value, .. } => format!("Set {prop} to {value}"),
            Command::SetKeyframe {
                prop, frame, value, ..
            } => format!("Keyframe {prop} at frame {frame} to {value}"),
            Command::RemoveKeyframe { prop, frame, .. } => {
                format!("Remove {prop} keyframe at frame {frame}")
            }
            Command::SetMatte {
                matte, matte_only, ..
            } => match matte {
                Some(id) if *matte_only => format!("Set matte to {id}, matte only"),
                Some(id) => format!("Set matte to {id}"),
                None => "Clear matte".to_string(),
            },
            Command::SetExposureSpans { spans, .. } => match spans.len() {
                0 => "Clear the exposures".to_string(),
                1 => "Set one exposure".to_string(),
                n => format!("Set {n} exposures"),
            },
            Command::SetMask { mask, .. } => match mask {
                Some(m) => format!("Set mask of {} points", m.vertices.len()),
                None => "Clear mask".to_string(),
            },
            Command::AddEffect { effect, .. } => format!("Add {}", effect.type_id()),
            Command::RemoveEffect { instance_id, .. } => format!("Remove effect {instance_id}"),
            Command::ReorderEffect {
                instance_id,
                to_index,
                ..
            } => format!("Move effect {instance_id} to position {to_index}"),
            Command::SetEffectEnabled {
                instance_id,
                enabled,
                ..
            } => {
                if *enabled {
                    format!("Switch effect {instance_id} on")
                } else {
                    format!("Bypass effect {instance_id}")
                }
            }
            Command::SetEffectParameters { effect, .. } => {
                format!("Change {} settings", effect.type_id())
            }
        }
    }

    fn composition(&self) -> Option<&Id> {
        match self {
            Command::AddAsset { .. }
            | Command::RelinkAsset { .. }
            | Command::AddComposition { .. } => None,
            Command::AddLayer { composition, .. }
            | Command::RemoveLayer { composition, .. }
            | Command::RenameLayer { composition, .. }
            | Command::SetLayerEnabled { composition, .. }
            | Command::SetLayerLocked { composition, .. }
            | Command::ReorderLayer { composition, .. }
            | Command::ShiftLayer { composition, .. }
            | Command::TrimLayer { composition, .. }
            | Command::SetPropertyBase { composition, .. }
            | Command::SetKeyframe { composition, .. }
            | Command::RemoveKeyframe { composition, .. }
            | Command::SetMatte { composition, .. }
            | Command::SetExposureSpans { composition, .. }
            | Command::SetMask { composition, .. }
            | Command::AddEffect { composition, .. }
            | Command::RemoveEffect { composition, .. }
            | Command::ReorderEffect { composition, .. }
            | Command::SetEffectEnabled { composition, .. }
            | Command::SetEffectParameters { composition, .. } => Some(composition),
        }
    }

    /// The stable IDs a history record must name as affected.
    pub fn affected(&self) -> Vec<Id> {
        let mut ids: Vec<Id> = self.composition().cloned().into_iter().collect();
        match self {
            Command::AddAsset { asset } => ids.push(asset.id.clone()),
            Command::RelinkAsset { asset } => ids.push(asset.id.clone()),
            Command::AddComposition { composition } => ids.push(composition.id.clone()),
            Command::AddLayer { layer, .. } => ids.push(layer.id.clone()),
            Command::RemoveLayer { layer_id, .. }
            | Command::RenameLayer { layer_id, .. }
            | Command::SetLayerEnabled { layer_id, .. }
            | Command::SetLayerLocked { layer_id, .. }
            | Command::ReorderLayer { layer_id, .. }
            | Command::ShiftLayer { layer_id, .. }
            | Command::TrimLayer { layer_id, .. }
            | Command::SetPropertyBase { layer_id, .. }
            | Command::SetKeyframe { layer_id, .. }
            | Command::RemoveKeyframe { layer_id, .. }
            | Command::SetExposureSpans { layer_id, .. }
            | Command::SetMask { layer_id, .. }
            | Command::RemoveEffect { layer_id, .. }
            | Command::ReorderEffect { layer_id, .. }
            | Command::SetEffectEnabled { layer_id, .. }
            | Command::SetEffectParameters { layer_id, .. } => ids.push(layer_id.clone()),
            Command::AddEffect {
                layer_id, effect, ..
            } => {
                ids.push(layer_id.clone());
                ids.push(effect.instance_id.clone());
            }
            Command::SetMatte {
                layer_id, matte, ..
            } => {
                ids.push(layer_id.clone());
                ids.extend(matte.clone());
            }
        }
        ids
    }

    /// Whether this command is a later reading of the same thing `other` moved, so that during
    /// a drag it replaces `other` rather than joining it. Two moves of one layer's position are
    /// one control moved twice; that layer's position and its neighbour's are two controls.
    ///
    /// The property is part of what makes a control and not the layer alone: a corner drag that
    /// sent a scale and a position for one layer sends two things that must both survive.
    fn refines(&self, other: &Command) -> bool {
        match (self, other) {
            (
                Command::SetPropertyBase { prop: mine, .. },
                Command::SetPropertyBase { prop: theirs, .. },
            ) if mine != theirs => return false,
            // W-10: the same control, keyed. A drag on a keyframed property sends a keyframe at
            // the frame under the playhead each time the pointer moves, and two of those at one
            // frame are one key set twice; a key on another frame or another property is not.
            (
                Command::SetKeyframe {
                    prop: mine,
                    frame: at,
                    ..
                },
                Command::SetKeyframe {
                    prop: theirs,
                    frame: other_at,
                    ..
                },
            ) if mine != theirs || at != other_at => return false,
            _ => {}
        }
        self.command_id() == other.command_id() && self.affected() == other.affected()
    }

    /// True for commands a locked layer must refuse.
    ///
    /// Unlocking is not one of them: a lock the user cannot undo would be a trap.
    fn blocked_by_lock(&self) -> bool {
        !matches!(
            self,
            Command::SetLayerLocked { .. }
                | Command::AddAsset { .. }
                | Command::RelinkAsset { .. }
                | Command::AddComposition { .. }
                | Command::AddLayer { .. }
        )
    }

    fn layer_id(&self) -> Option<&Id> {
        match self {
            Command::RemoveLayer { layer_id, .. }
            | Command::RenameLayer { layer_id, .. }
            | Command::SetLayerEnabled { layer_id, .. }
            | Command::SetLayerLocked { layer_id, .. }
            | Command::ReorderLayer { layer_id, .. }
            | Command::ShiftLayer { layer_id, .. }
            | Command::TrimLayer { layer_id, .. }
            | Command::SetPropertyBase { layer_id, .. }
            | Command::SetKeyframe { layer_id, .. }
            | Command::RemoveKeyframe { layer_id, .. }
            | Command::SetMatte { layer_id, .. }
            | Command::SetExposureSpans { layer_id, .. }
            | Command::SetMask { layer_id, .. }
            | Command::AddEffect { layer_id, .. }
            | Command::RemoveEffect { layer_id, .. }
            | Command::ReorderEffect { layer_id, .. }
            | Command::SetEffectEnabled { layer_id, .. }
            | Command::SetEffectParameters { layer_id, .. } => Some(layer_id),
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

    /// A document whose contents did not come from the file it will be saved to.
    ///
    /// Recovery is the one way in that [`Document::new`] cannot serve. Dirty is a comparison
    /// against the last successful save, so opening a recovery snapshot with `new` would make
    /// the snapshot its own baseline and the window would report no unsaved work while the
    /// project on disk still held the older state — the exact thing document 07 forbids when it
    /// says an autosave "must not overwrite the last manual save".
    ///
    /// `saved` is the project as the file has it. The difference between the two is the work the
    /// snapshot is carrying, and it is what makes the document dirty until somebody saves it.
    pub fn recovered(project: Project, saved: Project) -> Self {
        Document {
            baseline: saved,
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
    /// Newest first, so the first entry is the label a Redo button should carry. The undo side
    /// is oldest first because a history panel reads downwards; this one has no panel and only
    /// ever answers "what would Redo do next".
    pub fn redo_labels(&self) -> Vec<&str> {
        self.redo.iter().rev().map(|r| r.label.as_str()).collect()
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
        // The last value of each control the drag has hold of, rather than the last value it
        // sent. A drag on three selected layers sends three commands per pointer move, and a
        // list cleared each time would keep whichever arrived last: the other two would follow
        // the pointer on screen and then jump back the moment the record was replayed. Document
        // 26 asks for one record from the value before the drag to the value at release, and
        // for a drag that moved three layers that is one command each.
        match drag.commands.iter_mut().find(|held| command.refines(held)) {
            Some(held) => *held = command,
            None => drag.commands.push(command),
        }
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
        // Every layer the drag moved, each named once. Document 26 has a record name the stable
        // IDs it affected, and for a drag on a multiple selection that is all of them.
        let mut affected: Vec<Id> = Vec::new();
        for held in drag.commands.iter().flat_map(Command::affected) {
            if !affected.contains(&held) {
                affected.push(held);
            }
        }
        self.undo.push(Record {
            command_id: first.command_id(),
            label: match drag.commands.len() {
                1 => first.label(),
                n => format!("{} and {} more", first.label(), n - 1),
            },
            affected,
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

/// The largest side a new composition may have, and the largest number of pixels in one.
///
/// Document 19 line 52: "Composition dimensions and duration are positive and bounded by
/// implementation safety limits." Until B-12d nothing in this build created a composition, so
/// there was no place for that bound to live and no number in it. These are that number, and
/// the reasoning is memory rather than taste: document 21 works in linear-light premultiplied
/// float32 RGBA, which is sixteen bytes a pixel, so a layer buffer at the pixel ceiling is one
/// gibibyte and a frame of a few layers is several. Anything past this is not a shot somebody
/// is finishing on the reference machine; it is a typo in a field, and the point of the limit is
/// that a typo is refused rather than allocated.
///
/// Both are provisional and neither is in document 14. See
/// `verification/B-12d_new_composition_table.md`.
const LARGEST_SIDE: u32 = 16_384;
const LARGEST_PIXELS: u64 = 67_108_864;

/// Ten thousand frames is nearly seven minutes at 24 fps. A composition is a shot.
const LONGEST_COMPOSITION: u32 = 10_000;

/// Document 19 line 52's positivity and safety bounds, and document 19 line 13's unique IDs.
///
/// Checked here rather than in [`Composition::new`] because the model constructor is what the
/// loader and the fixtures use, and a project file that already holds a composition this build
/// would not create is document 28's business, not a panic in a constructor.
fn check_a_new_composition(project: &Project, composition: &Composition) -> Result<(), Diagnostic> {
    if project.composition(&composition.id).is_some() {
        return Err(reject(
            &format!(
                "A composition with the ID {} is already in the project.",
                composition.id
            ),
            "Document 19: stable IDs are unique within a project.",
        ));
    }
    if composition.width == 0 || composition.height == 0 || composition.duration_frames == 0 {
        return Err(reject(
            "A composition needs a width, a height and a length, and one of them was zero.",
            "Document 19 line 52: composition dimensions and duration are positive.",
        ));
    }
    if composition.width > LARGEST_SIDE
        || composition.height > LARGEST_SIDE
        || composition.width as u64 * composition.height as u64 > LARGEST_PIXELS
    {
        // Two limits, and the one that refuses a request is rarely the one the person was
        // watching. 16384x16384 is inside "no side past 16384" and four times over the pixel
        // ceiling, and the owner read the first half of this sentence and asked for it anyway.
        // So when the width is legal the message finishes the arithmetic: it says what height
        // that width allows, which is the number they would otherwise have to work out.
        let at_that_width = if composition.width <= LARGEST_SIDE && composition.width > 0 {
            format!(
                " At {} wide the tallest this build will make is {}.",
                composition.width,
                (LARGEST_PIXELS / composition.width as u64).min(LARGEST_SIDE as u64)
            )
        } else {
            String::new()
        };
        return Err(reject(
            &format!(
                "{}x{} is larger than this build will make: no side past {LARGEST_SIDE} and no \
                 more than {LARGEST_PIXELS} pixels in all.{at_that_width}",
                composition.width, composition.height
            ),
            "Document 19 line 52: bounded by implementation safety limits.",
        ));
    }
    if composition.duration_frames > LONGEST_COMPOSITION {
        return Err(reject(
            &format!(
                "{} frames is longer than this build will make; the limit is \
                 {LONGEST_COMPOSITION}.",
                composition.duration_frames
            ),
            "Document 19 line 52: bounded by implementation safety limits.",
        ));
    }
    Ok(())
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

    if let Command::RelinkAsset { asset } = command {
        let Some(slot) = project.assets.iter_mut().find(|a| a.id == asset.id) else {
            return Err(missing(
                format!("The asset {} is not in this project.", asset.id),
                "A relink names the asset record it replaces; that ID is not present.".to_string(),
            ));
        };
        *slot = (**asset).clone();
        return Ok(());
    }

    if let Command::AddComposition { composition } = command {
        check_a_new_composition(project, composition)?;
        project.compositions.push((**composition).clone());
        return Ok(());
    }

    let comp_id = command
        .composition()
        .expect("only the project-level commands have none")
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
        Command::AddAsset { .. } | Command::RelinkAsset { .. } | Command::AddComposition { .. } => {
            unreachable!("handled above")
        }
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
        Command::ShiftLayer {
            layer_id, in_frame, ..
        } => {
            let layer = layer_mut(project, &comp_id, layer_id)?;
            layer.out_frame += in_frame - layer.in_frame;
            layer.in_frame = *in_frame;
        }
        Command::TrimLayer {
            layer_id,
            in_frame,
            out_frame,
            ..
        } => {
            if in_frame >= out_frame {
                return Err(reject(
                    &format!(
                        "A layer that starts at frame {in_frame} and ends at frame {out_frame} \
                         is not a span."
                    ),
                    "Document 19 invariant: in_frame < out_frame.",
                ));
            }
            let layer = layer_mut(project, &comp_id, layer_id)?;
            // Document 20: local_frame = composition_frame - in_frame + source_offset_frames.
            // The in point moving by d must move the offset by d for the local frame under any
            // surviving composition frame to be unchanged.
            layer.source_offset_frames += in_frame - layer.in_frame;
            layer.in_frame = *in_frame;
            layer.out_frame = *out_frame;
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
            layer_id,
            matte,
            matte_only,
            ..
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
            let matte_ref = matte.clone().map(|layer_id| MatteReference {
                layer_id,
                matte_only: *matte_only,
            });
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
        Command::SetExposureSpans {
            layer_id, spans, ..
        } => {
            // Document 20's rule, checked by the same constructor the renderer and the loader
            // use rather than by a second copy of it written here.
            ExposureMap::new(spans.clone()).map_err(|e| {
                reject(
                    &format!("Those exposures cannot be used: {e}."),
                    "Document 20: exactly one exposure span covers each frame, so spans are \
                     ordered and do not overlap.",
                )
            })?;
            layer_mut(project, &comp_id, layer_id)?.exposure_spans = spans.clone();
        }
        Command::SetMask { layer_id, mask, .. } => {
            // Document 19: a polygon mask is "an ordered list of vec2 vertices, closed by
            // definition", and "self-intersection behavior is unsupported in G1 and must be
            // rejected or normalized only through an explicit command". This build rejects.
            // Normalizing would hand back a different shape from the one that was drawn, and
            // doing that silently inside an edit is how a person loses work without being told.
            //
            // MASK_INVALID_OUTLINE is document 28's identifier for both refusals, added to
            // the catalogue by D-43 rather than reusing COMMAND_INVALID_VALUE. It is an ERROR
            // here because a command that would create one is refused outright.
            if let Some(m) = mask {
                if !m.has_enough_vertices() {
                    return Err(Diagnostic::new(
                        DiagnosticId::MaskInvalidOutline,
                        Severity::Error,
                        format!(
                            "A mask needs at least three points, and this one has {}.",
                            m.vertices.len()
                        ),
                        "Document 19: a polygon mask is a closed ordered list of vertices. Fewer \
                         than three enclose no area, so there is nothing for the mask to keep."
                            .to_string(),
                    )
                    .with_remediation("Add points until the shape closes on an area."));
                }
                if !crate::mask::is_simple(&m.vertices) {
                    return Err(Diagnostic::new(
                        DiagnosticId::MaskInvalidOutline,
                        Severity::Error,
                        "The mask crosses itself, which this build does not draw.".to_string(),
                        "Document 19: self-intersection is unsupported in G1 and must be \
                         rejected, never normalized silently, because a normalized polygon is a \
                         different shape from the one that was drawn."
                            .to_string(),
                    )
                    .with_remediation(
                        "Move the points so that no edge crosses another. A figure-of-eight has \
                         to become two masks, which this build does not have yet.",
                    ));
                }
            }
            layer_mut(project, &comp_id, layer_id)?.mask = mask.clone();
        }
        Command::AddEffect {
            layer_id,
            effect,
            index,
            ..
        } => {
            // Parameters outside document 21's ranges are refused here, not clamped, for the
            // same reason a crossed mask is: accepting a number and rendering a different one
            // is how a person ends up with a picture they did not ask for and no way to tell.
            if !effect.effect.is_valid() {
                return Err(invalid_effect(&effect.effect));
            }
            let layer = layer_mut(project, &comp_id, layer_id)?;
            if layer
                .effects
                .iter()
                .any(|e| e.instance_id == effect.instance_id)
            {
                return Err(Diagnostic::new(
                    DiagnosticId::CommandInvalidValue,
                    Severity::Error,
                    format!(
                        "This layer already has an effect called {}.",
                        effect.instance_id
                    ),
                    "Document 07 requires effect instance IDs to be stable and to identify one \
                     record; two with the same ID would make a later edit ambiguous."
                        .to_string(),
                )
                .with_remediation("Give the new effect its own instance ID."));
            }
            let at = index
                .unwrap_or(layer.effects.len())
                .min(layer.effects.len());
            layer.effects.insert(at, effect.clone());
        }
        Command::RemoveEffect {
            layer_id,
            instance_id,
            ..
        } => {
            let layer = layer_mut(project, &comp_id, layer_id)?;
            let Some(at) = layer
                .effects
                .iter()
                .position(|e| &e.instance_id == instance_id)
            else {
                return Err(missing_effect(layer_id, instance_id));
            };
            layer.effects.remove(at);
        }
        Command::ReorderEffect {
            layer_id,
            instance_id,
            to_index,
            ..
        } => {
            let layer = layer_mut(project, &comp_id, layer_id)?;
            let Some(at) = layer
                .effects
                .iter()
                .position(|e| &e.instance_id == instance_id)
            else {
                return Err(missing_effect(layer_id, instance_id));
            };
            if *to_index >= layer.effects.len() {
                return Err(reject(
                    &format!(
                        "Position {to_index} is past the end of a stack of {} effects.",
                        layer.effects.len()
                    ),
                    "Effect order index out of range.",
                ));
            }
            let moved = layer.effects.remove(at);
            layer.effects.insert(*to_index, moved);
        }
        Command::SetEffectEnabled {
            layer_id,
            instance_id,
            enabled,
            ..
        } => {
            let layer = layer_mut(project, &comp_id, layer_id)?;
            let Some(e) = layer
                .effects
                .iter_mut()
                .find(|e| &e.instance_id == instance_id)
            else {
                return Err(missing_effect(layer_id, instance_id));
            };
            e.enabled = *enabled;
        }
        Command::SetEffectParameters {
            layer_id,
            instance_id,
            effect,
            ..
        } => {
            if !effect.is_valid() {
                return Err(invalid_effect(effect));
            }
            let layer = layer_mut(project, &comp_id, layer_id)?;
            let Some(existing) = layer
                .effects
                .iter_mut()
                .find(|e| &e.instance_id == instance_id)
            else {
                return Err(missing_effect(layer_id, instance_id));
            };
            if existing.type_id() != effect.type_id() {
                return Err(Diagnostic::new(
                    DiagnosticId::CommandInvalidValue,
                    Severity::Error,
                    format!(
                        "Effect {instance_id} is a {}, and these are settings for a {}.",
                        existing.type_id(),
                        effect.type_id()
                    ),
                    "Changing an effect's type in place would leave the instance ID pointing at \
                     something else, which document 07's stable references do not allow."
                        .to_string(),
                )
                .with_remediation("Remove the effect and add the one you want."));
            }
            existing.effect = effect.clone();
        }
    }
    Ok(())
}

/// Document 28's `EFFECT_PARAMETER_INVALID`, at the command boundary where it is an ERROR
/// because the command is refused outright rather than bypassed.
fn invalid_effect(effect: &crate::effects::Effect) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::EffectParameterInvalid,
        Severity::Error,
        effect.why_invalid(),
        "Document 21 states the range for each G1 effect parameter. A value outside it is \
         refused rather than clamped, so that what the project says and what the picture shows \
         never disagree."
            .to_string(),
    )
    .with_remediation("Choose a value inside the range.")
}

fn missing_effect(layer_id: &Id, instance_id: &Id) -> Diagnostic {
    missing(
        format!("There is no effect called {instance_id} on this layer."),
        format!("Layer {layer_id} has no effect instance {instance_id}."),
    )
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
