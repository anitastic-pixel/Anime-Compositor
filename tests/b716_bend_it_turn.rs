//! B-716: D-836, Bend It turns 1.8 degrees for each unit of Bend (Bend 100 is half a turn),
//! After Effects' CC Bend It as seven tutorials measure it (it was a degree a unit).
//!
//! Writes `verification/B-716_bend_it_turn_table.md`. The fixtures' numbers are the reference's
//! (`tools/bend_it_reference.py`), whose cases' Bend D-836 divided by 1.8 so their pixels stay;
//! the turn is read off a white bar bent in a precomposition.

mod effect_table;

use std::fs;

use effect_table::{both, largest_difference, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;

const W: usize = 400;
const BAR: (f64, f64) = (20.0, 200.0); // the bar's width and length, standing up the middle

/// A 400 by 400 composition holding a 400 by 400 precomposition of a white 20 by 200 bar
/// standing up its middle (rows 100 to 299), with Bend It over the whole bar: Start at its foot
/// (50, 75), End at its top (50, 25), Render Prestart None, Distort Legal.
fn shot(bend: f64) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let comp = |id: &str, layers: J, order: J| json!({
        "id": id, "name": id, "width": W, "height": W, "pixel_aspect_ratio": 1,
        "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
        "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
        "layer_order": order, "layers": layers
    });
    let transform = |anchor: J, position: J| json!({
        "anchor": t(anchor), "position": t(position),
        "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
    });
    let bar = json!({
        "id": "bar", "kind": "solid", "name": "bar", "enabled": true, "locked": false,
        "in_frame": 0, "out_frame": 1,
        "solid": {"color": [1.0, 1.0, 1.0], "width": BAR.0 as u32, "height": BAR.1 as u32},
        "transform": transform(json!([BAR.0 / 2.0, BAR.1 / 2.0]), json!([200, 200])),
        "matte": null, "blend_mode": "normal", "effects": []
    });
    let inner = json!({
        "id": "inner", "kind": "composition", "name": "inner", "composition_id": "comp-bar",
        "enabled": true, "locked": false, "in_frame": 0, "out_frame": 1, "source_offset_frames": 0,
        "transform": transform(json!([200, 200]), json!([200, 200])),
        "masks": [], "matte": null, "blend_mode": "normal",
        "effects": [{"instance_id": "f", "type_id": "core.bend_it", "enabled": true, "parameters": {
            "bend": bend, "start": [50, 75], "end": [50, 25],
            "render_prestart": "none", "distort": "legal"}}]
    });
    json!({
        "schema_version": 0, "project_id": "proj-b716",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [
            comp(MAIN, json!([inner]), json!(["inner"])),
            comp("comp-bar", json!([bar]), json!(["bar"]))
        ]
    })
    .to_string()
}

/// Where a bar bent `theta` radians has its covering's centre: the bar's points (u along it from
/// the foot, v across) laid on the circle as document 21 lays them, weighted by the area each
/// covers, |R - v| / |R|.
fn centre_for(theta: f64) -> (f64, f64) {
    let (sx, sy, l) = (200.0, 300.0, BAR.1);
    let (mut x, mut y, mut w) = (0.0, 0.0, 0.0);
    for i in 0..200 {
        for j in 0..20 {
            let u = (i as f64 + 0.5) / 200.0 * l;
            let v = (j as f64 + 0.5) / 20.0 * BAR.0 - BAR.0 / 2.0;
            let (px, py, k) = if theta.abs() < 1e-9 {
                (sx + v, sy - u, 1.0)
            } else {
                let r = l / theta;
                let phi = u * theta / l;
                // t = (0, -1), n = (1, 0); C = S + R n; the point C + (v - R) n(phi).
                (sx + r + (v - r) * phi.cos(), sy + (v - r) * phi.sin(), ((r - v) / r).abs())
            };
            x += k * px;
            y += k * py;
            w += k;
        }
    }
    (x / w, y / w)
}

/// The turn, in degrees, of the bar `project` draws: the theta whose covering's centre is nearest
/// the drawn covering's centre, searched every 0.05 degrees from -359 to 359.
fn turn(project: &str) -> f64 {
    let loaded = persist::load_str(project).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log)
        .expect("the shot draws");
    let (mut x, mut y, mut w) = (0.0, 0.0, 0.0);
    for py in 0..W {
        for px in 0..W {
            let a = f.pixel(px, py)[3] as f64;
            x += a * (px as f64 + 0.5);
            y += a * (py as f64 + 0.5);
            w += a;
        }
    }
    let (cx, cy) = (x / w, y / w);
    let mut best = (f64::INFINITY, 0.0);
    for k in -7180..=7180 {
        let deg = k as f64 * 0.05;
        let (ex, ey) = centre_for(deg.to_radians());
        let d = (ex - cx).hypot(ey - cy);
        if d < best.0 {
            best = (d, deg);
        }
    }
    best.1
}

