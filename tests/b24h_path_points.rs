//! B-24h: a point added to or taken off a path that moves, against D-79.
//!
//! Writes `verification/B-24h_path_point_table.md`.
//!
//! # Where the expected values come from
//!
//! Not from a fixture of pixels, because nothing here is a new picture: the claim D-79 makes is
//! that adding a point *changes no shape at all*, and the honest way to check that is to compare
//! the curve against the curve there was. The curve is read straight from document 19's cubic --
//! `(1-u)^3 A + 3(1-u)^2 u B + 3(1-u) u^2 C + u^3 D`, written out again here rather than called
//! out of `src/mask.rs` -- at two hundred places along every segment, before and after. De
//! Casteljau's cut says where each of those places went: the piece before the cut holds the
//! first `t` of the curve, stretched, and the piece after it holds the rest. If the two agree
//! everywhere, to the last few digits a double can hold, the shape did not move.
//!
//! The pixels are checked as well, and they are the one place where an answer is not exact:
//! `flatten` cuts a curve into pieces about two pixels long, and two shorter curves are not cut
//! at the same places as one longer one. That difference is measured and printed rather than
//! hidden, against a ceiling written into this file.
//!
//! # What is deliberately not here
//!
//! The window -- the pen on an edge or on a point, and Delete with several points chosen -- is
//! B-24h's and B-24i's playtest sheets. What a
//! path that stands still does is B-24b's and B-06's tables, and how a keyed path is drawn at a
//! frame is B-24d's, all unchanged by this unit.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::mask::{self, Mask, MaskKey, MaskPoint};
use anime_compositor::model::{Id, Interp};
use anime_compositor::persist;

const COMP: &str = "comp-main";

/// The largest the drawn edge may move when a curved segment is cut, in coverage, on the fields
/// below: one of the sixteen samples in a pixel, and no pixel more than one.
///
/// It is flattening and nothing else. `flatten` cuts a curve into pieces about two pixels long,
/// and two short curves are not cut at the same places as one long one, so a sample that fell a
/// hair inside the chord of a long piece can fall a hair outside the chord of a short one. A
/// straight segment is cut exactly and is checked for no difference at all.
const FLATTENING_CEILING: f32 = 1.0 / 16.0;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

struct Table {
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | As asked |\n| --- | --- | --- |\n"
        ));
    }
}

/// A layer's masks as the document holds them now.
fn masks_of(document: &Document) -> Vec<Mask> {
    document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new("cel"))
        .unwrap()
        .masks
        .clone()
}

/// One edit, and the entry the history panel would show for it.
fn did(
    t: &mut Table,
    document: &mut Document,
    what: &str,
    want: &str,
    change: impl FnOnce(&mut Vec<Mask>),
) {
    let mut masks = masks_of(document);
    change(&mut masks);
    let got = document
        .apply(Command::SetMasks {
            composition: Id::new(COMP),
            layer_id: Id::new("cel"),
            masks,
        })
        .map(|r| r.label.clone())
        .unwrap_or_else(|d| format!("refused: {}", d.message));
    t.row(what, &format!("\"{got}\""), got == want);
}

/// Document 19's cubic at `u`, from the two points that hold the segment.
fn cubic(a: &MaskPoint, b: &MaskPoint, u: f64) -> (f64, f64) {
    let p = a.point;
    let q = (p.0 + a.out_handle.0, p.1 + a.out_handle.1);
    let s = b.point;
    let r = (s.0 + b.in_handle.0, s.1 + b.in_handle.1);
    let v = 1.0 - u;
    let w = (v * v * v, 3.0 * v * v * u, 3.0 * v * u * u, u * u * u);
    (
        w.0 * p.0 + w.1 * q.0 + w.2 * r.0 + w.3 * s.0,
        w.0 * p.1 + w.1 * q.1 + w.2 * r.1 + w.3 * s.1,
    )
}

