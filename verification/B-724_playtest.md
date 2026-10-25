# B-724 playtest: a better way to compare a tutorial with ours

## What changed for you

Nothing in the app. This adds a measuring tool, `tools/visual_compare.py`, that checks an After
Effects tutorial frame against our render in several ways at once:
- colour difference as the eye sees it;
- whether the shapes and edges line up;
- the shape of a glow's, blur's or shadow's fall-off;
- how pixels moved, for warps.

Before, most checks used one number. That number could not say whether a glow was too wide, too
strong or the wrong colour. This tool can.

## What to look at

Open the four sheets in `verification/B-724 pictures/`. Each sheet has four parts:
1. After Effects' frame on the left.
2. Ours next to it.
3. A heat map of the difference (red: ours brighter, blue: ours darker).
4. Where a glow or blur is involved, a plot of how the light falls off with distance. The white
   line is After Effects and the orange line is ours.

| Sheet | What you should see |
| --- | --- |
| glow_motionnations_i1_measured.jpg | The two "GLOW" pictures look the same, the heat map is nearly black, and the white and orange lines lie close together. |
| glow_motionnations_i0.2_measured.jpg | The pictures look nearly the same. In the plot, the orange line (ours) runs a little above the white one. At Intensity 0.2, our glow is a little stronger than After Effects'. |
| twirl_dope_a58_measured.png | The twirled swirl has the same shape in After Effects and in "after D-744". The "before" picture is turned less. |
| gaussian_bullo_measured.png | The blurred orange disc's soft edge is as wide in After Effects as in ours. |

## Check it works

Run `python -I tools/visual_compare.py selftest`. It should end with
`selftest passed (23 of 23)`.

Correct if the sheets look as described and the self-test passes. Incorrect if any sheet shows
the two pictures clearly different where the table says they match.
