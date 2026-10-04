# D-316 / B-197: Colorama (reduced)

Found by P-26. The Colorful Glitch tutorial (3) repaints a picture's brightnesses with After Effects' **Colorama**. Here there was none.

## What changed

- A new effect, **Colorama**, under Color Correction. Each brightness of the layer is repainted with a colour from a ring of colours: black takes the first colour, and going up to white runs once round the ring and back to the first.
- Its settings:
  - **Get Phase From**: Intensity, Luminance, Red, Green, Blue or Alpha.
  - **Phase Shift**: turns the ring, in degrees.
  - **Cycle Repetitions**: how many times round from black to white.
  - **Colours In Ring**: 2 to 5.
  - **Colour 1** to **Colour 5**.
  - **Blend With Original**: mixes back toward the layer's own colour.
  - **Add Phase** and **Add Phase Placement**: another layer whose brightness is added, as in After Effects.
- It starts as After Effects' does: an even hue cycle (here five hues).
- Reduced: After Effects also has some 30 palette presets, a wheel of any number of colours, and Modify and pixel-selection steps. These are left until asked for.
- It is drawn on the CPU, so the preview and the export match.

## Checks (cargo test)

`tests/b197_colorama.rs`: 4 of 4 pass.

| Check | Expected | Got |
|---|---|---|
| Black and white ring: greys 0.1, 0.25, 0.75, 0 | 0.2, 0.5, 0.5 (on the way back), black | so |
| Phase Shift 180, -180 and 360; repetitions 2 and 0 | 0.8, 0.8, unchanged; 0.4; all the first colour | so |
| Blend With Original 50 and 100 | half way back in linear light; the original exactly | so |
| The starting five hues at 0, 0.2 and 0.1 | red, the second colour, half between them | so |
| Three stops; 3.9 stops | the first three colours; taken as 3 | so |
| Red, green, blue, alpha, intensity, luminance on a half-clear colour | each read as worked by hand; alpha kept at half | so |
| A clear picture | unchanged | so |
| Another layer, white, added to a black card at half a repetition | black without it, white with it | so |
| Wrong phase word, colour, stops, repetitions | refused with a sentence | refused |
| Saved and read back | written as read | so |

Also still passing: the whole core suite and the app suite, including the window's effect contract.

## Pictures (the test copy, never the owner's app)

The setup is a 640 x 360 card with a Gradient running from black at the top to white at the bottom, then Colorama.

| Picture | Look for | Pass? |
|---|---|---|
| `D-316 pictures/1_gradient.png` | the ramp alone, black to white | pass |
| `D-316 pictures/2_colorama_as_it_starts.png` | the ramp as one rainbow: red, yellow-green, green, blue, purple, back to red at the bottom | pass |
| `D-316 pictures/3_three_times_round_shift_90.png` | three rainbows stacked, starting a quarter way round | pass |
| `D-316 pictures/4_two_colours_navy_orange.png` | two colours: navy at top and bottom, orange in the middle | pass |
| `D-316 pictures/5_blend_50.png` | the same, mixed half way back toward the grey ramp | pass |

## For the owner to try

1. Put **Colorama** on any picture. Its brightnesses become a rainbow.
2. Drag **Phase Shift**: the colours cycle. Set a key at 0 and another at 360 a second later, and the colours cycle through once.
