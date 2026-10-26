//! B-774 (D-894): the card-against-CPU checks do not grade the colour of a pixel fully
//! transparent on both sides. This checks the rule itself, pixel by pixel, over black and over
//! white; B-688, B-748, B-749 and B-750's tables show what it changes on the fixtures.

mod effect_table;

use effect_table::{hidden, seen, shown, Table};

#[test]
fn b774_unseen_pixels() {
    let mut t = Table::new(
        "eight_bpc",
        "# B-774 (D-894): pixels nobody can see are not graded\n\n\
         The rule, for the card's straight 8-bit viewer picture against the CPU's: where both \
         alpha bytes are 0, the colours are not compared. Each pair is also composited over \
         black and over white, as a viewer would see it, rounded to 8 bits.\n",
    );
    t.heading("The rule against what a viewer sees");

    // Every pair of hidden colours with both alpha bytes 0: not graded, and nothing to see.
    let (mut pairs, mut worst_seen, mut worst_shown) = (0usize, 0u8, 0u8);
    for c in 0..=255u8 {
        for d in (0..=255u8).step_by(5) {
            let (p, q) = ([c, 255 - c, d, 0], [d, c, 255 - d, 0]);
            pairs += 1;
            worst_seen = worst_seen.max(seen(&p, &q));
            worst_shown = worst_shown.max(shown(&p, &q));
        }
    }
    t.row(
        &format!("{pairs} pairs of different hidden colours, both alpha 0"),
        &format!("graded difference {worst_seen}; largest difference over black or white {worst_shown}"),
        worst_seen == 0 && worst_shown == 0,
    );

    // One side alpha 0, the other alpha 1 of 255: still graded on all four channels.
    let (p, q) = ([255, 0, 0, 0], [0, 0, 255, 1]);
    t.row(
        "alpha 0 against alpha 1, colours 255 apart",
        &format!("graded difference {}; excused {}", seen(&p, &q), hidden(&p, &q)),
        seen(&p, &q) == 255 && !hidden(&p, &q),
    );

    // Both visible at alpha 1: graded too, though the viewer sees at most 1 level of it.
    let (p, q) = ([255, 0, 0, 1], [0, 0, 0, 1]);
    t.row(
        "both alpha 1, colours 255 apart",
        &format!("graded difference {}; over black or white {}", seen(&p, &q), shown(&p, &q)),
        seen(&p, &q) == 255 && shown(&p, &q) == 1,
    );

    // Equal pixels are 0 either way.
    let p = [10, 20, 30, 0];
    t.row("the same hidden colour", &format!("graded difference {}; excused {}", seen(&p, &p), hidden(&p, &p)), seen(&p, &p) == 0 && !hidden(&p, &p));

    t.finish("B-774_unseen_pixels_table.md");
}
