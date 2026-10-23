# B-623 playtest: Twirl turns as After Effects' does (D-744)

## What to try

1. Add **Twirl** to a picture layer in a 1920 by 1080 composition. It starts at Angle 90 and **Twirl Radius** 30 %, the centre in the middle.
2. Set Angle to 70. The twist should reach out to about 576 pixels from the centre (30 % of 1920) and fade out softly at its edge, with no kink where it stops.
3. Type Twirl Radius 100: the twist now covers the whole frame. Type 101: the box refuses it (0 to 100).
4. Open a project saved before this build that has a Twirl. It keeps its Radius in pixels and Twirl Radius shows 0; the twist reaches as far as before, with the new softer falloff.

Before this build, Twirl had only a Radius in pixels and its twist fell off along a sine curve, so the middle turned too little and the outer part too much. Pictures made with the same numbers as an After Effects tutorial looked visibly less twisted in the middle.

## What the check found

Fixture check `tests/b623_twirl.rs`: 93 of 93 (see `B-623_twirl_table.md`). FX-TWIRL-001 to 027 are pixel-compared with `tools/twirl_reference.py` (tolerance 2e-5). Other checks: an old project (`fx_twirl_001.json`, no Twirl Radius) opens with Twirl Radius 0 and its Radius of 50 pixels, and saves without a Twirl Radius; Twirl Radius is refused outside 0 to 100, including when keyed; Draft leaves the per cent alone; the picture is the same however it is cut into tiles. `tests/b94_twirl.rs` 85 of 85 and the card's check `tests/b107_gpu_fx.rs` (7122 of 7122, Twirl 276 of 276, now including FX-TWIRL-022 to 027) pass. The app's tests: 112 passed, 5 ignored.

## Against the tutorials (eight cases from Wave 1)

Each sheet shows After Effects (from the video), the build before this one (old exe, Radius in pixels = Twirl Radius × 19.2) and this build (Radius 0, Twirl Radius as in the video), all drawn by the real app on the CPU. The number is the grey difference against After Effects (lower is closer; the untwirled picture's own difference is given for scale).

| Case | Angle, radius | Untwirled | Before | After | Verdict |
| --- | --- | --- | --- | --- | --- |
| KuroCutt 0:25 | 70, 30 % | 36.8 | 28.0 | 7.6 | matches |
| KuroCutt 0:30 | 70, 100 % | 37.1 | 24.4 | 5.8 | matches |
| Ilija Kostic 2:06 | -55, 30 % | 20.7 | 14.6 | 3.1 | matches |
| Ag Motion Designs 0:45 | 69, 30 % | 56.5 | 48.7 | 30.7 | close (the video frame has guide lines and a second animation on top) |
| Dope Motions 8:36 | 58, 30 % | 40.1 | 37.6 | 21.8 | close (blurry, compressed video) |
| Dope Motions 8:48 | 211, 25 % | 38.5 | 37.7 | 22.5 | close (same) |
| Ervin 1:58 | 100, 30 % | 46.2 | 38.2 | 23.4 | close (moving footage: the cars are elsewhere) |
| eskchat 2:56 | 1165, 30 % | 14.6 | 20.7 | 22.6 | differs: our layer is a screenshot that includes After Effects' own window bars, which our twirl bends and theirs does not; the spiral itself has the same number of turns |

3 match, 4 close, 1 differs (for a reason in the test picture, not the effect).

## Pictures (`B-623 pictures/`)

- `tw_kuro_a70_r30.png`: the word TWIRL. After Effects and this build bend it the same way; the old build bent it too little in the middle.
- `tw_ilija_a-55_r30.png`: a negative angle; the direction matches.
- `tw_dope_a211_r25.png`: a strong twist on smoke. The old build's arms were too loose near the centre; this build's are as tight as After Effects'.

## How this differs from After Effects

- Twirl starts at Angle 90 here. No tutorial shows a fresh Twirl, so this start is ours, not claimed to be After Effects'.
- After Effects smooths its samples at the edge of the twist in its own way; we keep our usual blend of the four nearest pixels.
- Projects made before this build keep Radius in pixels. Set Radius to 0 to switch such a Twirl to Twirl Radius in per cent.
