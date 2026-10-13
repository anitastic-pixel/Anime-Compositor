//! B-516: D-636, Shatter's other ten patterns, after After Effects' Shatter (Simulation): herringbone
//! 2, squares and triangles, stars and triangles, overlapping squares, chevrons, puzzle, Js, eggs,
//! crescents and carpenter's wheel. Adobe names them and shows no geometry, so the shapes are ours,
//! stated in `tools/shatter_reference.py` (step 1).
//!
//! Writes `verification/D-636_patterns_table.md` and draws pictures into
//! `verification/D-636 pictures/`, over black so the white outlines show.
//!
//! Every expected pixel is in `Fixtures/shatter/expected_shatter.json` (FX-SH-093 to 104, and
//! FX-SH-061 checked by B-513's test), written by the tool before this code existed. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// FX-SH-001 at the street's size in `dir`, the holder's settings changed by `p`; `frame` drawn
/// on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/shatter/fx_sh_001.json")).unwrap()).unwrap();
    let a = project["assets"].as_array_mut().unwrap().iter_mut().find(|a| a["id"] == "asset-shot").unwrap();
    a["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let holder = comp["layers"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "holder").unwrap();
    let parameters = &mut holder["effects"][0]["parameters"];
    for (k, v) in p.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b516_patterns() {
    let mut t = Table::new(
        "shatter",
        "# D-636: Shatter's other ten patterns\n\nB-516, After Effects' Herringbone 2, Squares & \
         Triangles, Stars & Triangles, Overlapping Squares, Chevrons, Puzzle, Js, Eggs, Crescents \
         and Carpenter's Wheel, under the D-630 approval for the owner's /loop request (\"The \
         D-630 design is approved; go ahead.\") and the coordinator's \"Go on with D-636.\" Adobe \
         names the patterns and shows no geometry, so the shapes are ours, stated in \
         `tools/shatter_reference.py` (step 1). Every expected pixel was written by the tool \
         before this code existed. Tolerance 2e-5.\n",
    );

    t.heading("FX-SH-093 to 104 (document 25)");
    t.fixtures_numbered("expected_shatter.json", 93..=104);
    let why = t.load("fx_sh_104.json").document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0]
        .effect
        .why_invalid();
    t.row("fx_sh_104.json, pattern \"Puzzle\", is refused in a sentence naming the pattern", &why, why.to_lowercase().contains("pattern"));
    let list: Vec<String> = (93..=104).map(|n| format!("fx_sh_{n:03}.json")).collect();
    t.round_trips(&list.iter().map(String::as_str).collect::<Vec<_>>());

    t.heading("Pictures (verification/D-636 pictures)");
    let dir = std::env::temp_dir().join("b516_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(&dir.join("town.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    for m in ["back", "ramp", "colours", "cracks"] {
        fs::copy(repo(&format!("Fixtures/shatter/media/{m}.png")), dir.join(format!("media/{m}.png"))).unwrap();
    }
    let out = repo("verification/D-636 pictures");
    fs::create_dir_all(&out).unwrap();
    let patterns = [
        "herringbone_2",
        "squares_triangles",
        "stars_triangles",
        "overlapping_squares",
        "chevrons",
        "puzzle",
        "js",
        "eggs",
        "crescents",
        "carpenters_wheel",
    ];
    let mut shots: Vec<(String, J, i32, bool)> = patterns
        .iter()
        .enumerate()
        // Six across, the stars (three triangles across each) twelve.
        .map(|(n, p)| {
            let across = if *p == "stars_triangles" { 12 } else { 6 };
            (format!("{}_{p}_outlines", n + 1), json!({"view": "wireframe_front", "pattern": p, "repetitions": across, "force_1_radius": 0}), 0, true)
        })
        .collect();
    shots.push(("11_puzzle_burst".into(), json!({"pattern": "puzzle", "repetitions": 8, "force_1_strength": 1.5, "force_1_radius": 0.3}), 3, false));
    shots.push(("12_stars_burst".into(), json!({"pattern": "stars_triangles", "repetitions": 12, "force_1_strength": 1.5, "force_1_radius": 0.3}), 3, false));
    for (name, p, frame, wire) in shots {
        let (bytes, said) = picture(&dir, &p, frame);
        let white = bytes.chunks_exact(4).filter(|p| *p == [255, 255, 255, 255]).count();
        let clear = bytes.chunks_exact(4).filter(|p| p[3] == 0).count();
        // Saved over black, so the white outlines and the holes the pieces leave show in any
        // picture viewer; only the clear pixels change.
        let shown: Vec<u8> = bytes.chunks_exact(4).flat_map(|p| if p[3] == 0 { [0, 0, 0, 255] } else { [p[0], p[1], p[2], p[3]] }).collect();
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &shown).unwrap();
        if wire {
            t.row(
                &format!("{name}.png: drawn without a warning, white outlines on clear and nothing else"),
                &format!("{white} white, {clear} clear, {} other; warnings {said:?}", TOWN.0 * TOWN.1 - white - clear),
                said.is_empty() && white > 0 && white + clear == TOWN.0 * TOWN.1,
            );
        } else {
            t.row(
                &format!("{name}.png: drawn without a warning, the flown pieces leaving clear holes"),
                &format!("{clear} clear; warnings {said:?}"),
                said.is_empty() && clear > 0,
            );
        }
    }
    t.finish("D-636_patterns_table.md");
}
