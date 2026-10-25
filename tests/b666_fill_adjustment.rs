//! B-666 (D-786): Fill on an adjustment layer fills the frame beneath it.

mod effect_table;

use effect_table::Table;

#[test]
fn b666_fill_adjustment() {
    let mut t = Table::new(
        "fill_adjustment",
        "# D-786: Fill on an adjustment layer\n\nB-666, after Ben Marriott's \"2D Smoke & Explosions\" \
         (9:27 to 9:45), where Fill on the Effects adjustment layer turns the grey cloud strokes white: \
         a Fill with Fill Mask 0 and All Masks off on an adjustment layer fills everything beneath it \
         by D-418's rule, mixed back by D-65's coverage and opacity. Since D-830 one that names a \
         mask fills inside that mask of the adjustment layer (FX-FILLADJ-007, 008, 010, 011); one \
         naming a mask that is not there fills nothing and says EFFECT_PATH_MISSING (012). Before B-666 the frame was left as it was \
         and nothing was said. Every expected pixel is \
         `Fixtures/fill_adjustment/expected_fill_adjustment.json`, written by \
         `tools/fill_adjustment_reference.py` before this code existed and printed in document 25 as \
         FX-FILLADJ-001 to 012. Tolerance 2e-5.\n",
    );
    t.heading("FX-FILLADJ-001 to 012 (document 25)");
    t.fixtures_warned_at("expected_fill_adjustment.json", 1..=12, 0);

    t.heading("The file");
    // 004, 007 and 008 write their mask in the old single `mask` record (as `Fixtures/adjust`
    // does), which opens through document 07's migration and is saved as a `masks` list, so they
    // are the old-project load check: their pixels above are drawn from that old record.
    let files: Vec<String> = [1, 2, 3, 5, 6, 9, 10, 11, 12]
        .iter()
        .map(|n| format!("fx_filladj_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    // An old project (D-418's own Fill fixture, written before D-786) opens and draws as pinned.
    t.heading("An old project");
    t.root = effect_table::repo("Fixtures/fill");
    t.fixtures_numbered("expected_fill.json", 1..=1);
    t.finish("B-666_fill_adjustment_table.md");
}
