# B-250: laying out and drawing a line of text, before and after shaping

Measured on 2026-10-09, after the code commit (cb2b749), on the code that commit holds. No other
cargo or rustc process was running: checked before every round and after the last, none. The
owner's app was not open. The timing test is `d371_text_shaping_timing` in
`tests/d371_text_shaping.rs`, built with `cargo test --release --test d371_text_shaping --no-run`
and its executable run directly with `--ignored d371_text_shaping_timing --exact`. "Before" is the
same function built from 209360c, the commit before B-250's fixtures, in a separate worktree with
its own build folder, release build. The rounds took turns, before then after, three times.

- Processor: AMD Ryzen 9 9900X, 24 threads
- Card: NVIDIA GeForce RTX 4070 Ti SUPER (not used: text is drawn on the processor)
- System: Windows 11
- Build: release

**What is timed.**

- **The lines.** "Typewriter effect, 30 letters!", 30 characters in the bundled M PLUS Rounded 1c,
  and "as-salamu alaykum wa rahmatullahi wa barakatuh" in Arabic, 31 characters, in Segoe UI (on
  every Windows machine; the before build cannot join it, so it draws the letters unjoined and
  left to right). Size 100 at 100, 540, no animator, kerning off, as a new text layer has it.
- **Laid out** is `text::placed`: shaping (after only), placing every letter, and its outline cut
  into straight pieces. **Drawn** is `text::draw`, all of that plus filling the outlines into a
  1920 by 1080 picture.
- Each figure is the median of 200 runs, after one run to warm up; the table gives the median of
  the three rounds, in milliseconds.

| Line | Laid out, before | Laid out, after | Drawn, before | Drawn, after |
|---|---:|---:|---:|---:|
| 30-character Latin line | 0.240 | 0.256 | 1.607 | 1.607 |
| 31-character Arabic line | 0.298 | 0.334 | 1.685 | 1.817 |

The rounds, laid out / drawn:

| Line | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Latin, before | 0.249 / 1.672 | 0.240 / 1.530 | 0.240 / 1.607 |
| Latin, after | 0.256 / 1.549 | 0.254 / 1.640 | 0.257 / 1.607 |
| Arabic, before | 0.309 / 1.685 | 0.298 / 1.661 | 0.298 / 1.760 |
| Arabic, after | 0.350 / 1.757 | 0.333 / 1.854 | 0.334 / 1.817 |

**What it means.** Shaping adds about 0.02 ms to laying out the Latin line and about 0.04 ms to
the Arabic one. Drawing the Latin line takes the same time as before. The Arabic line takes about
0.13 ms more to draw, because its joined letters are now drawn as Segoe UI's joined forms, a
different set of outlines from the unjoined ones. Either way a line costs under 2 ms on the
processor, well inside a 24 frames a second budget (41.7 ms). The round-to-round spread in "drawn"
(about 0.1 ms) is as large as the Latin difference, so the Latin "drawn" figures are the same
within what this measures.
