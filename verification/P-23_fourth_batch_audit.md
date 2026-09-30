# P-23: the performance audit of the fourth batch

**Everything built since P-22, B-115 to B-150, was timed on 2026-09-29: the twenty-five new effects one at a time, and on the reference shot the effects that read another layer or other frames, motion blur, time stretch, Frame Mix and the drawing dissolve. One slow spot was made faster without moving a single pixel: motion blur. The reference shot with two layers sliding under a 180-degree, 16-sample shutter took 312.0 ms a frame and now takes 68.8 ms, 4.5 times faster; the same slide without motion blur takes 45.3 ms, so the blur's own cost went from about 266 ms to about 24 ms. All 229 of 229 results, the older effects' included, have the same fingerprint before and after.**

Asked for by the owner on 2026-09-29: "sweet! it works! motion blur, and let's do a performance audit along with gpu-acceleration please." This page is the audit on the processor. Moving the batch onto the graphics card is B-151, which follows it; motion blur and Frame Mix on the card are a separate study and are not in either.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads; 61.5 GB of memory
- Graphics card: NVIDIA GeForce RTX 4070 Ti SUPER, driver 32.0.16.1088 (NVIDIA 610.88; P-22 ran on 610.74), through Vulkan. It is not used by these numbers.
- OS: Microsoft Windows 11 Education, 10.0.26300 (build 26300)
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Pool: rayon's default, one thread per hardware thread
- **The owner's copy of the app was not open** for either run, and no other cargo or rustc process was running when each run started or when it ended (checked with `tasklist`). The screen was locked (`LockApp.exe` running). A read-only research agent was running at the same time; it does not build.
- `tests/p16_effect_cost.rs`: 7 runs a case, median reported, run with `cargo test --release --test p16_effect_cost -- --ignored --nocapture --test-threads=1`. **Before**: commit b8ac2f2 with this audit's new cases added and nothing else changed. **After**: the commit that carries this page. Both on the same evening, 2026-09-29.

## What was changed

- **Motion blur** (`src/compose.rs`, `settle`). A motion-blurred layer is drawn at each of its 16 moments and the drawings are averaged. Each moment used to be drawn as a whole 1920x1080 frame, a new 33 MB picture each time, and then added into the average one number at a time on a single thread. Now each moment is added straight into the average, pixel by pixel, with exactly the same arithmetic the frame drawing does, and only where the drawing has anything in it: a character cel is mostly empty, and an empty part of a moment is exactly zero, which adds nothing. The additions that do happen are the same ones in the same order, so the average is the same to the last bit. It is the same for the Full and the Draft preview and for a motion-blurred matte.
- **Bevel Alpha**: one small step, clearing the colour before the covering is blurred, is shared between the threads. Its times moved by no more than the usual spread from one run to the next, so it is not counted as a saving.
- Everything else was already spread over every thread, or its cost is work that cannot be skipped without changing the answer, and it is unchanged.

## The twenty-five effects, one at a time

As in P-20 to P-22: **character** is a 1920x1080 figure in flat colours on nothing, **background** is an opaque plate, and the fingerprint is the first 16 hex digits of the SHA-256 of the effect's result, every float by its bits. Median and Smart Blur are timed at their starting radius and at radius 10; Radial Shadow at its start and at softness 20. The last row is Mix, B-137, which works on every effect. `72cac54dd11e4165` is the character unchanged: HSV Key, Change to Color and Bevel Edges at their starting settings find nothing on it to change. The spread between two runs of the same unchanged case is up to about 20% on cases this short (Gaussian Blur 10 at Mix 50% went 17.3 then 14.2 ms with no change to it at all).

