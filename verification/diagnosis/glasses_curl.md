# Diagnosis lane glasses_curl: 3D Glasses and Curl Noise

Read-only diagnosis on main at 90a33e9a. No code, fixture or test was changed and nothing was built or run.

## Plain summary

**3D Glasses is not broken. The check was set up so it could not show anything.** A layer named as a view is read as its own picture before it is moved. That is the accepted rule (D-189), and it is how After Effects reads a view layer too. In all three failing tries the views were the same picture: either the layer itself, or a hidden copy of the same image that was shifted only by its Position. Position does not count, so the two views were the same picture. Convergence was 0, so nothing moved them apart. When both eyes see the same picture, the "balanced colored red blue" mode gives back the original picture exactly. So "0 pixels changed" is the right answer for those settings. The engine's own test (B-386) uses a right view that really is a different picture, and that test passes. The fix is to redo the check with a right view that really differs. No engine change is needed.

**Curl Noise does what its written rule says, but the rule has three visible weaknesses. The fixtures pin all three, so changing the look is your decision.**
1. *Comb or ladder ripples along the streaks.* Each streak is built from only 12 + 12 points, about 3.3 px apart at radius 40. The grain it smears is made of blocks about 2.8 px wide (Size / 50). The points are spaced wider than the grain, so the streak cannot blend the grain smoothly. The gaps between the points show as a regular comb.
2. *Hard seams.* With Swirl above 0, the flow is no longer a true "curl", the kind of flow that never piles up. Our own rule says so: "at Swirl 0 exactly the curl, which never gathers or spreads". With Swirl 90, flow lines pile into lines from both sides. The streaks on the two sides of such a line come from different places, so a hard edge shows where they meet.
3. *Swirls smaller than Size.* Size 140 sets the coarsest noise layer's grid to 140 px, so its bumps are about 70 px across. At Smoothness 50 the three finer layers (70, 35 and 17.5 px) steer the flow exactly as much as the coarse one. So most swirls are 40 to 100 px.

## Bug 1: 3D Glasses "ignores named view layers"

**1. Symptom.** With `left_view` or `right_view` naming a layer, the frame is identical to no effect, with no message (`three_d_glasses`, `three_d_glasses_shown`, `three_d_glasses_right`). With both views "" and convergence 40 it works (`three_d_glasses_self`).

**2. Root cause: the snapshot projects, not the engine.** The engine path is complete and correct:
- `src/effects.rs:10006-10008` lists both views as layer settings (centre fit), and `src/effects.rs:10092-10094` gives their map slots.
- `src/compose.rs:4081-4128` (`fill_maps`) reads every named view through `setting_map` (`src/compose.rs:5511-5549`). It is called from the layer path (`src/compose.rs:2982`) and the adjustment path (`src/compose.rs:407`), so the processor and the card both get the maps.
- `src/layer_fx.rs:7704-7738` (`glasses_settings`) returns `None` only when a view is named but was not read. A missing layer also writes EFFECT_LAYER_MISSING (`src/compose.rs:5534`). The card uses the same function (`src/gpu.rs:17086-17101`).
- `setting_map` reads the layer's **own picture before it is placed** (`resolve_layer`, `src/compose.rs:5547`; the holder itself is stripped of its effects at `src/compose.rs:5542`). This is D-189's accepted rule (`Markdown/14_Decisions_Risks.md:449`): "its transform, parent, the camera, its matte, opacity, blend mode, its switch ... make no difference."

In the projects, layer `right` is `asset-layer1`, the same image as `layer-1`, moved only by `position: [40, 0]`. So:
- `three_d_glasses` and `_shown`: left = `layer-1`, which is the holder, its own drawing. Right = `right`, the same drawing, because the position is ignored and on/off does not count.
- `three_d_glasses_right`: left = "" (the holder), right = `right`, the same drawing again.