/// The largest distance, in pixels, between the curve `before` traces and the curve `after`
/// traces, once the cut at `(seg, t)` is undone by parameter.
///
/// Every segment but the cut one is the same segment moved along by one place. The cut one is
/// two: the first holds `u` from 0 to `t`, the second the rest, each stretched over its own 0
/// to 1. Two hundred places along each segment.
fn curves_apart(before: &[MaskPoint], after: &[MaskPoint], seg: usize, t: f64) -> f64 {
    let n = before.len();
    let m = after.len();
    let mut worst: f64 = 0.0;
    for j in 0..n {
        for k in 0..=200 {
            let u = k as f64 / 200.0;
            let was = cubic(&before[j], &before[(j + 1) % n], u);
            let is = if j == seg && u <= t {
                cubic(&after[seg], &after[seg + 1], u / t)
            } else if j == seg {
                cubic(&after[seg + 1], &after[(seg + 2) % m], (u - t) / (1.0 - t))
            } else {
                let start = j + (j > seg) as usize;
                cubic(&after[start], &after[(start + 1) % m], u)
            };
            worst = worst.max((was.0 - is.0).hypot(was.1 - is.1));
        }
    }
    worst
}

/// A mask of these points, and nothing else set, drawn into a coverage field 64 by 48.
fn field(points: &[MaskPoint]) -> Vec<f32> {
    let mask = Mask {
        points: points.to_vec(),
        ..Default::default()
    };
    mask::coverage(&[mask], 64, 48).expect("a mask in Add mode covers something")
}

/// How many pixels of two fields differ at all, and by how much at the worst of them.
fn pixels_apart(before: &[f32], after: &[f32]) -> (usize, f32) {
    let differ = before.iter().zip(after).filter(|(a, b)| a != b).count();
    let worst = before
        .iter()
        .zip(after)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f32::max);
    (differ, worst)
}

fn square() -> Vec<MaskPoint> {
    vec![
        MaskPoint::corner(12.0, 10.0),
        MaskPoint::corner(50.0, 10.0),
        MaskPoint::corner(50.0, 38.0),
        MaskPoint::corner(12.0, 38.0),
    ]
}

/// A blob: four points, every one of them with two handles, none of them symmetrical.
fn blob() -> Vec<MaskPoint> {
    let p = |x, y, ix, iy, ox, oy| MaskPoint {
        point: (x, y),
        in_handle: (ix, iy),
        out_handle: (ox, oy),
    };
    vec![
        p(32.0, 6.0, -11.0, 1.0, 13.0, -2.0),
        p(56.0, 24.0, 2.0, -9.0, -1.0, 12.0),
        p(30.0, 42.0, 14.0, 3.0, -12.0, -1.0),
        p(8.0, 22.0, -2.0, 10.0, 1.0, -13.0),
    ]
}