| Effect and settings | Picture | Before median ms | After median ms | Before fastest ms | After fastest ms | Frame SHA-256 (first 16) | After |
|---|---|---|---|---|---|---|---|
| Color Lookup, cool_3.cube | character | 0.7 | 0.8 | 0.7 | 0.7 | `8814645e045fd151` | same |
| Line Blur, as it starts | character | 10.0 | 10.2 | 9.7 | 9.8 | `b828a2210a7073ed` | same |
| HSV Key, as it starts | character | 1.8 | 1.9 | 1.6 | 1.7 | `72cac54dd11e4165` | same |
| Paraffin, as it starts | character | 4.4 | 4.5 | 4.3 | 4.3 | `b397e22b15fd36f3` | same |
| Kira-kira, as it starts | character | 4.5 | 4.9 | 4.4 | 4.6 | `4e13df6642d6e4da` | same |
| Lightning Bolt, as it starts | character | 2.3 | 2.4 | 2.2 | 2.3 | `d77e18dd6a173230` | same |
| Change to Color, as it starts | character | 0.8 | 0.9 | 0.8 | 0.8 | `72cac54dd11e4165` | same |
| Corner Pin, one corner pulled in | character | 4.2 | 4.1 | 4.0 | 4.1 | `b2131bd26d075d87` | same |
| Light Sweep, as it starts | character | 5.2 | 5.1 | 4.9 | 4.9 | `058be4b749cdb295` | same |
| Radio Waves, at frame 48 | character | 2.1 | 2.1 | 2.0 | 2.0 | `015065fde8aca047` | same |
| Polar Coordinates, as it starts | character | 6.3 | 6.0 | 6.0 | 5.8 | `82b96579b9bb9e73` | same |
| Median, radius 2, as it starts | character | 8.1 | 7.6 | 7.6 | 7.5 | `d7fbafe9168b7ed9` | same |
| Median, radius 10 | character | 98.6 | 98.8 | 97.9 | 97.2 | `8e67b25d501f8372` | same |
| Smart Blur, radius 3, as it starts | character | 6.4 | 6.6 | 6.2 | 6.3 | `fe8479c8ab91dd76` | same |
| Smart Blur, radius 10 | character | 34.8 | 35.0 | 33.3 | 34.6 | `8d4af9d66908f2e0` | same |
| Snowfall, as it starts | character | 3.8 | 3.4 | 3.4 | 3.2 | `38f96c8549916f44` | same |
| Kaleidoscope, as it starts | character | 7.8 | 6.6 | 6.8 | 6.5 | `b592b70e8f279128` | same |
| Roughen Edges, as it starts | character | 11.3 | 10.4 | 10.8 | 10.0 | `13feb44f7f43f90a` | same |
| Beam, as it starts | character | 2.6 | 2.4 | 2.5 | 2.3 | `ab2ca932d8a1d43c` | same |
| 4-Color Gradient, as it starts | character | 2.4 | 2.2 | 2.1 | 2.0 | `e52d6158735b5407` | same |
| Cell Pattern, as it starts | character | 3.1 | 2.9 | 3.0 | 2.8 | `61835f1c9b097eeb` | same |
| Optics Compensation, field of view 60 | character | 5.8 | 5.3 | 5.8 | 5.1 | `f061a42b3ddd67c8` | same |
| Radial Shadow, as it starts | character | 10.4 | 9.4 | 9.8 | 8.9 | `75975ac52975838f` | same |
| Radial Shadow, softness 20 | character | 24.3 | 20.9 | 23.0 | 20.6 | `6081ff405f506a0a` | same |
| Extract, black point 64, softness 32 | character | 0.9 | 0.8 | 0.9 | 0.8 | `266303c537ec63c6` | same |
| Bevel Alpha, as it starts | character | 15.4 | 12.7 | 14.9 | 12.4 | `398c7b78e81a686f` | same |
| Bevel Edges, as it starts | character | 1.0 | 0.8 | 1.0 | 0.8 | `72cac54dd11e4165` | same |
| Block Dissolve, half way, 8-pixel blocks | character | 3.7 | 3.4 | 3.3 | 3.3 | `09d856de69929541` | same |
| Gaussian Blur 10 at Mix 50% | character | 17.3 | 14.2 | 15.8 | 13.5 | `508008082950dcd2` | same |
| Color Lookup, cool_3.cube | background | 0.9 | 0.8 | 0.8 | 0.8 | `8582fdbdeb984ded` | same |
| Line Blur, as it starts | background | 17.9 | 18.4 | 17.5 | 17.9 | `a5a526b581265a57` | same |
| HSV Key, as it starts | background | 5.8 | 5.7 | 5.4 | 5.5 | `46c8c1224f54e229` | same |
| Paraffin, as it starts | background | 12.4 | 12.5 | 12.2 | 12.3 | `88fc35db0e977d5f` | same |
| Kira-kira, as it starts | background | 6.9 | 7.1 | 6.7 | 6.6 | `881f1187a95d20ff` | same |
| Lightning Bolt, as it starts | background | 2.1 | 2.0 | 2.1 | 1.9 | `4f6c78ac23919b16` | same |
| Change to Color, as it starts | background | 0.9 | 0.9 | 0.8 | 0.8 | `5e8c26a9f8d4132a` | same |
| Corner Pin, one corner pulled in | background | 4.0 | 4.0 | 4.0 | 4.0 | `04065badfa41a43d` | same |
| Light Sweep, as it starts | background | 5.1 | 5.3 | 4.9 | 5.0 | `c0dd4b0e19eb1860` | same |
| Radio Waves, at frame 48 | background | 2.1 | 2.1 | 1.9 | 2.0 | `8bd7d3be2a327e45` | same |
| Polar Coordinates, as it starts | background | 6.0 | 6.0 | 5.9 | 5.8 | `015301e6cb674d77` | same |
| Median, radius 2, as it starts | background | 11.4 | 11.7 | 11.1 | 11.2 | `6831be2f2b270dde` | same |
| Median, radius 10 | background | 196.7 | 196.6 | 193.4 | 193.9 | `2d84b81da3286bd5` | same |
| Smart Blur, radius 3, as it starts | background | 18.2 | 18.8 | 17.9 | 18.4 | `e572cbc79cb9ac69` | same |
| Smart Blur, radius 10 | background | 141.6 | 144.6 | 140.3 | 143.7 | `de6208cdf33a0e8e` | same |
| Snowfall, as it starts | background | 12.8 | 12.9 | 12.7 | 12.7 | `0374e7ffd2c7985d` | same |
| Kaleidoscope, as it starts | background | 6.7 | 6.7 | 6.4 | 6.4 | `132c345ac4e760db` | same |
| Roughen Edges, as it starts | background | 28.9 | 28.9 | 28.4 | 28.4 | `03d5940840595f27` | same |
| Beam, as it starts | background | 2.5 | 2.4 | 2.4 | 2.4 | `fb69c3707e890e76` | same |
| 4-Color Gradient, as it starts | background | 6.9 | 6.9 | 6.7 | 6.7 | `8a94c6da474aac38` | same |
| Cell Pattern, as it starts | background | 10.7 | 10.6 | 10.2 | 10.2 | `a2cff1d49bf56c92` | same |
| Optics Compensation, field of view 60 | background | 5.3 | 5.5 | 5.1 | 5.1 | `7b5a81bfceef5cad` | same |
| Radial Shadow, as it starts | background | 9.6 | 9.5 | 9.4 | 9.0 | `f41aa61d2d29a396` | same |
| Radial Shadow, softness 20 | background | 22.1 | 22.3 | 21.5 | 21.7 | `35c770ccb8d7da8c` | same |
| Extract, black point 64, softness 32 | background | 0.8 | 0.8 | 0.8 | 0.8 | `5e8c26a9f8d4132a` | same |
| Bevel Alpha, as it starts | background | 14.3 | 14.4 | 13.7 | 13.9 | `3402ce42ac175eed` | same |
| Bevel Edges, as it starts | background | 1.3 | 1.2 | 1.2 | 1.1 | `64b85b9baddd5c26` | same |
| Block Dissolve, half way, 8-pixel blocks | background | 3.2 | 3.1 | 3.1 | 2.9 | `30a348ceb8806ed1` | same |
| Gaussian Blur 10 at Mix 50% | background | 17.3 | 18.4 | 16.7 | 17.6 | `e0144b1f4537b073` | same |

