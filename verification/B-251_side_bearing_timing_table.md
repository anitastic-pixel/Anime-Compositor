# B-251: laying out and drawing a line of text, before and after the side-bearing slide

Measured on 2026-10-09, after the code commit (1af43d4), on the code that commit holds. No other
cargo or rustc process was running: checked before every round and after the last, none. The
owner's app was not open. The timing test is B-250's `d371_text_shaping_timing` in
`tests/d371_text_shaping.rs`, built with `cargo test --release --test d371_text_shaping --no-run`
and its executable run directly with `--ignored d371_text_shaping_timing --exact`. "Before" is the
same test built with `src/text.rs` as 11d870e holds it (before D-372's slide), release build, its
executable copied aside; the rounds took turns, before then after, three times.

- Processor: AMD Ryzen 9 9900X, 24 threads
- Card: NVIDIA GeForce RTX 4070 Ti SUPER (not used: text is drawn on the processor)
- System: Windows 11
- Build: release

**What is timed.** As in `verification/B-250_text_shaping_timing_table.md`: "Typewriter effect, 30
letters!" in the bundled M PLUS Rounded 1c and a 31-character Arabic line in Segoe UI, both
TrueType fonts with a glyf table, so every glyph now reads its side bearing and header box. Size
100, no animator, kerning off. **Laid out** is `text::placed`; **drawn** is `text::draw` into a
1920 by 1080 picture. Each figure is the median of 200 runs after one to warm up; the table gives
the median of the three rounds, in milliseconds.

| Line | Laid out, before | Laid out, after | Drawn, before | Drawn, after |
|---|---:|---:|---:|---:|
| 30-character Latin line | 0.249 | 0.254 | 1.530 | 1.491 |
| 31-character Arabic line | 0.310 | 0.342 | 1.679 | 1.730 |

Rounds (before / after, laid out): Latin 0.246, 0.255, 0.249 / 0.249, 0.254, 0.254; Arabic
0.336, 0.310, 0.309 / 0.343, 0.342, 0.312. Drawn: Latin 1.476, 1.532, 1.530 / 1.459, 1.491,
1.498; Arabic 1.704, 1.679, 1.642 / 1.757, 1.730, 1.648.

The differences are within the spread of the rounds: reading two numbers from the font per glyph
costs nothing that this measurement can see.