#[test]
fn b24h_path_points() {
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };
    t.out.push_str(
        "# B-24h: a point added to a path that moves\n\nD-79 lifts the limit B-24d wrote down: a \
         point may now be added to or taken off a path that has keys, because the point is added \
         to the path and to every key on it at once, which keeps the agreement D-77 rests on -- \
         every key holds as many points as the path itself.\n\nThe promise a person is being \
         asked to believe is that **adding a point moves nothing**. That is checked here against \
         the curve rather than against a picture: document 19's cubic is written out again in \
         this file and read at two hundred places along every segment, before and after, with de \
         Casteljau's cut undone by parameter. A cut segment is two segments afterwards, so the \
         first holds the curve from 0 to `t` and the second the rest. A segment with no handles \
         is a straight line and is checked as a line instead, because document 19's cubic walks \
         such a segment unevenly and cutting it changes when the line is walked without changing \
         where it goes.\n\nThe drawn pixels are checked too, and they are the one honest \
         exception: `flatten` cuts a curve into pieces about two pixels long, and two short \
         curves are not cut at the same places as one long one. A straight segment is cut exactly \
         and is checked for **no pixel at all**; a curved one is measured against a ceiling of \
         one of a pixel's sixteen samples, written into `tests/b24h_path_points.rs`.\n\nThe last \
         table is the other limit B-24d wrote down: every mask edit there has ever been read \
         \"Set mask of N points\" in the history, because the core takes the whole list of masks \
         as one command. It now reads what changed, which is worked out from the list as it was \
         rather than from anything the window says about itself.\n",
    );

    // -----------------------------------------------------------------------------------
    t.heading("A curve cut: the curve does not move (D-79)");
    for (seg, tt) in [(0usize, 0.5f64), (1, 0.25), (2, 0.1), (3, 0.8)] {
        let mut after = blob();
        mask::insert_point(&mut after, &mut [], seg, tt);
        let apart = curves_apart(&blob(), &after, seg, tt);
        t.row(
            &format!(
                "a blob of curves, cut at {}% of segment {seg}{}",
                tt * 100.0,
                if seg == 3 { " -- the segment that closes the path" } else { "" }
            ),
            &format!("{} points, {apart:.2e} px apart at the worst", after.len()),
            after.len() == 5 && apart < 1e-9,
        );
    }

    // -----------------------------------------------------------------------------------
    t.heading("A straight segment cut: a corner, on the line");
    // A segment with no handles is a straight line, and it is checked as a line rather than by
    // parameter: document 19's cubic walks a handle-less segment unevenly -- slowly at each end
    // and quickly in the middle -- so cutting it in two changes when the line is walked without
    // changing one point of where it goes. What a person can see is the where, and the pixels
    // below say it more plainly than any number of decimal places.
    let mut corners = square();
    mask::insert_point(&mut corners, &mut [], 1, 0.4);
    let (top, bottom) = (square()[1].point, square()[2].point);
    let side = (bottom.0 - top.0, bottom.1 - top.1);
    let off = (corners[2].point.0 - top.0, corners[2].point.1 - top.1);
    let adrift = (side.0 * off.1 - side.1 * off.0).abs() / side.0.hypot(side.1);
    t.row(
        "the new point sits on the line it was put on",
        &format!("{:?}, {adrift:.2e} px off the line", corners[2].point),
        adrift == 0.0,
    );
    t.row(
        "and it is a corner: no handles on it, and none grown on its neighbours",
        &format!(
            "{:?} and {:?} on the new point, {:?} and {:?} beside it",
            corners[2].in_handle, corners[2].out_handle, corners[1].out_handle, corners[3].in_handle
        ),
        corners[2].in_handle == (0.0, 0.0)
            && corners[2].out_handle == (0.0, 0.0)
            && corners[1].out_handle == (0.0, 0.0)
            && corners[3].in_handle == (0.0, 0.0),
    );

    // -----------------------------------------------------------------------------------
    t.heading("The pixels drawn");
    let straight = pixels_apart(&field(&square()), &field(&corners));
    t.row(
        "a straight segment cut: not one pixel of the drawn mask differs",
        &format!("{} pixels differ, worst {:.6}", straight.0, straight.1),
        straight == (0, 0.0),
    );
    let plain = field(&blob());
    let (mut most, mut worst) = (0usize, 0.0f32);
    for (seg, tt) in [(0usize, 0.5f64), (1, 0.25), (2, 0.1), (3, 0.8)] {
        let mut curved = blob();
        mask::insert_point(&mut curved, &mut [], seg, tt);
        let (differ, by) = pixels_apart(&plain, &field(&curved));
        most = most.max(differ);
        worst = worst.max(by);
    }
    t.row(
        "a curved segment cut, four places: the edge moves by flattening and no more than that",
        &format!(
            "at the worst of the four, {most} of 3072 pixels differ, by {worst:.6} of full \
             coverage -- {} of the sixteen samples in a pixel (ceiling {FLATTENING_CEILING})",
            (worst * 16.0).round()
        ),
        worst <= FLATTENING_CEILING,
    );

    // -----------------------------------------------------------------------------------
    t.heading("Every key keeps its own shape (D-79)");
    // Two keys that are not the same shape, so that a cut which used one key's numbers on the
    // other would show at once.
    let moved: Vec<MaskPoint> = blob()
        .iter()
        .map(|p| MaskPoint {
            point: (p.point.0 * 0.6 + 8.0, p.point.1 * 1.2 - 3.0),
            in_handle: (p.in_handle.1, -p.in_handle.0),
            out_handle: (p.out_handle.1, -p.out_handle.0),
        })
        .collect();
    let mut keyed = Mask {
        points: blob(),
        keys: vec![
            MaskKey {
                frame: 0,
                points: blob(),
                interp: Interp::Linear,
            },
            MaskKey {
                frame: 12,
                points: moved.clone(),
                interp: Interp::Linear,
            },
        ],
        ..Default::default()
    };
    let was = keyed.clone();
    mask::insert_point(&mut keyed.points, &mut keyed.keys, 2, 0.35);
    t.row(
        "the base and both keys hold the same number of points, as D-77 asks",
        &format!(
            "base {}, keys {:?}",
            keyed.points.len(),
            keyed.keys.iter().map(|k| k.points.len()).collect::<Vec<_>>()
        ),
        keyed.points.len() == 5 && keyed.keys.iter().all(|k| k.points.len() == 5),
    );
    for (i, key) in keyed.keys.iter().enumerate() {
        let apart = curves_apart(&was.keys[i].points, &key.points, 2, 0.35);
        t.row(
            &format!("the key at frame {}: its own curve is where it was", key.frame),
            &format!("{apart:.2e} px apart at the worst"),
            apart < 1e-9,
        );
    }
    let mut between = 0.0f64;
    for frame in -2..=15 {
        let a = mask::points_at(&was.points, &was.keys, frame);
        let b = mask::points_at(&keyed.points, &keyed.keys, frame);
        between = between.max(curves_apart(&a, &b, 2, 0.35));
    }
    t.row(
        "and the shape between the keys does not move either, frames -2 to 15",
        &format!("{between:.2e} px apart at the worst of all of them"),
        between < 1e-9,
    );

    // -----------------------------------------------------------------------------------
    t.heading("A point taken off (D-79)");
    let mut short = was.clone();
    mask::remove_point(&mut short.points, &mut short.keys, 1);
    t.row(
        "the point goes off the base and off every key, and the rest are untouched",
        &format!(
            "base {} points, keys {:?}",
            short.points.len(),
            short.keys.iter().map(|k| k.points.len()).collect::<Vec<_>>()
        ),
        short.points.len() == 3
            && short.keys.iter().all(|k| k.points.len() == 3)
            && short.points == [was.points[0], was.points[2], was.points[3]]
            && short.keys[1].points == [moved[0], moved[2], moved[3]],
    );

    // -----------------------------------------------------------------------------------
    t.heading("What the core makes of it");
    let opened = persist::load(&repo("Fixtures/masks/fx_msk_001.json")).expect("fx_msk_001 opens");
    let preserved = opened.preserved;
    let mut document = opened.document;
    let set = |document: &mut Document, masks: Vec<Mask>| {
        document
            .apply(Command::SetMasks {
                composition: Id::new(COMP),
                layer_id: Id::new("cel"),
                masks,
            })
            .map(|r| r.label.clone())
            .map_err(|d| (d.id, d.detail))
    };
    let mut mask = document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new("cel"))
        .unwrap()
        .masks[0]
        .clone();
    mask.keys = vec![
        MaskKey {
            frame: 0,
            points: mask.points.clone(),
            interp: Interp::Linear,
        },
        MaskKey {
            frame: 6,
            points: mask.points.iter().map(|p| MaskPoint { point: (p.point.0 + 4.0, p.point.1), ..*p }).collect(),
            interp: Interp::Linear,
        },
    ];
    let keyed_mask = mask.clone();
    let before = document.project().clone();
    mask::insert_point(&mut mask.points, &mut mask.keys, 0, 0.5);
    let added = set(&mut document, vec![mask.clone()]);
    t.row(
        "a keyed path with a point added is accepted, where one key alone is refused",
        &match &added {
            Ok(label) => format!("accepted, and the history entry reads \"{label}\""),
            Err((id, detail)) => format!("{id:?}: {detail}"),
        },
        added.as_deref() == Ok("Add a point to Mask 1"),
    );
    let mut one_key = keyed_mask.clone();
    mask::insert_point(&mut one_key.keys[1].points, &mut [], 0, 0.5);
    let refused = set(&mut document, vec![one_key]);
    t.row(
        "the D-77 gate still holds: a point added to one key alone is refused",
        &match &refused {
            Ok(label) => format!("accepted: {label}"),
            Err((id, _)) => format!("{id:?}"),
        },
        refused.as_ref().err().map(|(id, _)| *id) == Some(DiagnosticId::MaskInvalidOutline),
    );
    let mut taken = mask.clone();
    mask::remove_point(&mut taken.points, &mut taken.keys, 0);
    let removed = set(&mut document, vec![taken.clone()]);
    t.row(
        "a point taken off a keyed path is accepted too",
        &match &removed {
            Ok(label) => format!("accepted, and the history entry reads \"{label}\""),
            Err((id, detail)) => format!("{id:?}: {detail}"),
        },
        removed.as_deref() == Ok("Remove a point from Mask 1"),
    );
    let mut down_to_two = taken.clone();
    mask::remove_point(&mut down_to_two.points, &mut down_to_two.keys, 0);
    mask::remove_point(&mut down_to_two.points, &mut down_to_two.keys, 0);
    let too_few = set(&mut document, vec![down_to_two]);
    t.row(
        "a path taken down to two points is refused, by the rule this build already had",
        &match &too_few {
            Ok(label) => format!("accepted: {label}"),
            Err((_, detail)) => detail.clone(),
        },
        too_few.is_err(),
    );
    while document.undo().is_some() {}
    t.row(
        "and undo gives back the project as it was before any of it",
        if document.project() == &before {
            "the same project"
        } else {
            "different"
        },
        document.project() == &before,
    );

    // A file with the added point, saved and opened again.
    let _ = set(&mut document, vec![mask.clone()]);
    let out = std::env::temp_dir().join("b24h_added_point.json");
    persist::save(&out, &mut document, &preserved).expect("it saves");
    let again = persist::load(&out).expect("it opens");
    let back = again
        .document
        .project()
        .composition(&Id::new(COMP))
        .unwrap()
        .layer(&Id::new("cel"))
        .unwrap()
        .masks[0]
        .clone();
    // The file is written to a temporary folder, which is not where the fixture's drawing is, so
    // the drawing is reported missing. That is the folder and not the path: what this row asks
    // is that nothing at all is said about the mask.
    let said: Vec<DiagnosticId> = again.warnings.iter().map(|d| d.id).collect();
    t.row(
        "saved and opened again, the added point is on the base and on both keys, unremarked",
        &format!(
            "base {} points, keys {:?}, and the file is read back with {said:?}",
            back.points.len(),
            back.keys.iter().map(|k| k.points.len()).collect::<Vec<_>>(),
        ),
        back == mask
            && !said.contains(&DiagnosticId::ProjectSchemaInvalid)
            && !said.contains(&DiagnosticId::MaskInvalidOutline),
    );
    fs::remove_file(&out).ok();

    // -----------------------------------------------------------------------------------
    t.heading("What the history entry says (B-24h)");
    while document.undo().is_some() {}
    let mut d = document;
    did(&mut t, &mut d, "drew a second mask", "Draw Mask 2", |masks| {
        masks.push(Mask {
            name: "Mask 2".to_string(),
            points: square(),
            ..Default::default()
        })
    });
    did(&mut t, &mut d, "pulled one point", "Move a point of Mask 1", |masks| {
        masks[0].points[1].point.0 += 5.0
    });
    did(&mut t, &mut d, "pulled the whole path", "Move Mask 1's path", |masks| {
        masks[0].points.iter_mut().for_each(|p| p.point.1 += 2.0)
    });
    did(&mut t, &mut d, "set a feather", "Set Mask 1's feather to 6 px", |masks| {
        masks[0].feather_px = 6.0
    });
    did(&mut t, &mut d, "set an expansion", "Set Mask 1's expansion to -3.5 px", |masks| {
        masks[0].expansion_px = -3.5
    });
    did(&mut t, &mut d, "set an opacity", "Set Mask 1's opacity to 40%", |masks| {
        masks[0].opacity = 0.4
    });
    did(&mut t, &mut d, "changed the mode", "Set Mask 1 to subtract", |masks| {
        masks[0].mode = anime_compositor::mask::MaskMode::Subtract
    });
    did(&mut t, &mut d, "inverted it", "Invert Mask 1", |masks| masks[0].inverted = true);
    did(&mut t, &mut d, "switched it off", "Switch Mask 1 off", |masks| {
        masks[0].enabled = false
    });
    did(&mut t, &mut d, "renamed it", "Rename Mask 1 to Face", |masks| {
        masks[0].name = "Face".to_string()
    });
    did(&mut t, &mut d, "pressed the path's stopwatch", "Start Face's path moving", |masks| {
        masks[0].keys = vec![MaskKey {
            frame: 0,
            points: masks[0].points.clone(),
            interp: Interp::Linear,
        }]
    });
    did(&mut t, &mut d, "put a second key down", "Key Face's path at frame 6", |masks| {
        let mut moved = masks[0].points.clone();
        moved[0].point.0 += 7.0;
        masks[0].keys.push(MaskKey {
            frame: 6,
            points: moved,
            interp: Interp::Linear,
        })
    });
    did(&mut t, &mut d, "pulled a point at a key", "Move a point of Face at frame 6", |masks| {
        masks[0].keys[1].points[2].point.1 += 3.0
    });
    did(&mut t, &mut d, "dragged that key along its row", "Move Face's path key from frame 6 to 9", |masks| {
        masks[0].keys[1].frame = 9
    });
    did(&mut t, &mut d, "eased it with F9", "Change the ease on Face's path key at frame 0", |masks| {
        masks[0].keys[0].interp = Interp::Ease { x1: 0.33, y1: 0.0, x2: 0.67, y2: 1.0 }
    });
    did(&mut t, &mut d, "added a point to the moving path", "Add a point to Face", |masks| {
        let mask = &mut masks[0];
        mask::insert_point(&mut mask.points, &mut mask.keys, 1, 0.5)
    });
    did(&mut t, &mut d, "took it off again", "Remove a point from Face", |masks| {
        let mask = &mut masks[0];
        mask::remove_point(&mut mask.points, &mut mask.keys, 2)
    });
    did(&mut t, &mut d, "took the stopwatch off", "Stop Face's path moving", |masks| {
        masks[0].keys.clear()
    });
    did(&mut t, &mut d, "deleted the second mask", "Delete Mask 2", |masks| {
        masks.remove(1);
    });

    // -----------------------------------------------------------------------------------
    t.out.push_str(&format!(
        "\n## Result\n\n{} of {} checks pass.\n",
        t.passed, t.checks
    ));
    fs::write(repo("verification/B-24h_path_point_table.md"), &t.out).unwrap();
    assert_eq!(
        t.passed, t.checks,
        "see verification/B-24h_path_point_table.md"
    );
}
