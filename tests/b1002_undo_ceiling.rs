//! B-1002, D-1137: a ceiling on undo memory.
//!
//! Undo steps are counted by the layers each one alone holds (B-999 made steps share every layer
//! an edit did not change). Past the ceiling the oldest steps are let go, and the document says
//! how many. What must not change: every step that is kept undoes and redoes to the project equal
//! by value, every field, and dirty is still a comparison with the last save (document 26).
//!
//! The expected projects are copies taken as the edits are made, not anything the ceiling works
//! out, so a step that came back wrong cannot agree with itself.
//!
//! `cargo test --release --test b1002_undo_ceiling`

use anime_compositor::command::{Command, Document, Target, DEFAULT_UNDO_CEILING};
use anime_compositor::model::{Id, Project, Prop, Value};
use anime_compositor::persist;

const COMP: &str = "comp-reference-shot";

fn shot() -> Project {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(repo.join("verification/B-08a_project.json")).unwrap();
    persist::load_str(&text).expect("the reference shot opens").document.project().clone()
}

fn layers(p: &Project) -> Vec<Id> {
    p.compositions[0].layers_in_order().map(|l| l.id.clone()).collect()
}

/// Edit `n`: a rotation on one of the shot's layers, a different layer each time.
fn edit(p: &Project, n: usize) -> Command {
    let ids = layers(p);
    Command::SetPropertyBase {
        composition: Id::new(COMP),
        target: Target::Layer(ids[n % ids.len()].clone()),
        prop: Prop::Rotation,
        value: Value::Scalar(n as f64 + 0.5),
    }
}

/// `edits` edits, returning the document and the project as it stood before each edit and after
/// the last (`states[k]` is the project after `k` edits).
fn run(ceiling: Option<usize>, edits: usize) -> (Document, Vec<Project>) {
    let mut doc = Document::new(shot());
    if let Some(c) = ceiling {
        doc.set_undo_ceiling(c);
    }
    let mut states = vec![doc.project().clone()];
    for n in 0..edits {
        let c = edit(doc.project(), n);
        doc.apply(c).unwrap();
        states.push(doc.project().clone());
    }
    (doc, states)
}

/// Undo every kept step, checking each against its copy, then redo every one.
fn walk(doc: &mut Document, states: &[Project]) {
    let last = states.len() - 1;
    let kept = doc.undo_depth();
    for k in 1..=kept {
        assert!(doc.undo().is_some(), "undo {k} of {kept}");
        assert!(*doc.project() == states[last - k], "undo {k} gives back the project after edit {}", last - k);
    }
    assert!(doc.undo().is_none(), "no step older than the kept ones");
    for k in (0..kept).rev() {
        assert!(doc.redo().is_some(), "redo back to edit {}", last - k);
        assert!(*doc.project() == states[last - k], "redo gives back the project after edit {}", last - k);
    }
    assert!(doc.redo().is_none());
}

#[test]
fn the_default_ceiling_is_512_mb_and_keeps_every_step_of_a_short_session() {
    assert_eq!(DEFAULT_UNDO_CEILING, 512_000_000);
    let (mut doc, states) = run(None, 30);
    assert_eq!(doc.undo_ceiling(), DEFAULT_UNDO_CEILING);
    assert_eq!(doc.undo_depth(), 30);
    assert_eq!(doc.undo_dropped(), 0);
    assert!(doc.undo_held_bytes() > 0, "the steps are counted as holding something");
    walk(&mut doc, &states);
    // Back to the project as opened: not dirty, as before.
    for _ in 0..30 {
        doc.undo();
    }
    assert!(*doc.project() == states[0]);
    assert!(!doc.is_dirty());
}

#[test]
fn past_the_ceiling_the_oldest_steps_go_and_the_kept_ones_still_undo_and_redo_exactly() {
    // A ceiling that holds about five steps of this shot, whatever one step is counted at.
    let (probe, _) = run(None, 10);
    let step = probe.undo_held_bytes() / 10;
    let (mut doc, states) = run(Some(step * 5), 30);
    assert!(doc.undo_depth() >= 2 && doc.undo_depth() < 30, "kept {}", doc.undo_depth());
    assert_eq!(doc.undo_depth() + doc.undo_dropped(), 30, "every step is kept or counted as let go");
    assert!(doc.undo_held_bytes() <= step * 5);
    walk(&mut doc, &states);
    // The saved state's steps are gone, so undo cannot reach it: still dirty, and redo is intact.
    while doc.undo().is_some() {}
    assert!(doc.is_dirty());
}

#[test]
fn at_least_the_newest_step_is_kept_however_low_the_ceiling() {
    let (mut doc, states) = run(Some(0), 12);
    assert_eq!(doc.undo_depth(), 1);
    assert_eq!(doc.undo_dropped(), 11);
    walk(&mut doc, &states);
}

#[test]
fn lowering_the_ceiling_lets_go_at_once_and_raising_it_brings_nothing_back() {
    let (mut doc, states) = run(None, 20);
    doc.set_undo_ceiling(0);
    assert_eq!(doc.undo_depth(), 1);
    assert_eq!(doc.undo_dropped(), 19);
    doc.set_undo_ceiling(DEFAULT_UNDO_CEILING);
    assert_eq!(doc.undo_depth(), 1);
    assert_eq!(doc.undo_dropped(), 19);
    walk(&mut doc, &states);
}

#[test]
fn a_drag_is_one_step_under_the_ceiling_too() {
    let (mut doc, mut states) = run(Some(0), 3);
    doc.begin_drag().unwrap();
    for n in 0..5 {
        let c = edit(doc.project(), 0);
        let c = match c {
            Command::SetPropertyBase { composition, target, prop, .. } => Command::SetPropertyBase {
                composition,
                target,
                prop,
                value: Value::Scalar(100.0 + n as f64),
            },
            other => other,
        };
        doc.update_drag(c).unwrap();
    }
    doc.end_drag().expect("the drag moved something");
    states.push(doc.project().clone());
    assert_eq!(doc.undo_depth(), 1);
    assert_eq!(doc.undo_dropped(), 3);
    walk(&mut doc, &states);
}

#[test]
fn undo_then_a_new_edit_keeps_dirty_as_a_comparison_with_the_save() {
    let (mut doc, _) = run(Some(0), 5);
    doc.mark_saved();
    let saved = doc.project().clone();
    let c = edit(doc.project(), 7);
    doc.apply(c).unwrap();
    assert!(doc.is_dirty());
    assert_eq!(doc.undo_depth(), 1);
    doc.undo();
    assert!(*doc.project() == saved);
    assert!(!doc.is_dirty(), "undone to the saved project: clean, as document 26 asks");
    doc.redo();
    assert!(doc.is_dirty());
}

#[test]
fn a_step_is_counted_by_the_layers_it_alone_holds_not_the_whole_project() {
    // Every edit here touches one layer of four; a step that counted the whole project would be
    // the whole project's size.
    let (doc, states) = run(None, 8);
    let whole = format!("{:?}", states[0]).len();
    let per_step = doc.undo_held_bytes() / 8;
    assert!(per_step < whole, "one step {per_step} bytes, all layers {whole}");
}