## The effects that read other layers or other frames

These draw the reference shot at frame 101 the way export draws a frame (`render_frame`, Full), so each run also reads the drawings from their files; the first row, with nothing added, is that part. At frame 101 layer 3 (on twos) and layer 4 (on threes) are each on the last frame of a drawing, where the drawing dissolve shows. The slide moves layers 3 and 4 400 pixels across between frames 90 and 110. The fingerprint is of the whole frame.

| Case | Before median ms | After median ms | Before fastest ms | After fastest ms | Frame SHA-256 (first 16) | After |
|---|---|---|---|---|---|---|
| the reference shot, nothing added | 45.3 | 46.8 | 44.1 | 44.9 | `95319195210fcdd3` | same |
| Compound Blur 20 on layer 1, reading layer 2 | 141.9 | 150.9 | 137.5 | 145.4 | `b06d486ffd185640` | same |
| Displacement Map as it starts on layer 1, reading layer 2 | 66.8 | 67.4 | 64.3 | 66.5 | `ea35642c6f73e0b0` | same |
| Gradient Wipe half way, softness 10, on layer 1, reading layer 2 | 60.1 | 61.8 | 57.7 | 60.1 | `9ea127aed4b2ca5f` | same |
| Echo on layer 3, 4 echoes 2 frames apart | 119.4 | 119.9 | 113.9 | 115.3 | `5b163bb4e4c654ec` | same |
| Posterize Time 12 on layer 2 | 46.1 | 46.2 | 44.1 | 45.1 | `3fa9b15126666ffd` | same |
| layers 3 and 4 sliding, no motion blur | 45.7 | 45.3 | 43.6 | 44.4 | `48b58b41331ea2c5` | same |
| **layers 3 and 4 sliding, motion blur 180 degrees, 16 samples** | **312.0** | **68.8** | 309.0 | 68.4 | `c34db56b03e9ccef` | same |
| layer 2 at time stretch 150%, no blending | 44.8 | 45.4 | 44.0 | 44.0 | `46cfc57697ff2a7a` | same |
| layer 2 at time stretch 150%, Frame Mix | 60.0 | 61.3 | 59.4 | 58.7 | `54cea602b8725989` | same |
| drawing dissolve of 2 on layers 3 and 4 | 59.8 | 60.7 | 58.8 | 59.1 | `6bd0dd28107db462` | same |

