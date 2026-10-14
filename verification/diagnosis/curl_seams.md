# Diagnosis: Curl Noise's straight seams and triangles

This was a read-only diagnosis. No code, fixture or test was changed, and nothing was built with cargo. The pictures are copied from the app's own frames (`tools/app_snapshot.py`, frame 12, on the processor, full size). The crop is x 800-1120, y 440-640, shown at 2x.

## Plain summary

**The seams come from the walk that builds each streak, not from Swirl.** Every pixel draws its streak by walking a fixed 20 or 40 px along the flow, in steps of the same length, and averaging the grain it passes. The flow has many *crossing points*. At a crossing point, flow comes in from two sides and leaves on the other two, like traffic at a junction. A pixel just left of the incoming line walks out one exit, and its neighbour just right of it walks out the opposite exit. Those two pixels get unrelated grain, so a hard edge shows between them. Close to the crossing point these dividing lines are straight, so each crossing point draws a straight "X" about one walk long.

**The triangles are the four wedges between the arms of each X.** Every pixel in one wedge walks out the same exit and ends up averaging nearly the same grain. The streak brightener (step 6 of the rule: it multiplies by about 4.4 at 12 samples and 6.1 at 24) then pushes that shared average to solid white or solid black. That is why the wedges are flat, hard-edged triangles.

The walk always moves its full length, even where the flow has almost stopped. That is what makes it jump across a crossing point. A test copy whose walk slows down where the flow is slow removed every seam pixel at Swirl 0, 24 samples / 20 px (2.6 % of the crop before, 0.00 % after). At the settings as saved, it removed about 83 % of them.

Swirl adds very little. The seam share is 6.2 % at Swirl 0 and 6.4 % at Swirl 90. The earlier diagnosis (`glasses_curl.md`, point 2) named the wrong main cause.

## 1. Symptom

In Final Render, the streak picture breaks along straight lines, and some regions are flat white or black triangles with straight sides. The seams stay with Swirl 0. They stay with 24 samples / 20 px, which removes the comb ripples. Their size follows Sample Radius.

## 2. Root cause

Rule: `Markdown/21_Rendering_Math_Specification.md:567`, steps 3 and 6. Processor: `src/grade.rs`. Card: the same in `src/gpu.rs`.

- `src/grade.rs:1476`: the flow is "made one long". Its strength is thrown away, so a pixel next to a crossing point, where the flow is nearly zero, gets a full-strength direction. The card does the same in `curlfield` (`src/gpu.rs`, around 9798-9801).
- `src/grade.rs:1611-1622`: the walk takes `sample_count` steps of exactly `sample_radius / sample_count` each, whatever the flow's strength (`qx += way * step * ex / length`). It covers the full radius even while it passes through a crossing point, so it always leaves through one of the exits. The card does the same in `curlflow` (`src/gpu.rs:9896`).
- `src/grade.rs:1631`: `lines = ss / sw * (sw / sws.sqrt())` multiplies the averaged grain by about 4.4 at 12 samples or 6.1 at 24. This is right when the samples are independent. In a wedge, though, they all come from the same exit line, so the shared average is pushed to white or black. Card: `src/gpu.rs:9907`. This makes the triangles solid. It does not make the edges.

Value noise always has many crossing points (saddles). About 35 X marks are visible in the 320 x 200 crop at 24/20.

## 3. Evidence

**Seam map.** `curl_seams/curl_seams_where.png`. Left: the app's picture. Right: the same picture with red wherever two neighbouring pixels' walks end more than 3 px apart. (The walk's average position was measured by giving it a stand-in grain equal to the pixel's x, then its y.) At Swirl 0, 24/20 (bottom), the red marks are clean X shapes, and each one sits on a straight seam with its triangles in the app picture. At the settings as saved (top), the X's are stretched into curves by Swirl and the longer 40 px walk.

