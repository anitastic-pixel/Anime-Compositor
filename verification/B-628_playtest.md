# B-628 playtest: Curl Noise's look after After Effects (D-749)

Audit rows 8, 13 and 14. Curl Noise is a 1:1 After Effects effect. It now draws large, soft marbling carried along the flow, where before it drew a fine grey hatching.

## What to try in the app

1. Put a solid on a layer and add Curl Noise. **Expect:** large, soft, high-contrast marbling, like the left of each sheet below. Before this build it was fine grey hatching.
2. Raise **Size** from 100 to 300. **Expect:** the marbling grows larger and smoother (row 8).
3. Raise **Sample Radius** from 0 to 80. **Expect:** the swirls stretch further along the flow (row 8).
4. Move **Flow Softness** from 0 to 100. **Expect:** higher is crisper. Before this build, higher was softer (row 13).
5. Key **Speed** from 0 at the start to 40 a few frames later and play it. **Expect:** the noise speeds up smoothly from where it is. Before, it jumped (row 14).
6. Open a project saved before this build that has Curl Noise. **Expect:** it opens with the same settings, but it **looks different** (the marbling, not the hatching). That is intended (D-749).

## Three-way sheets: After Effects / before / after

In `verification/B-628 pictures/`, each sheet shows After Effects on the left, the build before in the middle and this build on the right:

- `7_sheet_desh_sr0.png`: DeshCloud, Sample Radius 0.
- `7_sheet_desh_sr42.png`: DeshCloud, Sample Radius 42.
- `7_sheet_stern_s40.png`: SternFX, Scale 40.

Each sheet has three numbers:
- **Spread:** how far the greys spread. Higher means more contrast.
- **Across** and **down:** how big the pattern is, in comp pixels.

**Verdicts:**
- **matches:** each number is within 1.25 times After Effects'.
- **close:** each is within 1.5 times.
- **differs:** otherwise.

Two cases were used to fit the numbers. The other seven were only checked.

| Tutorial case | Fit or check | After Effects | Before | After | Verdict before | Verdict after |
|---|---|---|---|---|---|---|
| fimje, defaults | fit | 59 / 126 x 69 | 17 / 30 x 3 | 67 / 159 x 72 | differs | close |
| DeshCloud, Sample Radius 0 | fit | 98 / 294 x 140 | 30 / 36 x 38 | 77 / 275 x 154 | differs | close |
| DeshCloud, Sample Radius 42 | check | 73 / 255 x 104 | 22 / 16 x 3 | 71 / 220 x 113 | differs | matches |
| DeshCloud, Sample Radius 80 | check | 62 / 123 x 69 | 17 / 31 x 3 | 67 / 160 x 72 | differs | close |
| SternFX, Scale 100 | check | 57 / 125 x 65 | 18 / 28 x 2 | 67 / 158 x 72 | differs | close |
| SternFX, Scale 40 | check | 53 / 53 x 26 | 11 / 21 x 5 | 60 / 63 x 30 | differs | matches |
| Vijay Gupta, defaults | check | 59 / 126 x 69 | 17 / 30 x 3 | 67 / 159 x 72 | differs | close |
| Jake In Motion, frame 46 | check | 60 / 182 x 77 | 16 / 29 x 3 | 68 / 166 x 73 | differs | matches |
| Softoni, frame 27 | check | 54 / 182 x 70 | 18 / 27 x 2 | 68 / 167 x 70 | differs | close |

Each entry gives the spread, then the pattern size across x down.

## Where it still differs from After Effects

- **Busier finest swirls:** our smallest swirls are busier than After Effects'. You can see it on the sheets; the numbers do not catch it.
- **Different defaults:** After Effects' defaults differ from ours (Swirl 180, Sample Count 3, Sample Radius 80, Flow Softness 25, Speed 0, Turbulence Speed 10). They are not changed here.
- **Fit sources:** the fit is to YouTube-compressed tutorial screenshots, not to After Effects itself.

## Checks

The checks are in `verification/B-628_curl_look_table.md` (18 of 18 pass). B-326, B-461, B-525 and P9's tables hold FX-CURL-001 to 077.

## Speed (measured, the card)

**Setup:**
- Machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan) and an AMD Ryzen 9 9900X (24 threads), Windows.
- Build: release, 8 loops at 1920 by 1080.
- Measured: 2026-10-23, 14:42 to 14:50.
- The machine was **not quiet**: other build lanes were running.

**What changed:**
- The new look reads four layers of noise at every step of every flow line, so it costs more than the old one.
- The card first did the noise in double precision, which was about 7 times slower than before.
- It now does the noise as audit P2 did: the corner in double precision, the rest in single precision. The pictures still land within 1 level of 255 of the processor's.
- **The Source This Layer shots (B-461) are still about 2.5 times slower.** I tried single precision on the picture read too, but it broke a fixture, so I took it back.

Again, in ms, before D-749 / D-749 in double / D-749 as committed:

| Shot | Before | Double | Committed |
|---|---:|---:|---:|
| B-326 Curl Noise as added | 32.6 | 234.5 | 74.1 |
| B-326 Input Noise | 19.8 | 240.4 | 82.0 |
| B-326 Sample Radius 100, 24 samples | 48.6 | 444.8 | 130.4 |
| B-461 This Layer as added | 83.8 | 220.6 | 212.5 |
| B-461 This Layer, Softness 50 | 87.8 | 227.2 | 239.5 |
| B-525 as added | 111.8 | 240.7 | 81.9 |
| B-525 Rotation 45, Scale 150, Perspective Offset | 110.2 | 238.7 | 81.8 |

**Notes and gaps:**
- B-525's "before" was measured on a different day, so its drop may partly be noise.
- The committed B-326, B-461 and B-525 timing tables are not rewritten. These numbers are recorded here instead.
- The processor timings (`B326_CPU`) were not run.
- **Input Noise still runs the trace.** It shows only the starting noise, so it could skip the trace and be cheaper. That is left for later.

## Verdict for the owner

- **Pass** if steps 1 to 5 look as described and the sheets' right-hand pictures look more like the left than the middle ones do.
- **Fail** if the marbling looks wrong to you, or if a project's saved settings do not open.