#[test]
fn b716_bend_it_turn() {
    let mut t = Table::new(
        "bend_it",
        "# B-716: Bend It turns 1.8 degrees for each unit of Bend (D-836)\n\n\
         Seven tutorials from six creators measure After Effects' CC Bend It turning 1.72 to 1.90 \
         degrees for each unit of Bend (median 1.82): Bend 100 is half a turn. Ours turned a degree \
         a unit. The bar from Start to End is now bent through Bend x 1.8 degrees, on the processor \
         and the card. The turn is read off a white 20 by 200 bar standing up a 400 by 400 \
         precomposition, Start at its foot and End at its top: the turn whose bent bar has its \
         covering's centre where the drawn bar has it. The fixtures' numbers are the reference's, \
         whose cases' Bend D-836 divided by 1.8 so their pixels stay.\n",
    );

    t.heading("The bar's turn, in degrees, within 1 degree of Bend x 1.8");
    for bend in [25.0, 50.0, 100.0, -50.0, 150.0] {
        let got = turn(&shot(bend));
        let want = bend * 1.8;
        t.row(
            &format!("Bend {bend}: {want} degrees ({bend} degrees before D-836)"),
            &format!("{got:.2} degrees, {:.3} a unit", got / bend),
            (got - want).abs() <= 1.0,
        );
    }

    t.heading("The fixtures, at the reference's numbers (tolerance 2e-5)");
    t.fixtures("expected_bend_it.json");

    t.heading("An old project");
    let now = fs::read_to_string(t.root.join("fx_bendit_003.json")).unwrap();
    let old = now.replace("\"bend\": 50", "\"bend\": 90");
    let loaded = persist::load_str(&old).expect("the old fx_bendit_003.json opens");
    let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
    t.row(
        "fx_bendit_003.json as saved before D-836 (Bend 90, its file at c4b0696c) opens with no warning",
        &format!("{on_open:?}"),
        old != now && on_open.is_empty(),
    );
    let written: J = serde_json::from_str(&old).unwrap();
    let same = effect_table::same_json(&effect_table::saved(&loaded), &written);
    t.row(
        "and saves back holding what it held: Bend still 90, nothing added",
        if same { "the same" } else { "differs" },
        same,
    );
    let expected: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_bend_it.json")).unwrap()).unwrap();
    let mut log = FrameLog::new(8);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, &t.root, 64, &mut log).unwrap();
    let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
    let d = largest_difference(&frame, &expected["cases"]["FX-BENDIT-003"]["frames"]["0"]);
    t.row(
        "and bends further (162 degrees), not FX-BENDIT-003's quarter turn, with no warning",
        &format!("largest difference from FX-BENDIT-003 {d:.1e}; warnings {said:?}"),
        d > 1e-3 && said.is_empty(),
    );
    let old_turn = turn(&shot(90.0));
    t.row(
        "Bend 90 on the bar turns it 162 degrees, within 1",
        &format!("{old_turn:.2} degrees"),
        (old_turn - 162.0).abs() <= 1.0,
    );

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cases: Vec<(String, anime_compositor::model::Project, std::path::PathBuf)> = Vec::new();
    for bend in [25.0, 100.0, -50.0, 150.0] {
        let project = persist::load_str(&shot(bend)).unwrap().document.project().clone();
        cases.push((format!("The bar at Bend {bend}"), project, std::path::PathBuf::from(".")));
    }
    for n in [1u32, 5, 8, 10, 16, 17] {
        let name = format!("fx_bendit_{n:03}.json");
        let p = t.load(&name).document.project().clone();
        cases.push((name, p, t.root.clone()));
    }
    for (name, project, root) in &cases {
        let (mut largest, mut refused, mut agree) = (0, false, true);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let (d, r, a, b) = both(&mut gpu, project, &Id::new(MAIN), root, 0, quality);
            largest = largest.max(d.0);
            refused |= r;
            agree &= a == b;
        }
        t.row(
            &format!("{name}, frame 0 at Full and Draft"),
            &format!("largest difference {largest} of 255; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree,
        );
    }

    t.finish("B-716_bend_it_turn_table.md");
}
