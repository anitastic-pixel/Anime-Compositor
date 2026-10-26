//! B-772: CC Light Rays fades with distance as After Effects' does (D-892): on display values
//! in 8 bpc and 32 bpc (After Effects), at 0.625 of the zoomed light. Writes
//! `verification/B-772_cc_light_rays_display_table.md`.

mod effect_table;

use std::fs;

use effect_table::{both, on_card, repo, Table};
use serde_json::{json, Value as J};

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;

fn is_cc(e: &Effect) -> bool {
    matches!(e, Effect::CcLightRays { .. })
}

/// The reference shot at `depth`, its three layers each given a CC Light Rays with `p` over
/// the settings a new one takes.
fn shot(depth: &str, p: &J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).unwrap();
    let mut j: J = serde_json::from_str(&text).unwrap();
    let comp = &mut j["compositions"][0];
    match depth {
        "8 bpc" => comp["eight_bpc"] = J::from(true),
        _ => {
            comp["float_depth"] = J::from(true);
            comp["ae_32bpc"] = J::from(true);
        }
    }
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let mut all = json!({"intensity": 100, "center": [25, 25], "radius": 50, "warp_softness": 50, "shape": "round", "direction": 0, "color_from_source": "on", "allow_brightening": "on", "color": "#ffffff", "transfer_mode": "none"});
        for (k, v) in p.as_object().unwrap() {
            all[k] = v.clone();
        }
        comp["layers"][i]["effects"] =
            json!([{"instance_id": format!("b772-{id}"), "type_id": "core.light_rays", "enabled": true, "parameters": all}]);
    }
    persist::load_str(&j.to_string()).unwrap().document.project().clone()
}

#[test]
fn b772_cc_light_rays_display() {
    let mut t = Table::new(
        "light_rays_cc",
        "# B-772: CC Light Rays on display values, at 0.625 of the zoomed light\n\nD-892, accepted \
         on 2026-10-26. In an 8 bpc composition and in 32 bpc (After Effects) CC Light Rays works \
         on display values, as Glow does there, so its rays fade with distance as After Effects' \
         do; at every depth the rays are laid on at 0.625 of the zoomed light. Every expected \
         pixel is `Fixtures/light_rays_cc/expected_light_rays_cc.json`, written by \
         `tools/light_rays_cc_reference.py`; the answer is the largest difference against the \
         catalogue's tolerance of 2e-5. The card must be within 1 level of 255 of the processor.\n",
    );

    t.heading("FX-RAYSCC-001 to 026 and 040 to 042 (document 25), every case D-892 changed or added");
    t.fixtures_numbered("expected_light_rays_cc.json", 1..=26);
    t.fixtures_numbered("expected_light_rays_cc.json", 40..=42);

    t.heading("Older projects");
    let files: Vec<String> = (1..=26)
        .filter(|n| *n != 11) // its capitals are saved in small letters, as before D-892
        .map(|n| format!("fx_rayscc_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let cc_root = t.root.clone();
    t.root = repo("Fixtures/light_rays");
    t.heading("D-124's form (FX-RAYS-001 to 024), drawn as before");
    t.fixtures("expected_light_rays.json");
    t.root = cc_root;

    t.heading("On the card against the processor, the reference shot");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new("comp-reference-shot");
    let root = repo("Fixtures/reference_shot");
    for depth in ["8 bpc", "32 bpc (After Effects)"] {
        for (what, p) in [
            ("as it starts", json!({})),
            ("radius 400, warp 300, Add", json!({"radius": 400, "warp_softness": 300, "transfer_mode": "add"})),
            ("Color from Source off #ff8000, intensity 250, Screen", json!({"color_from_source": "off", "color": "#ff8000", "intensity": 250, "transfer_mode": "screen", "radius": 300})),
        ] {
            let project = shot(depth, &p);
            for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
                for frame in [0, 100] {
                    let (d, r, a, b) = both(&mut gpu, &project, &comp, &root, frame, quality);
                    let card = on_card(&project, &comp, &root, frame, quality, is_cc);
                    // In 32 bpc (After Effects) an effect on display values is the processor's.
                    let want = if depth == "8 bpc" { 3 } else { 0 };
                    t.row(
                        &format!("{depth}, {what}, frame {frame}, {}", quality.label()),
                        &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                        d.0 <= 1 && !r && a == b && card == want,
                    );
                }
            }
        }
    }

    t.finish("B-772_cc_light_rays_display_table.md");
}
