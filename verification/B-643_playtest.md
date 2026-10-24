# B-643 playtest: a new Curl Noise starts at After Effects' values (D-764)

## What changed for you

A Curl Noise you add from now on starts at **Speed 0, Turbulence Speed 10, Swirl 180, Sample
Count 3, Sample Radius 80 and Flow Softness 25**, as After Effects' panel shows a fresh Curl Noise
(Softoni's tutorial at 0:48). Since D-749 ours uses After Effects' names and units for these six, so
the numbers are the same. Before, ours started at 10, 20, 45, 12, 30 and 20.

A project you saved before this change looks exactly as it did: its Curl Noise keeps the six
numbers the file holds.

No picture of an older project changes, so the proof is the panel's starting values side by side.

## What to look at

`verification/B-643 pictures/1_curl_noise_start_ae_before_after.png`: After Effects' panel for a
fresh Curl Noise, then ours before (10, 20, 45, 12, 30, 20) and after (0, 10, 180, 3, 80, 25).

`verification/B-643_curl_noise_start_table.md`: the app check, 6 of 6 pass.

## Try it

1. Add Curl Noise to a layer. The panel shows Speed 0, Turbulence Speed 10, Swirl 180, Sample
   Count 3, Sample Radius 80 and Flow Softness 25. With Speed 0 the noise does not slide; it
   still churns over time (Turbulence Speed 10).
2. Type 10, 20, 45, 12, 30 and 20 back in: all six are kept.
3. Open a project you saved before today that has a Curl Noise: it looks the same as before.

## Known differences from After Effects (not fixed here)

- After Effects' panel also starts Smoothness and Vertical Bias at 0. Ours keeps 50 for both,
  because ours reads them differently (our Vertical Bias 50 means "even").
