# B-620 playtest: Curl Noise's Speed and Density hints, and two documents brought up to date

Audit rows 1 and 2 (22 October 2026). Wording only: no picture, no saved project and no fixture changes.

## What changed for you

- **Curl Noise, Speed.** The hover hint used to say "pixels a second the noise drifts". The noise really moves a tenth of a pixel each frame for each unit of Speed (`src/grade.rs`, `speed / 10.0` times the frame). It now says "how fast the noise drifts: each 10 moves it 1 pixel a frame".
- **Curl Noise, Density.** The hint used to say "lighter or darker noise". Density really scales the pattern (`size * 2^(-density / 50)`). It now says "makes the pattern smaller (above 0) or bigger (below 0); 50 halves it".
- **Speed Lines' row in `docs/effects/EFFECTS.md`** no longer claims a parallel mode. The effect only draws radial wedges towards a centre.
- **`docs/SUPPORTED_ENVELOPE.md`** (shipped as "READ ME FIRST.md") no longer says the app has three effects, cannot draw a mask or make a keyframe, cannot change a blend mode, effect order or composition size, exports only PNG sequences, or exports MP4 with no sound.

## Try it

1. Add Curl Noise to a layer (Effects, Generate, Curl Noise).
2. Hold the mouse over **Speed** in the Effect Controls. You should see "0 to 100, how fast the noise drifts: each 10 moves it 1 pixel a frame".
3. Hold it over **Density**. You should see "-100 to 100, makes the pattern smaller (above 0) or bigger (below 0); 50 halves it".
4. Set Density to 50: the swirls should look half as big as at 0.
