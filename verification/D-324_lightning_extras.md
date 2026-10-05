# D-324 / B-203: Advanced Lightning extras

From D-308, approved by the owner on 2026-10-04, after P-26. In tutorial 2 (Advanced Electric), the bolt is After Effects' **Advanced Lightning** and ends on the ground, because the ground is an **Alpha Obstacle**. D-190's Lightning Bolt had one kind of bolt and no obstacle, so tutorial 2's bolt ran on through the ground.

## What changed

- Lightning Bolt has a **Lightning Type** choice in Effect controls:
  - **Direction**: D-190's bolt, as before.
  - **Strike**: the forks reach on toward the end point.
  - **Breaking**: the forks start as bright as the bolt.
  - **Bouncy**: there, back and there again.
  - **Omni**: six bolts from the start, 60 degrees apart.
  - **Anywhere**: one bolt from the start to a new place every `hold` frames.
  - **Vertical**: straight down to the layer's bottom edge, the end point unused.
  - **Two-Way Striking**: one bolt from each end, meeting in the middle.
- Four new numbers, each keyable:
  - **Turbulence** (0 to 100): rougher, with more forks.
  - **Decay** (0 to 100): how much the bolt thins toward its end.
  - **Conductivity State** (0 to 10000): moving it moves the bolt smoothly through new shapes.
  - **Alpha Obstacle** (0 to 100): the bolt and its forks stop where the layer, as it was before the bolt, covers more than 1 - obstacle / 100.
- At their starting values (Direction, 0) they draw D-190's bolt exactly. A file without them draws as before and is saved as it was. Each is written only when moved, keyed or already in the file.
- A type that is not one of the eight is kept and reported, and the effect is left out (as D-46).

**What is not built, and why:**

- The types are this program's own reading of Adobe's one-line descriptions. No After Effects frame was compared, because Adobe's help page could not be read (it refused the request).
- After Effects' negative Alpha Obstacle, which keeps the bolt inside a shape, is not built.
- The bolt stops at an obstacle. It does not go round it.

## Checks (cargo test)

All in `verification/B-203_lightning_extras_table.md`, **128 of 128 pass**:

| Check | Expected | Got |
|---|---|---|
| FX-LIGHTX-001 to 024: obstacle, each type, turbulence, decay, conductivity, keyed | every pixel as `Fixtures/lightning_extras/expected_lightning_extras.json`, within 2e-5 | pass, largest difference 6.6e-8 |
| FX-LIGHTX-001: the five new settings written at their starting values | exactly FX-BOLT-001 | pass |
| FX-BOLT-001 to 032 (B-126), D-190's own bolt | unchanged | pass |
| The 24 files opened and saved | the same file back | pass |
| An old Lightning Bolt file | saved with none of the five new settings | pass |
| FX-LIGHTX-020, type "sideways" | kept in the file, the effect left out, a sentence naming the eight types | pass |
| A word for turbulence, or a number for the type | the file refused, as a fault in its shape | pass |
| A draft preview | only width and glow halved, as before | pass |
| Type "sideways", turbulence 101, decay -1, conductivity 10001, obstacle -1, obstacle keyed to 150 | refused with a sentence, nothing changes | pass |
| Each of the eight types, every number at its top, conductivity and obstacle keyed | taken; ten undos give back the frame byte for byte | pass |
| Frames in tiles of 1 and of 64 | byte for byte the same | pass |

## Pictures

In `verification/D-324 pictures/`.

Pictures `before.png` and 1 to 12 are drawn by the test (B-203). They show B-126's night scene, with a hill as a drawing of its own carrying the bolt. `sky.png` and `ground.png` are its two drawings.

Pictures `app_1` to `app_4` are from the app's test copy, not your app (`target/p26/d324_steps.js`):

- The scene is a new 640 by 360 composition "Storm": a dark sky, a brown strip of ground at the bottom, and a black Lightning solid.
- The Lightning solid has a mask over the ground strip, so the solid is there only where the ground is. That is what the bolt stops on.
- Your Unsaved work and window were put back after the run.

| Picture | What it shows | Look for | Pass? |
|---|---|---|---|
| `before.png` | The sky with the hill, no bolt | Sky at the top, hill at the bottom | |
| `1_no_obstacle.png` | Direction, Alpha Obstacle 0 | The bolt runs on through the hill to the bottom edge | |
| `2_obstacle_50.png` | Alpha Obstacle 50 | The bolt ends where it reaches the hill; nothing below the hill's top | |
| `3_strike.png` | Strike | Forks reaching on toward the bottom | |
| `4_breaking.png` | Breaking | Forks as bright as the bolt where they leave it | |
| `5_bouncy.png` | Bouncy | Three bolts over the same path, more crowded than picture 1 | |
| `6_omni.png` | Omni | Six bolts spreading out from the start | |
| `7_anywhere.png` | Anywhere | One bolt from the start to some other place | |
| `8_vertical.png` | Vertical | A bolt straight down to the bottom edge | |
| `9_two_way.png` | Two-Way Striking | Two bolts, one from each end, meeting in the middle | |
| `10_turbulence_100.png` | Turbulence 100 | Rougher than picture 1, with more forks | |
| `11_decay_100.png` | Decay 100 | Thinning to nothing toward its end | |
| `12_conductivity_0_5.png` | Conductivity 0.5 | Picture 1's bolt, part way to another shape | |
| `app_1_no_obstacle.png` | The app, a bolt from the top to the bottom, Alpha Obstacle 0 | The bolt runs on through the brown ground to the bottom edge; Effect controls lists Turbulence, Decay, Conductivity State and Alpha Obstacle under Seed | |
| `app_2_obstacle_50_stops_on_ground.png` | Alpha Obstacle 50 | The same bolt, ending where it reaches the top of the ground; nothing on the ground | |
| `app_3_omni_obstacle_50.png` | Lightning Type Omni, the start moved to the middle | Six bolts spreading from one point in the sky | |
| `app_4_undo_back_to_direction.png` | Ctrl+Z once | Back to picture `app_2`: one bolt ending on the ground | |

The app read back obstacle 50 after picture `app_2`, type omni after `app_3`, and no type (Direction) with obstacle 50 still set after the undo.

## For the owner to try

1. Make a solid, add **Lightning Bolt**, and drag the End Point below the bottom of the layer.
2. Draw a mask on the same solid over its lower part, so the solid is only there.
3. Raise **Alpha Obstacle** above 0: the bolt should stop where the mask begins.
4. Try each **Lightning Type**, and drag **Conductivity State** slowly: the bolt should change shape smoothly, not jump.
