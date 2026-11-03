//! B-905: D-1025, Particle Space's and Particle Plane's Extra Angle past a full turn. After
//! Effects types "1x+77" (437); ours refused anything past 360 and switched the effect off.
//!
//! Writes `verification/B-905_extra_angle_turns_table.md`. The fixtures' numbers are the
//! reference's (`tools/particle_space_reference.py`).

mod effect_table;

use std::fs;

use effect_table::{both, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;

/// `file` with its first effect's Extra Angle set to `angle`, opened; the effect's refusal.
fn why_at(t: &Table, dir: &str, file: &str, angle: f64) -> String {
    let path = t.root.parent().unwrap().join(dir).join(file);
    let mut j: J = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    j["compositions"][0]["layers"][0]["effects"][0]["parameters"]["extra_angle"] = J::from(angle);
    let loaded = anime_compositor::persist::load_str(&j.to_string()).expect("the file opens");
    let p = loaded.document.project();
    p.composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid()
}

#[test]
fn b905_extra_angle_turns() {
    let mut t = Table::new(
        "particle_space",
        "# B-905: Extra Angle past a full turn (D-1025)\n\n\
         After Effects shows CC Particle World's Extra Angle as turns and degrees and takes any \
         number of turns; the snow tutorial types 0x+77 after 1x+0. Ours refused anything past \
         360 and switched the effect off. The range is now 0 to 36000 on Particle Space and \
         Particle Plane, and a cone's width past 360 is held at 360. The fixtures' numbers are \
         the reference's.\n",
    );

    t.heading("The fixtures, at the reference's numbers: FX-PSPACE-001 to 073 unmoved, 074 new");
    t.fixtures("expected_particle_space.json");

    t.heading("What is taken and what is refused");
    for (dir, file, name) in [
        ("particle_space", "fx_pspace_026.json", "Particle Space"),
        ("particle_plane", "fx_pplane_001.json", "Particle Plane"),
    ] {
        for (angle, ok) in [(360.0, true), (437.0, true), (36000.0, true), (36000.5, false), (-1.0, false)] {
            let why = why_at(&t, dir, file, angle);
            t.row(
                &format!("{name}, Extra Angle {angle}: {}", if ok { "taken" } else { "refused, naming extra angle" }),
                if why.is_empty() { "taken" } else { &why },
                if ok { why.is_empty() } else { why.to_lowercase().contains("extra angle") || why.contains("extra_angle") },
            );
        }
    }

    t.heading("An old project");
    for file in ["fx_pspace_026.json", "fx_pspace_027.json", "fx_pspace_035.json"] {
        let text = fs::read_to_string(t.root.join(file)).unwrap();
        let loaded = anime_compositor::persist::load_str(&text).unwrap();
        let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
        let saved = effect_table::saved(&loaded);
        let same = effect_table::same_json(&saved, &serde_json::from_str(&text).unwrap());
        t.row(
            &format!("{file}, written before D-1025, opens with no warning and saves back as it was"),
            &format!("warnings {on_open:?}; saved {}", if same { "the same" } else { "differs" }),
            on_open.is_empty() && same,
        );
    }

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let project = t.load("fx_pspace_074.json").document.project().clone();
    let (mut largest, mut refused, mut agree) = (0, false, true);
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for frame in [12, 24] {
            let (d, r, a, b) = both(&mut gpu, &project, &Id::new(MAIN), &t.root, frame, quality);
            largest = largest.max(d.0);
            refused |= r;
            agree &= a == b;
        }
    }
    t.row(
        "fx_pspace_074.json, frames 12 and 24 at Full and Draft",
        &format!("largest difference {largest} of 255; refused by the card: {refused}; the same warnings: {agree}"),
        largest <= 1 && !refused && agree,
    );

    t.finish("B-905_extra_angle_turns_table.md");
}
