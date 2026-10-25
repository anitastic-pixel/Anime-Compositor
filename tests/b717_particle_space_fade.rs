//! B-717: D-837, Particle Space's Opacity Map: its particles fade in at birth and out at death,
//! as CC Particle World's map does when added (five tutorials); Constant for a file without it.
//!
//! Writes `verification/B-717_particle_space_fade_table.md`. The fixtures' numbers are the
//! reference's (`tools/particle_space_reference.py`).

mod effect_table;

use std::fs;

use effect_table::{both, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::effects::{Effect, PARTICLE_SPACE_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;

/// The `art` layer's Particle Space words in `file`.
fn words(t: &Table, file: &str) -> Vec<String> {
    let d = t.load(file).document;
    let e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0]
        .effect
        .clone();
    match e {
        Effect::ParticleSpace { words, .. } => words.to_vec(),
        _ => panic!("not a Particle Space"),
    }
}

/// The map D-837 states, at age fraction `f`.
fn fade(f: f64) -> f64 {
    let s = [(0.0, 0.0), (0.025, 0.24), (0.05, 0.41), (0.075, 0.55), (0.1, 0.67), (0.15, 0.84), (0.2, 0.94), (0.25, 0.98), (0.3, 1.0), (0.7, 1.0), (0.75, 0.98), (0.8, 0.94), (0.85, 0.84), (0.9, 0.67), (0.925, 0.55), (0.95, 0.41), (0.975, 0.24), (1.0, 0.0)];
    let i = s.windows(2).position(|w| f <= w[1].0).unwrap();
    s[i].1 + (s[i + 1].1 - s[i].1) * (f - s[i].0) / (s[i + 1].0 - s[i].0)
}

#[test]
fn b717_particle_space_fade() {
    let mut t = Table::new(
        "particle_space",
        "# B-717: Particle Space fades its particles in and out (D-837)\n\n\
         CC Particle World's Opacity Map, as added, fades each particle in at birth and out at death \
         (five tutorials from five creators; the shape read off two of them at 18 points: life 0, 0.025, 0.05, \
         0.075, 0.1, 0.15, 0.2, 0.25, 0.3 at opacity 0, 0.24, 0.41, 0.55, 0.67, 0.84, 0.94, 0.98, 1, and mirrored). Ours drew every particle \
         at Max Opacity all its life. Particle Space now has an Opacity Map, Fade In and Out when \
         added and Constant for a file without it, on the processor and the card. The fixtures' \
         numbers are the reference's.\n",
    );

    t.heading("The fixtures, at the reference's numbers: FX-PSPACE-001 to 071 unmoved, 072 and 073 new");
    t.fixtures("expected_particle_space.json");

    t.heading("The one disc's centre through frames 10 to 21: the map at its age");
    let p = t.load("fx_pspace_072.json").document;
    let born = 0.5 / 1.3 * 24.0;
    for frame in [10, 11, 12, 13, 14, 15, 17, 19, 20, 21] {
        let f = (frame as f64 - born) / 48.0;
        let got = t.render(&p, frame, 64);
        let centre = (0..got.width() * got.height())
            .map(|i| got.pixel(i % got.width(), i / got.width())[3] as f64)
            .fold(0.0, f64::max);
        t.row(
            &format!("frame {frame}, {f:.3} of its life: {:.3} covered", fade(f)),
            &format!("{centre:.4}"),
            (centre - fade(f)).abs() < 1e-5,
        );
    }

    t.heading("The file");
    t.round_trips(&["fx_pspace_072.json", "fx_pspace_073.json"]);
    let saved = t.saved_parameters("fx_pspace_072.json");
    t.row(
        "fx_pspace_072.json is saved with its Opacity Map",
        &format!("opacity_map {}", saved["opacity_map"]),
        saved["opacity_map"] == "fade_in_and_out",
    );
    t.row(
        "A Particle Space added from the panel starts at Fade In and Out",
        PARTICLE_SPACE_WORDS[7].2,
        PARTICLE_SPACE_WORDS[7] == ("opacity_map", &["constant", "fade_in_and_out"][..], "fade_in_and_out"),
    );
    let mut bad: J = serde_json::from_str(&fs::read_to_string(t.root.join("fx_pspace_072.json")).unwrap()).unwrap();
    bad["compositions"][0]["layers"][0]["effects"][0]["parameters"]["opacity_map"] = J::from("fade_in");
    let loaded = anime_compositor::persist::load_str(&bad.to_string()).expect("the file opens");
    let e = &loaded.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect;
    let why = e.why_invalid();
    t.row(
        "Opacity Map \"fade_in\" is refused in a sentence naming opacity map",
        &why,
        why.to_lowercase().contains("opacity map"),
    );

    t.heading("An old project");
    for file in ["fx_pspace_002.json", "fx_pspace_024.json", "fx_pspace_047.json"] {
        let text = fs::read_to_string(t.root.join(file)).unwrap();
        let loaded = anime_compositor::persist::load_str(&text).unwrap();
        let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
        let w = words(&t, file);
        t.row(
            &format!("{file}, written before D-837 without an Opacity Map, opens with no warning and Constant"),
            &format!("opacity map {}; warnings {on_open:?}; the file holds opacity_map: {}", w[7], text.contains("opacity_map")),
            w[7] == "constant" && on_open.is_empty() && !text.contains("opacity_map"),
        );
        let saved = effect_table::saved(&loaded);
        let same = effect_table::same_json(&saved, &serde_json::from_str(&text).unwrap());
        t.row(
            "and saves back holding what it held, no Opacity Map added",
            if same { "the same" } else { "differs" },
            same && !saved.to_string().contains("opacity_map"),
        );
    }

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    for (file, frames) in [("fx_pspace_072.json", &[11, 20][..]), ("fx_pspace_073.json", &[12][..]), ("fx_pspace_024.json", &[12][..])] {
        let project = t.load(file).document.project().clone();
        let (mut largest, mut refused, mut agree) = (0, false, true);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in frames {
                let (d, r, a, b) = both(&mut gpu, &project, &Id::new(MAIN), &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
            }
        }
        t.row(
            &format!("{file}, frames {frames:?} at Full and Draft"),
            &format!("largest difference {largest} of 255; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree,
        );
    }

    t.finish("B-717_particle_space_fade_table.md");
}
