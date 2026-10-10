//! B-70: the turbulent displace in the core, against D-127.
//!
//! Writes `verification/B-70_turbulent_displace_table.md`.
//!
//! Every expected pixel is `Fixtures/turbulent_displace/expected_turbulent_displace.json`,
//! written by `tools/turbulent_displace_reference.py` before this code existed and printed in
//! document 25 as FX-TURB-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// `[amount, size, complexity, evolution, speed, seed]` and the edges.
fn turb(n: [f64; 6], edges: &str) -> Effect {
    Effect::TurbulentDisplace {
        amount: n[0],
        size: n[1],
        complexity: n[2],
        evolution: n[3],
        speed: n[4],
        seed: n[5],
        edges: edges.to_string(),
        frame: 0,
        displacement: "turbulent".to_string(),
        pinning: "none".to_string(),
        units: "classic".to_string(),
        new_seed_every: 0.0,
    }
}

const START: [f64; 6] = [10.0, 60.0, 2.0, 0.0, 20.0, 0.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    turb(n, "transparent")
}

#[test]
fn b70_turbulent_displace() {
    let mut t = Table::new(
        "turbulent_displace",
        "# B-70: turbulent displace\n\nD-127, accepted by the owner on 2026-09-26, the fifth of \
         the second batch of ten. Every expected pixel is \
         `Fixtures/turbulent_displace/expected_turbulent_displace.json`, written by \
         `tools/turbulent_displace_reference.py` before this code existed and printed in \
         document 25 as FX-TURB-001 to 026. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance \
         of 2e-5.\n",
    );

    t.heading("FX-TURB-001 to 026 (document 25)");
    t.fixtures("expected_turbulent_displace.json");

    t.heading("How far it reaches");
    let got = with(0, 2.5).bounds_expansion();
    t.row(
        "with transparent edges, amount 2.5 grows the drawing's bounds by 3 pixels",
        &got.to_string(),
        got == 3,
    );
    let got = turb([2.5, 60.0, 2.0, 0.0, 20.0, 0.0], "repeat").bounds_expansion();
    t.row("with repeat edges it grows them by nothing", &got.to_string(), got == 0);
    let got = with(0, 0.0).bounds_expansion();
    t.row("at amount 0 it grows them by nothing", &got.to_string(), got == 0);
    let mut draft = START;
    draft[0] = 20.0;
    let mut draft = turb(draft, "transparent");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the amount and the size",
        &format!("{draft:?}"),
        draft == turb([10.0, 30.0, 2.0, 0.0, 20.0, 0.0], "transparent"),
    );
    let mut draft = with(1, 1.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft of size 1 holds the size at 1, its range's bottom, rather than \
         leaving the effect out",
        &format!("{draft:?}"),
        draft == turb([5.0, 1.0, 2.0, 0.0, 20.0, 0.0], "transparent"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_turb_001.json",
        "fx_turb_005.json",
        "fx_turb_013.json",
        "fx_turb_014.json",
        "fx_turb_015.json",
        "fx_turb_016.json",
        "fx_turb_017.json",
        "fx_turb_018.json",
        "fx_turb_019.json",
        "fx_turb_022.json",
        "fx_turb_024.json",
        "fx_turb_025.json",
        "fx_turb_026.json",
    ]);
    t.shape_refused(
        "fx_turb_001.json",
        "no `edges` at all",
        r##"{"amount": 10, "size": 60, "complexity": 2, "evolution": 0, "speed": 20, "seed": 0}"##,
    );
    t.shape_refused(
        "fx_turb_001.json",
        "a speed that is a word",
        r##"{"amount": 10, "size": 60, "complexity": 2, "evolution": 0, "speed": "fast", "seed": 0, "edges": "transparent"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_turb_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 1001", set(with(0, 1001.0))),
            ("size 0.5", set(with(1, 0.5))),
            ("complexity 9", set(with(2, 9.0))),
            ("evolution 100001", set(with(3, 100001.0))),
            ("speed -361", set(with(4, -361.0))),
            ("seed 100001", set(with(5, 100001.0))),
            ("edges \"wrap\"", set(turb(START, "wrap"))),
            ("amount keyed to 1500", keys("amount", &[(0, &[10.0]), (4, &[1500.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_turb_001.json",
        vec![
            (
                "amount 1000, size 1000, complexity 8, evolution 100000, speed 360 and seed \
                 100000, the tops,",
                set(turb([1000.0, 1000.0, 8.0, 100000.0, 360.0, 100000.0], "repeat")),
            ),
            (
                "amount 0, size 1, complexity 1, evolution -100000, speed -360 and seed 0, the \
                 bottoms,",
                set(turb([0.0, 1.0, 1.0, -100000.0, -360.0, 0.0], "transparent")),
            ),
            ("speed keyed from 20 to -20", keys("speed", &[(0, &[20.0]), (4, &[-20.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_turb_003.json", 2), ("fx_turb_018.json", 0)]);

    t.finish("B-70_turbulent_displace_table.md");
}