The other 160 cases, the older effects, were run too. Their pictures are also unchanged, none of their code was touched, and their times moved only by the usual spread.

## What is left, and not done here

- **Median at radius 10** costs 98.8 ms on a character and 196.6 on a plate, and **Smart Blur at radius 10** 35.0 and 144.6. Every pixel looks at about 300 neighbours. Median has to find the middle of them, and Smart Blur adds them in a set order that fixes its last bit; neither has a shortcut that keeps every bit. At their starting radius they cost 8 to 19 ms. The card is the way past this, in B-151.
- **Compound Blur** adds about 100 ms to a frame at a largest blur of 20: it blurs the whole plate five times. Each blur is already P-20's fastest one.
- **Echo** adds about 75 ms here, most of it reading four more drawings from their files. In the app those drawings are kept once read, so scrubbing back and forth pays much less; export reads them as here.
- **Roughen Edges** (10 to 29 ms), **Radial Shadow at softness 20** (about 21 ms), **Line Blur** (10 to 18 ms), **Bevel Alpha** (13 to 14 ms), and **Paraffin, Snowfall and Cell Pattern** on a plate (11 to 13 ms) are work per pixel that cannot be skipped. B-151 takes the ones the card suits.
- Motion blur on the card and Frame Mix on the card are a separate architecture study.

## How to check this

1. Open the app on the reference shot, give layers 3 and 4 a slide, switch motion blur on for them and for the composition, and play or scrub it: it should keep up much better than before.
2. The picture should look exactly as it did before this change. `verification/B-124b_motion_blur_table.md`, the motion blur's own checks, was written again by this build and did not change by a single character.