Every case has convergence 0 and alignment 0, so l = r at every pixel (`src/layer_fx.rs:9203-9204`). The balance step at `src/layer_fx.rs:9208-9214` leaves equal views equal (m = l = r). Then balanced colored red blue = (r.r, l.g, l.b) (`src/layer_fx.rs:9218`), which is the original pixel, and alpha = max(l.a, r.a) is the original alpha. So the output equals the input exactly. That matches "0 pixels changed". `_self` works only because convergence 40 pulls the two copies 40 px apart.

**3. Confidence.** Confirmed by reading the code: the identity follows from the formula and the projects' data. It is also consistent with `tests/b386_three_d_glasses.rs:366-423`, which uses a right view that is a different image (`2_right.png`, or `layer-4`) and checks a visible anaglyph on both processor and card.

**4. Fix plan.** No engine change. Redo the check with views that really differ, using any one of these:
- (a) `right_view` = `layer-4` (another picture), as the B-386 test does;
- (b) keep the `right` copy, but shift it with an **effect** (for example `core.transform` with a 40 px position) instead of its layer Position, because D-189 reads a layer after its own effects;
- (c) keep the views as they are and set `scene_convergence` to 40.

Optional, not a bug: say in the Effect Controls tip for Left/Right View that a view layer's position, scale and so on are not used, because the owner's check shows the rule is easy to trip over. That wording would be a UX decision for the owner.

**5. Proof plan.** No fixture or test change. Before and after pictures: copy `three_d_glasses_right.json` to a new project, `three_d_glasses_offset`, where layer `right` carries a `core.transform` effect with position shifted 40 px (or `right_view: "layer-4"`), then run:
`python tools/app_snapshot.py --project verification/app_snapshots/three_d_glasses_offset/three_d_glasses_offset.json --frames 0,12,24 --on both`
Expected: red and cyan fringes about 40 px wide on every edge, with cpu and card within 1 level. The existing `three_d_glasses*` rows in LANE2_RESULTS.md should be re-marked "works, the settings give the same picture by D-189" instead of "fails". No Fixtures/ expected values change.

**6. Size.** Small: a new snapshot project and a corrected results row.

## Bug 2: Curl Noise looks wrong at full size

Project `verification/app_snapshots/curl_noise/curl_noise.json`: size 140, swirl 90, density 0, smoothness 50, sample_count 12, sample_radius 40, flow_softness 20, edge_definition 50, contrast 120. The processor (`src/grade.rs:1360-1657`) and the card (`src/gpu.rs:9748` `curlfield`, `src/gpu.rs:9858` `curlflow`) implement the same rule, written in document 21 (`Markdown/21_Rendering_Math_Specification.md:567`) and in `tools/curl_noise_reference.py:14-47`. CPU and card match (max difference 1). These are rule issues, not code bugs.

**1. Symptom.** Streaks carry a regular comb of dots instead of smooth lines. A few hard seams cut across the flow. The swirls are 40 to 100 px across at Size 140 on a 1920 frame.

**2. Root causes.**
- *Comb.* `src/grade.rs:1611-1629`: the walk takes `floor(sample_count)` fixed steps of `sample_radius / sample_count` = 40 / 12 = 3.33 px each way, with no sub-steps, no jitter, and only bilinear reads of a per-pixel field. The seed it averages is value noise on a grid of `s / 50` = 2.8 px (`src/grade.rs:1477-1486`), and with edge_definition 50 it is half hard blocks. The steps are longer than the grain, so the 25-tap triangle filter is undersampled. It cannot remove grain at its own spacing, and the leftover aliasing shows as rungs about 3 px apart along every streak. The defaults hit this too: size 100 and radius 30 give 2 px cells and 2.5 px steps.
- *Seams.* `src/grade.rs:1471-1476`: the flow is the slope turned by (-90 + swirl x N) degrees. Only at Swirl 0 is this a true curl. Document 21 says so itself ("at Swirl 0 exactly the curl, which never gathers or spreads"). At Swirl 90 the turn runs from -180 to 0 degrees, so in places the flow runs down or up the slope. There it piles into lines where flow comes in from both sides. Each side's streak is traced from different seed, so the picture breaks along that line. The flow is also made one long (`src/grade.rs:1476`), so a pile-up line gives a full-strength direction flip, not a fade.
- *Swirl size.* `src/grade.rs:1452-1462`: octave o is read on a grid of s / 2^o, and its slope is weighted `amp x fine` = (r x 2)^o, with r = 0.7 - 0.4 x smoothness / 100 = 0.5. That is 1 for every octave. The 140, 70, 35 and 17.5 px grids therefore steer the flow equally. On top of that, a value-noise bump is about half its grid, so even the coarsest swirls are about 70 px.

