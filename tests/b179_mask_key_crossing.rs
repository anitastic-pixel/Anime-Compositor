//! B-179: D-294, a mask key whose outline crosses itself is refused, as the path is.
//!
//! P-26 drew a wedge that Draw refused for crossing, then set the same wedge as a key, which was
//! taken and drew the layer unmasked on those frames. The square and the bowtie are worked by
//! hand: the bowtie's edges (4,0)-(0,4) and (4,4)-(0,0) cross at (2,2).

use anime_compositor::command::{Command, Document};
use anime_compositor::mask::{Mask, MaskKey};
use anime_compositor::model::{Asset, Composition, Id, Interp, Layer, Project};
use anime_compositor::time::FrameRate;

fn document() -> Document {
    let mut project = Project::new(Id::new("p"));
    let rate = FrameRate::new(24, 1).expect("24 fps");
    project.compositions.push(Composition::new(Id::new("comp"), "comp", 8, 8, rate, 0, 8));
    let mut doc = Document::new(project);
    let asset = Asset::still(Id::new("asset-a"), "a", "a.png".to_string());
    let layer = Layer::new(Id::new("a"), "a", asset.id.clone(), 0, 8);
    doc.apply_all(vec![
        Command::AddAsset { asset },
        Command::AddLayer { composition: Id::new("comp"), layer: Box::new(layer), index: 0 },
    ])
    .expect("seeding a layer is a valid command");
    doc
}

fn keyed(second: Vec<(f64, f64)>) -> Command {
    let square = Mask::polygon(vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
    let mut mask = square.clone();
    let other = Mask::polygon(second);
    mask.keys = vec![
        MaskKey { frame: 0, points: square.points.clone(), interp: Interp::Linear },
        MaskKey { frame: 4, points: other.points, interp: Interp::Linear },
    ];
    Command::SetMasks { composition: Id::new("comp"), layer_id: Id::new("a"), masks: vec![mask] }
}

#[test]
fn a_crossing_key_is_refused_and_a_plain_one_is_taken() {
    let mut doc = document();
    let refused = doc
        .apply(keyed(vec![(0.0, 0.0), (4.0, 0.0), (0.0, 4.0), (4.0, 4.0)]))
        .err()
        .expect("a crossing key is refused");
    assert_eq!(
        refused.message,
        "The mask crosses itself at its key on frame 4, which this build does not draw."
    );
    let layer = |doc: &Document| doc.project().composition(&Id::new("comp")).unwrap().layer(&Id::new("a")).unwrap().masks.len();
    assert_eq!(layer(&doc), 0, "a refused command leaves no mask");
    doc.apply(keyed(vec![(1.0, 1.0), (5.0, 1.0), (5.0, 5.0), (1.0, 5.0)])).expect("a plain key is taken");
    assert_eq!(layer(&doc), 1);
}
