# D-299 / B-184: Fractal Noise gets After Effects' look settings

Found by P-26, tutorials 1 (Video Copilot, Shockwave) and 3 (Video Copilot, Colorful Glitch). The Shockwave sets Fractal Type to Turbulent and stretches the noise with Scale Width; the Glitch uses Noise Type Block, Offset Turbulence and Cycle Evolution for blocks that loop. Our Fractal Noise had none of them, so neither tutorial could be followed past that step.

## What changed

Fractal Noise has seven new settings in Effect Controls, each starting where the old noise was:

| Setting | Starts at | What it does |
|---|---|---|
| Fractal Type | Basic | Turbulent folds the noise into sharp dark creases |
| Noise Type | Smooth | Block draws one flat grey a square |
| Invert | Off | On swaps dark and light |
| Offset Turbulence | 0, 0 | moves the clouds, in pixels |
| Scale Width, Scale Height | 100, 100 | stretch the clouds across or down, per cent of Size |
| Cycle Evolution | 0 (never) | the evolution repeats after this many turns, so a loop is seamless |

A file that never touches them saves exactly as before, and every older file draws the same.

## Checks (cargo test)

`tests/b184_fractal_noise_look.rs`, 5 of 5 pass. Worked by hand on an 8x8 white solid with black-to-white noise:

| Check | Expected | Got |
|---|---|---|
| Invert, every pixel | grey is 1 - the plain grey | the same, to 0.00001 |
| Turbulent, one octave, every pixel | grey is \|2 x plain - 1\| | the same, to 0.00001 |
| Block, size 4 | one grey over each 4x4 square; the four squares differ | so |
| Offset (3, 0) | the plain picture moved 3 pixels right | identical, pixel for pixel |
| Size 100 at Scale Width 50% against Size 50 at Scale Height 200% | the same picture | identical |
| Cycle 1: evolution 90 against 450; Cycle 2: 90 against 810 | the same picture | identical |
| Cycle 2: 90 against 450; no cycle: 90 against 450 | different | different |
| The file at the starts | none of the seven written | none |
| The file with all seven changed | all seven kept | kept |
| Fractal Type "wavy"; Scale Width 0 | refused with the reason | refused |
| A half-size draft | size and offset halved | halved |

The old noise's own fixtures, FX-FRACTAL-001 to 028 (`tests/b71_fractal_noise.rs`), and Turbulent Displace's (`tests/b70_turbulent_displace.rs`) still pass unchanged. The graphics card draws all seven the same as the CPU: `verification/B-155_gpu_chain_table.md`, 846 of 846, with a new row using all seven at once. The app suite's command walk (`verification/B-12b_state_fields_table.md`) sends all seven from the panel and the command accepts them.

## Pictures (the test copy, never the owner's app)

A 640x360 grey solid with Fractal Noise, Size 60, Complexity 6. Draft quality, so the viewer shows bigger pixels.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-299 pictures/1_basic_as_before.png` | all at their starts | soft clouds, as before | pass |
| `D-299 pictures/2_turbulent_stretched.png` | Turbulent, Scale Width 400 | thin dark creases, stretched sideways | pass |
| `D-299 pictures/3_block.png` | Block, Offset 20, 0 | squares of flat grey, big and small | pass |
| `D-299 pictures/4_inverted.png` | Invert On | picture 1 with dark and light swapped (measured: on average 0.06 of a level from the exact inverse) | pass |

Found on the way, in the window run: the panel drew a point box for Offset Turbulence and stopped with an error, because the file leaves Offset out at its start. Offset is pixels, not a place on the picture, so it now has no point box, as the Offset effect already had none; and Scale Width and Height show 100, not 0, when the file leaves them out.

## For the owner to try

1. Make a solid and add Fractal Noise.
2. Set Fractal Type to Turbulent: dark creases appear. Set Scale Width to 400: they stretch sideways.
3. Set Noise Type to Block: squares. Drag Offset Turbulence: they slide.
4. Set Evolution keys from 0 at frame 0 to 360 at the last frame, and Cycle Evolution to 1: the first and last frames match, so it loops.

Still proposed, not built, because each would change existing fixtures: Complexity above 8 (After Effects goes to 20), Brightness past +-100 (After Effects +-200), and brighter-than-white values kept for a later Glow.

Fixtures are unchanged.