**3. Confidence.**
- Comb: confirmed by reading (step 3.33 px against a 2.8 px seed grid). Single check to make sure: shot `sample_count 24`, `sample_radius 20` (0.83 px steps); the comb should go away.
- Seams: likely. Single check: the same project with `swirl 0`; the seams should vanish and the comb stay.
- Size: confirmed by reading the formula. How large the swirls "should" be is not written anywhere: Adobe published no formula (D-446), and "Size" here is the grid of the coarsest layer.

**4. Fix plan.** Each item is a change to the rule, so it needs the owner first (see 5). They are listed smallest first, and each fixes the processor and the card together, in `src/grade.rs` `curl_noise`/`Curl::at`, the WGSL `curlfield`/`curlflow` in `src/gpu.rs`, document 21, and `tools/curl_noise_reference.py`. No other effect shares this code. Brush Strokes and Vector Blur have their own walks.
- Comb: keep Sample Count as how often the direction is re-read, but add the seed every <= 1 px along the path, using sub-steps between the re-reads. Simpler alternative: make the seed grid at least 2 steps wide, `cell = max(s / 50, 2 x radius / count)`. Sub-stepping costs about radius / count times more seed reads, up to 9 times at radius 200 with 24 samples. Use a time-based perf check when built, not a timing claim.
- Seams: either (a) apply Swirl as a curl of a swirled potential, for example the curl of N plus swirl x N^2, so the flow stays a true curl, or (b) accept the seams as the look of Swirl and document it. (b) costs nothing.
- Size: either (a) weight octave slopes by `amp` only, so fine octaves add detail without steering, or (b) define Size as the swirl diameter (grid = 2 x size). Both change every Internal-source picture.

**5. Fixtures pin the current look. Any change is a specification decision for the owner.** FX-CURL-001 to 044 (`Fixtures/curl_noise/expected_curl_noise.json`, `Markdown/25_Test_Fixture_Catalog.md:208090` on) pin the exact rule within 2e-5, and so do FX-CURL-045 to 060 (D-583) and the D-646 transform set (`expected_curl_transform.json`). That covers the step spacing, the s / 50 seed grid, the swirl turn and the octave weights. Every option above except "seams, (b)" changes those expected values. Proposed proof once the owner picks: regenerate the expected files with the updated reference tool as a separate fixture commit before the code (per the two-commit rule), then run `tests/b326_curl_noise.rs`, `tests/b461_curl_sources.rs` and `tests/b525_curl_transform.rs`. Before and after pictures:
`python tools/app_snapshot.py --project verification/app_snapshots/curl_noise/curl_noise.json --frames 0,12,24 --on both --full`
Compare `full/frame_0012_card_full.png`: streaks smooth with no rungs (comb fix), no hard seams (seam fix a), and swirls about 140 px or larger (size fix). Cheap checks to run first, with no code change: the same project with `swirl 0`, and with `sample_count 24, sample_radius 20`. These show the owner each cause separately before deciding.

**6. Size.** Comb: medium (processor, card, reference, document 21 and regenerated fixtures). Seams (a): medium. Size: small in code, but it regenerates every Internal fixture. Done together, the three are medium to large, mostly fixture and spec work.
