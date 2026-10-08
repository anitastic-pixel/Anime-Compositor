# B-230: frame times with two text animators on a 30-character line

Measured on 2026-10-08, before the code commit, on the code that commit holds. No other cargo or
rustc process was running: checked before and after every round, none. The owner's app was not
open. The timing test is `b230_text_animator_timing` in `tests/b230_text_animator.rs`, built with
`cargo test --release --test b230_text_animator --no-run` and run three rounds in a row with
`--ignored b230_text_animator_timing --exact`, each round timing the three shots in the order
below.

The Text Animator is new, so there is no "before" build: the row without animators is the
comparison. The letters are drawn by the processor, as all text is; the viewer's frame is then
finished by the graphics card.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.**

- **The line.** FX-TXA-011: "Thirty characters on one line!", 30 characters, M PLUS Rounded 1c at
  100 pixels, in a 1920 by 1080 composition, with its two animators: a wave (Position 0, -60,
  Triangle) and a fade by character (Opacity 0, Ramp Up, eased). Both animators' Offsets are keyed
  from -100 at frame 0 to 100 at frame 47, so every one of the 48 frames is different.
- **Drawing the line** is `text::animated` alone, the processor's drawing of the words into a 1920
  by 1080 picture, with no animator, and with the two at each frame's values.
- **The viewer's whole frame** is the frame asked for whole, as the viewer asks for it
  (`preview_frame_srgb8`), at Full with Draw on: GPU, each frame new.
- **First** is the median of the first loop of 48 frames; **Again** the median of the next four
  loops, 192 frames.

Each figure is the median of the three rounds, in milliseconds a frame.

| Shot | First | Again |
|---|---:|---:|
| Drawing the line, no animator | 0.8 | 0.8 |
| Drawing the line, two animators | 4.1 | 4.1 |
| The viewer's whole frame, two animators, Full, Draw on: GPU | 16.7 | 18.3 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Drawing, no animator | 0.8 / 0.8 | 0.8 / 0.8 | 0.8 / 0.8 |
| Drawing, two animators | 4.0 / 3.9 | 4.1 / 4.1 | 4.2 / 4.2 |
| Viewer's whole frame | 16.5 / 17.9 | 16.7 / 18.3 | 17.0 / 18.6 |

**Against the target.** EFFECTS.md gives the Text Animator Target P2. Two animators add about
3.3 ms a frame to the drawing of a 30-character line (0.8 to 4.1): without animators every letter
is filled in one pass; with them, each group of letters sharing an opacity and colour is filled on
its own, and a fade by character gives nearly every letter its own opacity. The viewer's whole
frame with the two animators is 18.3 ms played again, inside a 24 frames a second budget (41.7 ms).
That "Again" is a little slower than "First" in every round was not looked into further. No
"whole frame without animators" was timed, so the viewer's figure is not split into its parts.