**The removal test.** `curl_seams/curl_seams_fix_test.png`, one row per setting. Left: the app now. Middle (test A): the walk's step is shortened in proportion to the flow's strength wherever that strength is below a typical value (1 / size), and is never longer than now. Right (test A+B): A, plus the brightener scaled by how far the walk really travelled, so a walk that barely moved is not pushed to white or black. In the bottom row the straight seams and flat triangles are gone in A+B. A alone also removes the seams but leaves white and black blotches: where the walk stalls, the raw blocky grain shows through the brightener.

Seam pixels (walks ending more than 3 px from a neighbour's), in the crop:

| Setting | As built | Test A (walk slows) |
|---|---|---|
| Swirl 90, 12 / 40 (as saved) | 6.35 % | 1.09 % |
| Swirl 0, 12 / 40 | 6.18 % | 1.14 % |
| Swirl 0, 24 / 20 | 2.64 % | 0.00 % |

**Reproduction.** A numpy copy of `Curl::at` and the Final Render walk reproduces the app's crop to within 1 grey level (mean difference 0.001, largest 0.002 on a 0 to 1 scale) for all three settings. All tests ran on that copy. It is kept outside the repo, in this session's scratchpad (`seams/curl.py`, `fixes.py`, `evidence.py`).

**Suspects ruled out.** Each was switched off one at a time, at Swirl 0, 24/20, and the seams stayed:
- *Direction read between pixels, or held from the last step.* Working the flow exactly at every walk point, with no grid, kept the seams.
- *Sign flips of the one-long direction.* Flipping any reversed reading back kept the seams.
- *Seams in the grain.* An all-smooth grain (Edge Definition 0) kept the seams.
- *Edges or tiles.* The margin is `ceil(radius) + 2` and steps never exceed the radius, so the walk never reaches the edge. The field is worked per pixel, not on a coarse grid.
- *Value noise's grid.* One noise layer shows the same X's. So does a grid-free stand-in noise (summed waves).
- *Swirl.* The seam share hardly changes between Swirl 0 and 90 (table above).

One seam was traced by hand. Pixels x 1047 and 1048 at y 566 both walk up to a crossing point near (1044, 556), where the flow is about a quarter of its usual strength. From there 1047 leaves to the left and 1048 to the right.

## 4. Confidence

**High** that walking full-length through crossing points is the main cause, and the brightener is what makes the triangles solid. The seam map lines up with the X's, and removing the cause removes every seam pixel at 24/20.

**Medium** that it is the whole story at the settings as saved. About 1 % of seam pixels remain at radius 40 with test A. They come from walks passing crossing points at full strength, plus some real piling-up of flow from Swirl. The remaining lines in the right column of the top two rows are faint.

## 5. Fix options

Any change to the streak picture changes the Final Render expected values in `Fixtures/curl_noise/expected_curl_noise.json` (FX-CURL cases). It also changes the written rule (document 21 line 567) and `tools/curl_noise_reference.py`. **Whether to change the look is your decision.** Nothing here was changed.

0. **No change, settings only.** A shorter Sample Radius makes the X's smaller. Lower Contrast or higher Flow Softness makes the triangles less solid. The seams stay. Size: none.
1. **Slow the walk where the flow is slow, and brighten by distance travelled (test A+B, recommended).** Keep the flow's strength in the field instead of making it one long, in units of 1 / size. The Curl Generation view must still show the one-long direction. Make each step `min(1, strength)` times today's step, so a step is never longer than now. Multiply the brightener's extra by (distance walked / (2 x radius)). Processor: about 15 lines in `src/grade.rs` (around 1476, 1611-1631). Card: the same in `src/gpu.rs` `curlfield` / `curlflow`. Plus the rule, the reference and the regenerated FX-CURL expected values. Open point: for Source This Layer / Other Layer (`CurlPlane`) the flow's strength is in lightness per pixel, not 1 / size, so it needs its own typical value. Picking it is a small specification decision.
2. **Test A alone.** It is about 5 lines smaller than option 1, but it leaves white and black blotches (middle column), so it is not recommended.
