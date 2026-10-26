//! B-749 (D-869): in 8 bpc (After Effects) the card lays an adjustment layer and a Light Wrap,
//! as the CPU does.
//!
//! Every fixture project holding an adjustment layer or a Light Wrap, each composition set to
//! 8 bpc, is drawn by the CPU and by the card at its first and middle frames, Full and Draft.
//! The frames the card draws are held to wave 8's bar; a frame the card still refuses, for a
//! reason other than 8 bpc, must be the CPU's exactly.
//!
//! Writes `verification/B-749_eight_adjust_wrap_table.md`.

use anime_compositor::gpu::Gpu;

mod effect_table;
mod eight_walk;
use effect_table::Table;

#[test]
fn b749_eight_adjust_wrap() {
    let mut gpu = Gpu::new().expect("a usable card");
    let adjust = |t: &str| t.contains("\"kind\": \"adjustment\"");
    let wrap = |t: &str| t.contains("\"core.light_wrap\"");
    let w = eight_walk::walk(
        &mut gpu,
        |t| adjust(t) || wrap(t),
        |_| {},
        |t| {
            let mut g = Vec::new();
            if adjust(t) {
                g.push("An adjustment layer".to_string());
            }
            if wrap(t) {
                g.push("A Light Wrap".to_string());
            }
            g
        },
    );
    let mut t = Table::new(
        "light_wrap",
        &format!(
            "# B-749 (D-869): adjustment layers and Light Wrap in 8 bpc on the card\n\n\
             Card: {}. Every fixture project holding an adjustment layer or a Light Wrap, each \
             composition set to 8 bpc (After Effects), drawn by the CPU and by the card at its \
             first and middle frames, Full and Draft. The bar: within 1 level of 255 on at least \
             99.9% of pixels and within 2 everywhere. A frame the card refuses must be the CPU's \
             exactly, and none for being 8 bpc.\n",
            gpu.about()
        ),
    );
    t.heading("The frames the card drew");
    for g in ["An adjustment layer", "A Light Wrap"] {
        let s = w.drawn.get(g).cloned().unwrap_or_default();
        t.row(g, &s.text(false), s.frames > 0 && s.ok());
    }
    t.heading("The frames the card refused, by its reason");
    for (why, (n, exact)) in &w.refused {
        let eight = why.contains("8 bpc");
        t.row(
            why,
            &format!("{n} frames, {exact} of them the CPU's exactly"),
            n == exact && !eight,
        );
    }
    t.heading("All the frames");
    let worst = if w.worst.is_empty() { "none".to_string() } else { w.worst.join("; ") };
    t.row(
        &format!("{} fixture projects", w.shots),
        &format!("frames with a pixel over 1 level: {worst}"),
        w.drawn.values().any(|s| s.frames > 0),
    );
    t.finish("B-749_eight_adjust_wrap_table.md");
}
