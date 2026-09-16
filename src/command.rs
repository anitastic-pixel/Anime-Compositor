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
    Asset, BlendMode, Composition, Id, Interp, Keyframe, Layer, MatteReference, Project, Prop,
    Value,
};
use crate::time::{ExposureMap, ExposureSpan, FrameRate};

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
    /// W-26: a composition deleted from the project panel. The last one in a project is refused,
    /// so the window always has a composition to show.
    RemoveComposition {
        composition: Id,
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
    /// W-24: After Effects' label colour, 0 for none and 1 to 8.
    SetLayerLabel {
        composition: Id,
        layer_id: Id,
        label: u8,
    },
    /// W-26: After Effects' shy switch. It changes nothing in the picture; the timeline leaves a
    /// shy layer out while its Hide shy layers switch is on.
    SetLayerShy {
        composition: Id,
        layer_id: Id,
        value: bool,
    },
    /// Document 24's `timeline.set_work_start` and `set_work_end`, W-24. Both ends at once, so a
    /// drag that moves one end replaces its earlier reading whole.
    SetWorkArea {
        composition: Id,
        start_frame: i32,
        end_frame_exclusive: i32,
    },
    /// W-24: the composition's markers, the whole list, so adding, moving, naming and removing
    /// one are all this command and a dragged marker is one entry to undo.
    SetMarkers {
        composition: Id,
        markers: Vec<crate::model::Marker>,
    },
    /// W-25: a layer's blend mode, from the inspector or the layer's right-click menu.
    SetBlendMode {
        composition: Id,
        layer_id: Id,
        mode: BlendMode,
    },
    /// W-25: After Effects' Composition Settings, Ctrl+K. A work area or a marker the new length
    /// leaves outside is cut back or dropped in the same entry to undo.
    SetCompositionSettings {
        composition: Id,
        name: String,
        width: u32,
        height: u32,
        frame_rate: FrameRate,
        duration_frames: u32,
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
    /// this every layer began where it was added. Its transform keyframes move by the same
    /// number of frames (owner decision, 2026-09-13); a trim leaves them where they are.
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
        /// D-53's path handles, `[in_x, in_y, out_x, out_y]`. Refused on anything but
        /// `position`. A caller replacing a key it means to keep the path of passes the
        /// existing key's handles, exactly as it passes the existing `interp`.
        spatial: Option<[f64; 4]>,
    },
    RemoveKeyframe {
        composition: Id,
        layer_id: Id,
        prop: Prop,
        frame: i32,
    },
    /// Document 24's `keyframe.move`. W-11: a key taken hold of on the timeline and put down on
    /// another frame.
    ///
    /// One command rather than a remove followed by a set, because those are two history entries
    /// for one gesture and the second can succeed where the first did not. The key keeps its
    /// value, its interpolation mode and its path handles: this moves when it happens, not
    /// what it does.
    MoveKeyframe {
        composition: Id,
        layer_id: Id,
        prop: Prop,
        from_frame: i32,
        to_frame: i32,
    },
    SetMatte {
        composition: Id,
        layer_id: Id,
        matte: Option<Id>,
        /// D-42: whether the layer named in `matte` stops being drawn in its own right.
        /// Ignored when `matte` is `None`, since there is then no layer to keep out of the stack.
        matte_only: bool,
    },
    /// D-57's parenting: the layer rides on `parent`, or on nothing when that is `None`.
    SetParent {
        composition: Id,
        layer_id: Id,
        parent: Option<Id>,
        /// The composition frame document 21's keep-place conversion is worked at.
        frame: i32,
        /// Whether the layer keeps where it is on screen, which is what a person choosing a
        /// parent means. False only where the layer is already in the parent's space and its
        /// values must not be touched: a copy pasted beside the layer it was copied from.
        keep_place: bool,
    },
    /// D-58's depth: which plane a layer sits on, in pixels from its parent's plane.
    ///
    /// Its own command rather than a sixth `Prop`, because `Prop` is the five the layer
    /// transform has and feeds `Transform::get`; and document 24 names it separately for
    /// the keep-place conversion it carries when a layer gains or loses a parent.
    SetDepth {
        composition: Id,
        layer_id: Id,
        value: f64,
    },
    /// D-58's camera, which belongs to the composition and not to a layer, so names none.
    ///
    /// A composition with no camera of its own gets the default lens written into it by
    /// the first of these, because a change to the camera a shot is drawn through has to
    /// be a change to something.
    SetCameraProperty {
        composition: Id,
        prop: crate::model::CameraProp,
        value: Value,
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
            Command::RemoveComposition { .. } => "REMOVE_COMPOSITION",
            Command::AddLayer { .. } => "ADD_LAYER",
            Command::RemoveLayer { .. } => "REMOVE_LAYER",
            Command::RenameLayer { .. } => "RENAME_LAYER",
            Command::SetLayerEnabled { .. } => "SET_LAYER_ENABLED",
            Command::SetLayerLocked { .. } => "SET_LAYER_LOCKED",
            Command::SetLayerLabel { .. } => "SET_LAYER_LABEL",
            Command::SetLayerShy { .. } => "SET_LAYER_SHY",
            Command::SetWorkArea { .. } => "SET_WORK_AREA",
            Command::SetMarkers { .. } => "SET_MARKERS",
            Command::SetBlendMode { .. } => "SET_BLEND_MODE",
            Command::SetCompositionSettings { .. } => "SET_COMPOSITION_SETTINGS",
            Command::ReorderLayer { .. } => "REORDER_LAYER",
            Command::ShiftLayer { .. } => "SHIFT_LAYER",
            Command::TrimLayer { .. } => "TRIM_LAYER",
            Command::SetPropertyBase { .. } => "SET_PROPERTY",
            Command::SetKeyframe { .. } => "SET_KEYFRAME",
            Command::RemoveKeyframe { .. } => "REMOVE_KEYFRAME",
            Command::MoveKeyframe { .. } => "MOVE_KEYFRAME",
            Command::SetMatte { .. } => "SET_MATTE",
            Command::SetParent { .. } => "SET_PARENT",
            Command::SetDepth { .. } => "SET_DEPTH",
            Command::SetCameraProperty { .. } => "SET_CAMERA_PROPERTY",
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
            Command::RemoveComposition { composition } => {
                format!("Delete composition {composition}")
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
            Command::SetLayerShy { value, .. } => match value {
                true => "Make the layer shy".to_string(),
                false => "Make the layer not shy".to_string(),
            },
            Command::SetLayerLabel { label, .. } => match label {
                0 => "Clear the layer's label".to_string(),
                n => format!("Set the layer's label to colour {n}"),
            },
            Command::SetWorkArea {
                start_frame,
                end_frame_exclusive,
                ..
            } => format!(
                "Set the work area to frames {start_frame} to {}",
                end_frame_exclusive - 1
            ),
            Command::SetMarkers { markers, .. } => match markers.len() {
                0 => "Clear the markers".to_string(),
                1 => "Set one marker".to_string(),
                n => format!("Set {n} markers"),
            },
            Command::SetBlendMode { mode, .. } => {
                format!("Set the blend mode to {}", mode.as_str())
            }
            Command::SetCompositionSettings { name, .. } => {
                format!("Change the settings of {name}")
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
            Command::MoveKeyframe {
                prop,
                from_frame,
                to_frame,
                ..
            } => format!("Move {prop} keyframe from frame {from_frame} to frame {to_frame}"),
            Command::SetMatte {
                matte, matte_only, ..
            } => match matte {
                Some(id) if *matte_only => format!("Set matte to {id}, matte only"),
                Some(id) => format!("Set matte to {id}"),
                None => "Clear matte".to_string(),
            },
            Command::SetParent { parent, .. } => match parent {
                Some(id) => format!("Set parent to {id}"),
                None => "Clear parent".to_string(),
            },
            Command::SetDepth { value, .. } => format!("Set depth to {value}"),
            Command::SetCameraProperty { prop, .. } => {
                format!("Set the camera's {}", prop.as_str())
            }
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
            | Command::AddComposition { .. }
            | Command::RemoveComposition { .. } => None,
            Command::AddLayer { composition, .. }
            | Command::RemoveLayer { composition, .. }
            | Command::RenameLayer { composition, .. }
            | Command::SetLayerEnabled { composition, .. }
            | Command::SetLayerLocked { composition, .. }
            | Command::SetLayerLabel { composition, .. }
            | Command::SetLayerShy { composition, .. }
            | Command::SetWorkArea { composition, .. }
            | Command::SetMarkers { composition, .. }
            | Command::SetBlendMode { composition, .. }
            | Command::SetCompositionSettings { composition, .. }
            | Command::ReorderLayer { composition, .. }
            | Command::ShiftLayer { composition, .. }
            | Command::TrimLayer { composition, .. }
            | Command::SetPropertyBase { composition, .. }
            | Command::SetKeyframe { composition, .. }
            | Command::RemoveKeyframe { composition, .. }
            | Command::MoveKeyframe { composition, .. }
            | Command::SetMatte { composition, .. }
            | Command::SetParent { composition, .. }
            | Command::SetDepth { composition, .. }
            | Command::SetCameraProperty { composition, .. }
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
            Command::RemoveComposition { composition } => ids.push(composition.clone()),
            Command::AddLayer { layer, .. } => ids.push(layer.id.clone()),
            Command::SetWorkArea { .. }
            | Command::SetMarkers { .. }
            | Command::SetCompositionSettings { .. } => {}
            Command::RemoveLayer { layer_id, .. }
            | Command::SetLayerLabel { layer_id, .. }
            | Command::SetBlendMode { layer_id, .. }
            | Command::SetLayerShy { layer_id, .. }
            | Command::RenameLayer { layer_id, .. }
            | Command::SetLayerEnabled { layer_id, .. }
            | Command::SetLayerLocked { layer_id, .. }
            | Command::ReorderLayer { layer_id, .. }
            | Command::ShiftLayer { layer_id, .. }
            | Command::TrimLayer { layer_id, .. }
            | Command::SetPropertyBase { layer_id, .. }
            | Command::SetKeyframe { layer_id, .. }
            | Command::RemoveKeyframe { layer_id, .. }
            | Command::MoveKeyframe { layer_id, .. }
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
            Command::SetParent {
                layer_id, parent, ..
            } => {
                ids.push(layer_id.clone());
                ids.extend(parent.clone());
            }
            Command::SetDepth { layer_id, .. } => ids.push(layer_id.clone()),
            // The camera is the composition's, and the composition is already in the list.
            Command::SetCameraProperty { .. } => {}
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
            // D-58: the camera's place, its depth and its zoom are three controls and not
            // one, so a drag on the zoom does not swallow the track that came before it.
            (
                Command::SetCameraProperty { prop: mine, .. },
                Command::SetCameraProperty { prop: theirs, .. },
            ) if mine != theirs => return false,
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
                | Command::SetLayerLabel { .. }
                | Command::SetLayerShy { .. }
                | Command::RemoveComposition { .. }
                | Command::AddAsset { .. }
                | Command::RelinkAsset { .. }
                | Command::AddComposition { .. }
                | Command::AddLayer { .. }
                // The camera belongs to the composition; a locked layer has no say in it.
                | Command::SetCameraProperty { .. }
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
            | Command::MoveKeyframe { layer_id, .. }
            | Command::SetLayerLabel { layer_id, .. }
            | Command::SetBlendMode { layer_id, .. }
            | Command::SetLayerShy { layer_id, .. }
            | Command::SetMatte { layer_id, .. }
            | Command::SetParent { layer_id, .. }
            | Command::SetDepth { layer_id, .. }
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
    check_composition_size(composition)
}

/// The size, rate and length rules a composition is held to, new or changed (W-25).
fn check_composition_size(composition: &Composition) -> Result<(), Diagnostic> {
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

    if let Command::RemoveComposition { composition } = command {
        if project.composition(composition).is_none() {
            return Err(missing(
                format!("The composition {composition} is not in this project."),
                "A delete names the composition it removes; that ID is not present.".to_string(),
            ));
        }
        if project.compositions.len() == 1 {
            return Err(reject(
                "This is the only composition in the project, so it stays. Make another one \
                 first if this one should go.",
                "W-26: a project keeps at least one composition for the window to show.",
            ));
        }
        project.compositions.retain(|c| &c.id != composition);
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
        Command::AddAsset { .. }
        | Command::RelinkAsset { .. }
        | Command::AddComposition { .. }
        | Command::RemoveComposition { .. } => {
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
        Command::SetLayerShy {
            layer_id, value, ..
        } => {
            layer_mut(project, &comp_id, layer_id)?.shy = *value;
        }
        Command::SetLayerLabel {
            layer_id, label, ..
        } => {
            if *label > 8 {
                return Err(reject(
                    &format!("There is no label colour {label}; the colours are 1 to 8."),
                    "W-24: a label is 0 for none or one of eight colours.",
                ));
            }
            layer_mut(project, &comp_id, layer_id)?.label = *label;
        }
        Command::SetWorkArea {
            start_frame,
            end_frame_exclusive,
            ..
        } => {
            let comp = comp_mut(project, &comp_id)?;
            let (first, past) = (
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            if *start_frame < first
                || *end_frame_exclusive > past
                || start_frame >= end_frame_exclusive
            {
                return Err(reject(
                    &format!(
                        "The work area has to be at least one frame, inside frames {first} to {}.",
                        past - 1
                    ),
                    "Document 19: the work area lies within the composition.",
                ));
            }
            comp.work_area = Some((*start_frame, *end_frame_exclusive));
        }
        Command::SetMarkers { markers, .. } => {
            let comp = comp_mut(project, &comp_id)?;
            let (first, past) = (
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            if let Some(m) = markers.iter().find(|m| m.frame < first || m.frame >= past) {
                return Err(reject(
                    &format!(
                        "A marker on frame {} is outside the composition's frames {first} to {}.",
                        m.frame,
                        past - 1
                    ),
                    "W-24: a marker is on a frame of its composition.",
                ));
            }
            let mut markers = markers.clone();
            markers.sort_by_key(|m| m.frame);
            comp.markers = markers;
        }
        Command::SetBlendMode { layer_id, mode, .. } => {
            layer_mut(project, &comp_id, layer_id)?.blend_mode = *mode;
        }
        Command::SetCompositionSettings {
            name,
            width,
            height,
            frame_rate,
            duration_frames,
            ..
        } => {
            let comp = comp_mut(project, &comp_id)?;
            check_composition_size(&Composition::new(
                comp.id.clone(),
                name.clone(),
                *width,
                *height,
                *frame_rate,
                comp.start_frame,
                *duration_frames,
            ))?;
            let past = comp.start_frame + *duration_frames as i32;
            comp.name = name.clone();
            comp.width = *width;
            comp.height = *height;
            comp.frame_rate = *frame_rate;
            comp.duration_frames = *duration_frames;
            comp.work_area = comp
                .work_area
                .map(|(start, end)| (start, end.min(past)))
                .filter(|(start, end)| start < end);
            comp.markers.retain(|m| m.frame < past);
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
            let by = in_frame - layer.in_frame;
            layer.out_frame += by;
            layer.in_frame = *in_frame;
            // The owner's decision of 2026-09-13: the keys travel with the bar, as in After
            // Effects. Every key moves by the same amount, so no two can land on one frame.
            for prop in [
                Prop::Anchor,
                Prop::Position,
                Prop::Scale,
                Prop::Rotation,
                Prop::Opacity,
            ] {
                layer.transform.get_mut(prop).shift_keyframes(by);
            }
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
            spatial,
            ..
        } => {
            let value = check_value(*prop, *value)?;
            if spatial.is_some() && *prop != Prop::Position {
                return Err(reject(
                    &format!("{prop} cannot carry path handles."),
                    "Document 19: the motion path belongs to position keyframes only.",
                ));
            }
            if spatial.is_some_and(|s| !s.iter().all(|n| n.is_finite())) {
                return Err(reject(
                    "A path handle must be a finite offset.",
                    "Document 19: spatial is four offsets in composition pixels.",
                ));
            }
            layer_mut(project, &comp_id, layer_id)?
                .transform
                .get_mut(*prop)
                .set_keyframe(Keyframe {
                    frame: *frame,
                    value,
                    interp: *interp,
                    spatial: *spatial,
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
        Command::MoveKeyframe {
            layer_id,
            prop,
            from_frame,
            to_frame,
            ..
        } => {
            if from_frame == to_frame {
                return Err(reject(
                    "That keyframe is already on that frame.",
                    "A move from a frame to itself is not an edit and must not enter history.",
                ));
            }
            let property = layer_mut(project, &comp_id, layer_id)?
                .transform
                .get_mut(*prop);
            // Refused rather than overwritten. Document 19 calls two keyframes at one frame
            // invalid, so a move onto an occupied frame has to lose one of them, and losing a key
            // the artist can no longer see the mark of is a worse answer than not moving.
            if property.keyframe_at(*to_frame).is_some() {
                return Err(reject(
                    &format!("There is already a {prop} keyframe on frame {to_frame}."),
                    "Document 19: two keyframes at one frame are invalid.",
                ));
            }
            let Some(key) = property.remove_keyframe(*from_frame) else {
                return Err(missing(
                    format!("There is no {prop} keyframe at frame {from_frame} to move."),
                    format!("Layer {layer_id} has no {prop} keyframe at frame {from_frame}."),
                ));
            };
            property.set_keyframe(Keyframe {
                frame: *to_frame,
                ..key
            });
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
        Command::SetParent {
            layer_id,
            parent,
            frame,
            keep_place: keeping,
            ..
        } => {
            let comp = project.composition(&comp_id).expect("checked above");
            if let Some(target) = parent {
                if comp.layer(target).is_none() {
                    return Err(Diagnostic::new(
                        DiagnosticId::ParentReferenceMissing,
                        Severity::Error,
                        format!(
                            "The layer chosen as a parent, {target}, is not in this composition."
                        ),
                        "D-57: a parent must be a layer in the same composition.".to_string(),
                    ));
                }
            }
            // Worked before the write, because it reads the chain the layer is in now.
            let conversion = match keeping {
                true => Some(keep_place(comp, layer_id, parent.as_ref(), *frame)?),
                false => None,
            };
            let layer = layer_mut(project, &comp_id, layer_id)?;
            layer.parent = parent.clone();
            if let Some(by) = &conversion {
                apply_keep_place(layer, by);
            }
            // Checked after the write and rolled back by the caller's working clone if bad, the
            // way the matte's cycle is. A layer named as its own parent is caught here too.
            let comp = project.composition(&comp_id).expect("checked above");
            if comp.parent_cycle_from(layer_id) {
                return Err(Diagnostic::new(
                    DiagnosticId::ParentCycle,
                    Severity::Error,
                    "That parent would make two layers ride on each other.".to_string(),
                    format!(
                        "Parenting {layer_id} to {parent:?} closes a loop in the parent graph."
                    ),
                )
                .with_remediation("Choose a layer that is not already riding on this one."));
            }
        }
        Command::SetDepth {
            layer_id, value, ..
        } => {
            if !value.is_finite() {
                return Err(reject(
                    &format!("A depth cannot be set to {value}."),
                    "D-58: a depth is a number of pixels from the parent's plane.",
                ));
            }
            let layer = layer_mut(project, &comp_id, layer_id)?;
            match &mut layer.depth {
                Some(depth) => depth.set_base(Value::Scalar(*value)),
                none => {
                    *none = Some(crate::model::Property::constant(Value::Scalar(*value)))
                }
            }
        }
        Command::SetCameraProperty { prop, value, .. } => {
            let value = check_camera_value(*prop, *value)?;
            let comp = comp_mut(project, &comp_id)?;
            let (width, height) = (comp.width, comp.height);
            comp.camera
                .get_or_insert_with(|| crate::model::Camera::default_for(width, height))
                .get_mut(*prop)
                .set_base(value);
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

/// D-58's camera, checked as `check_value` checks a layer's property, plus the one rule a
/// layer has no equivalent of: a zoom of nought or less has nothing in front of it.
fn check_camera_value(
    prop: crate::model::CameraProp,
    value: Value,
) -> Result<Value, Diagnostic> {
    if value.kind() != prop.kind() {
        return Err(reject(
            &format!(
                "The camera's {} takes a {} value, not a {}.",
                prop.as_str(),
                prop.kind(),
                value.kind()
            ),
            "D-58: the camera's place is a vec2; its depth and its zoom are scalars.",
        ));
    }
    if !finite(value) {
        return Err(reject(
            &format!("The camera's {} cannot be set to {value}.", prop.as_str()),
            "Property values must be finite numbers.",
        ));
    }
    if prop == crate::model::CameraProp::Zoom && !matches!(value, Value::Scalar(z) if z > 0.0)
    {
        return Err(reject(
            "A camera's zoom must be more than nought.",
            "D-58: a zoom of nought or less puts nothing in front of the camera to draw.",
        ));
    }
    Ok(value)
}

fn finite(value: Value) -> bool {
    match value {
        Value::Scalar(v) => v.is_finite(),
        Value::Vec2(x, y) => x.is_finite() && y.is_finite(),
    }
}

/// D-57's keep-place conversion, worked at one frame: what a layer must hold under its new
/// parent for every point of it to stay where it is on screen.
///
/// Document 21: with `W` the new parent's `M_world`, the new position is `W^-1` applied to the
/// position the layer has in the chain it is in now; the new rotation is its rotation less the
/// rotations along that chain; the new scale is its scale divided, component by component, by
/// the product of the chain's scales. The anchor does not move.
struct KeepPlace {
    /// `W_new^-1 * W_old`, which carries a point from where it is now into the new parent's
    /// space. Both keyframe values and base values go through it.
    map: crate::render::Affine,
    rotation: f64,
    scale: (f64, f64),
    /// D-58: what to add to the layer's own depth so that its distance from the camera
    /// does not change. A depth rides the chain by addition, so this conversion is always
    /// exact whatever `exact` says about the position, scale and rotation one.
    depth: f64,
    /// Document 21's exactness rule: true when every chain scales equally in x and y, or when
    /// neither the layer nor any layer in either chain is turned. False means the exact answer
    /// is a skew this transform cannot hold -- the anchor lands where it was and the rest may
    /// shift -- which the window says at the moment it happens.
    exact: bool,
}

fn keep_place(
    comp: &crate::model::Composition,
    layer_id: &Id,
    parent: Option<&Id>,
    frame: i32,
) -> Result<KeepPlace, Diagnostic> {
    let old = crate::compose::parent_chain_at(comp, layer_id, frame);
    let new = match parent {
        Some(id) => crate::compose::world_at(comp, id, frame),
        None => crate::compose::Chain::IDENTITY,
    };
    // Document 21: "A chain with a zero scale component at `f` has no inverse, and setting the
    // parent is refused." A layer scaled to nothing has no space to put the child in.
    let Some(inverse) = new.matrix.invert() else {
        return Err(Diagnostic::new(
            DiagnosticId::CommandInvalidValue,
            Severity::Error,
            "That layer cannot be a parent, because it is scaled to nothing.".to_string(),
            format!(
                "The parent chain has a zero scale component at frame {frame}, so it has no \
                 inverse and document 21's keep-place conversion is undefined."
            ),
        )
        .with_remediation("Give the parent a scale that is not zero, then set the parent again."));
    };
    let turned = comp
        .layer(layer_id)
        .and_then(|l| l.transform.rotation.value_at(frame).as_scalar())
        .is_some_and(|r| r != 0.0);
    // D-58: the chain's depth, which the layer's own is added to. Leaving the old chain
    // takes that chain's depth off the layer, and joining the new one puts the new
    // chain's on, so the layer stays the distance from the camera it was.
    let chain_depth = |id: Option<&Id>| match id {
        Some(id) => crate::compose::world_depth(comp, id, frame),
        None => 0.0,
    };
    let held = comp.layer(layer_id).and_then(|l| l.parent.as_ref());
    Ok(KeepPlace {
        map: old.matrix.then(inverse),
        rotation: old.rotation - new.rotation,
        scale: (old.scale.0 / new.scale.0, old.scale.1 / new.scale.1),
        depth: chain_depth(held) - chain_depth(parent),
        exact: (old.uniform && new.uniform) || (!turned && old.unrotated && new.unrotated),
    })
}

/// Write the conversion over position, scale and rotation, base value and every keyframe alike.
fn apply_keep_place(layer: &mut crate::model::Layer, by: &KeepPlace) {
    use crate::model::{Keyframe, Property, Value};

    let origin = by.map.apply(0.0, 0.0);
    // A spatial handle is an offset from its own key, so only the map's linear part moves it:
    // the map applied to the offset, less where the map sends the origin.
    let handle = |h: [f64; 4]| {
        let mut out = h;
        for pair in 0..2 {
            let (x, y) = by.map.apply(h[pair * 2], h[pair * 2 + 1]);
            out[pair * 2] = x - origin.0;
            out[pair * 2 + 1] = y - origin.1;
        }
        out
    };
    fn convert(
        prop: &mut Property,
        value: impl Fn(Value) -> Value,
        handle: impl Fn([f64; 4]) -> [f64; 4],
    ) {
        prop.set_base(value(prop.base()));
        for key in prop.keyframes().to_vec() {
            prop.set_keyframe(Keyframe {
                value: value(key.value),
                spatial: key.spatial.map(&handle),
                ..key
            });
        }
    }

    convert(
        &mut layer.transform.position,
        |v| match v.as_vec2() {
            Some((x, y)) => {
                let (nx, ny) = by.map.apply(x, y);
                Value::Vec2(nx, ny)
            }
            None => v,
        },
        &handle,
    );
    convert(
        &mut layer.transform.scale,
        |v| match v.as_vec2() {
            Some((x, y)) => Value::Vec2(x * by.scale.0, y * by.scale.1),
            None => v,
        },
        &handle,
    );
    convert(
        &mut layer.transform.rotation,
        |v| match v.as_scalar() {
            Some(r) => Value::Scalar(r + by.rotation),
            None => v,
        },
        &handle,
    );
    // D-58. Guarded, so that a layer with no depth in a project with no camera does not
    // gain one by being parented: nothing to keep means nothing to write.
    if by.depth != 0.0 {
        let depth = layer
            .depth
            .get_or_insert_with(|| Property::constant(Value::Scalar(0.0)));
        convert(
            depth,
            |v| match v.as_scalar() {
                Some(d) => Value::Scalar(d + by.depth),
                None => v,
            },
            &handle,
        );
    }
}

/// Whether parenting `layer_id` to `parent` at `frame` can keep every point of the layer where
/// it is, or only its anchor. Document 21's exactness rule, asked before the command is sent so
/// that the window can say which of the two happened.
pub fn parent_keep_place_is_exact(
    comp: &crate::model::Composition,
    layer_id: &Id,
    parent: Option<&Id>,
    frame: i32,
) -> bool {
    keep_place(comp, layer_id, parent, frame).is_ok_and(|k| k.exact)
}
