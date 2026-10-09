# D-364: Diffusion's second pass

Anime compositing's usual diffusion is two blurred copies: one in Lighten or Screen, one in Soft Light or Overlay at about 30 to 50 per cent. Diffusion (D-148) was the first; D-364 adds the second to the same effect, Second Blend (Soft Light or Overlay) and Second Amount (0 to 100, starting at 0, off), sharing the one blur. Every expected pixel is `Fixtures/diffusion/expected_diffusion_second.json`, written by `tools/diffusion_second_reference.py` before this code existed, printed in document 25 as FX-DIFFUSE-019 to 032. Tolerance 2e-5. FX-DIFFUSE-001 to 018 are still checked, unchanged, by `tests/b91_diffusion.rs`.

## FX-DIFFUSE-019 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DIFFUSE-019 frame 0: The two-layer diffusion as tomoex lays it: radius 10, lighten at 50, then soft light at 50. The shadow and the line lift as in lighten alone, then the soft light deepens the contrast a little: every dark channel (encoded below a half) of the glow pulls its pixel down, every light one lifts it. | largest difference 2.0e-7 | yes |
| FX-DIFFUSE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-020 frame 0: Screen at 50 then soft light at 50: the settings as they start (FX-DIFFUSE-001) with the second pass laid on. | largest difference 1.6e-7 | yes |
| FX-DIFFUSE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-021 frame 0: Lighten at 50 then overlay at 30. | largest difference 2.0e-7 | yes |
| FX-DIFFUSE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-022 frame 0: Screen at 50 with the second pass written at 0: exactly FX-DIFFUSE-001, sample for sample. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-023 frame 0: Radius 3, amount 0, soft light at 100: only the second pass, the blurred picture in soft light over the drawing. Each channel moves toward the glow's side of an encoded half, darker where the glow is dark, lighter where it is light: the shadow's far corner (1, 1) darkens in every channel, and every skin pixel lightens in every channel. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-024 frame 0: Radius 3, amount 0, overlay at 100: only the second pass, in overlay; the same directions as FX-DIFFUSE-023. | largest difference 2.1e-7 | yes |
| FX-DIFFUSE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-025 frame 0: Radius 0 with the second pass at 50: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-026 frame 0: Second amount keyed from 0 at frame 0 to 100 at frame 4, linear, over lighten at 50: frame 0 is lighten alone, frame 2 is FX-DIFFUSE-019, frame 4 the soft light laid fully on. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-026 frame 2: Second amount keyed from 0 at frame 0 to 100 at frame 4, linear, over lighten at 50: frame 0 is lighten alone, frame 2 is FX-DIFFUSE-019, frame 4 the soft light laid fully on. | largest difference 2.0e-7 | yes |
| FX-DIFFUSE-026 frame 4: Second amount keyed from 0 at frame 0 to 100 at frame 4, linear, over lighten at 50: frame 0 is lighten alone, frame 2 is FX-DIFFUSE-019, frame 4 the soft light laid fully on. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-027 frame 0: Second amount keyed from 0 to 100, eased past its end: held at 100 from frame 2, so frames 2 and 4 are FX-DIFFUSE-026's frame 4. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-027 frame 2: Second amount keyed from 0 to 100, eased past its end: held at 100 from frame 2, so frames 2 and 4 are FX-DIFFUSE-026's frame 4. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-027 frame 4: Second amount keyed from 0 to 100, eased past its end: held at 100 from frame 2, so frames 2 and 4 are FX-DIFFUSE-026's frame 4. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-028 frame 0: FX-DIFFUSE-019 moved three pixels right: the same, moved. | largest difference 2.0e-7 | yes |
| FX-DIFFUSE-028 frame 3: FX-DIFFUSE-019 moved three pixels right: the same, moved. | largest difference 2.0e-7 | yes |
| FX-DIFFUSE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-029 frame 0: Second amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-029 frame 4: Second amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-030 frame 0: Second amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-030 frame 4: Second amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-031 frame 0: Second blend "screen", which is not soft_light or overlay. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-031 frame 4: Second blend "screen", which is not soft_light or overlay. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-032 frame 0: Second amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-032 frame 4: Second amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_diffuse_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Saved with its second amount and second blend | {"amount":50,"blend":"lighten","radius":10,"second_amount":30,"second_blend":"overlay"} | yes |
| A file from before D-364 saves exactly as it was, with no second_amount or second_blend | {"amount":50,"blend":"screen","radius":10} | yes |
| it still grows the drawing's bounds by nothing | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| second amount 101 is refused with a sentence, and nothing changes | Diffusion's second amount runs from 0 to 100, and this is 101. | yes |
| second amount -1 is refused with a sentence, and nothing changes | Diffusion's second amount runs from 0 to 100, and this is -1. | yes |
| second blend "screen" is refused with a sentence, and nothing changes | Diffusion's second blend is "soft_light" or "overlay", and this is "screen". | yes |
| second amount keyed to 150 is refused with a sentence, and nothing changes | Diffusion's second amount runs from 0 to 100, and this is 150. | yes |
| second amount 100 in overlay, the top, is taken | taken | yes |
| second amount keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| FX-DIFFUSE-001 set to lighten 50 then soft light 50 by the command draws FX-DIFFUSE-019's frame | largest difference 2.0e-7 | yes |

## The preview, on the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_diffuse_019.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_020.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_021.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_022.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_023.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_024.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_025.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_026.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_027.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |
| fx_diffuse_028.json: the preview draws the same picture as the CPU, within 1 level of 255 | no fallback, largest difference 0 of 255 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_diffuse_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_diffuse_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_diffuse_028.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a street, in `verification/D-364 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street, no effect; draws cleanly | [] | yes |
| 2_lighten_50.png, Diffusion radius 30, lighten 50, second pass off: the glow as before D-364; draws cleanly | [] | yes |
| 3_lighten_50_soft_light_50.png, the same, then soft light 50: the two-layer stack; draws cleanly | [] | yes |
| 4_lighten_50_overlay_50.png, the same, then overlay 50: stronger contrast; draws cleanly | [] | yes |
| The second pass changes the lighten-only picture, overlay more than soft light | soft light: largest change 14 of 255 over 128701 pixels; overlay: 34 over 128765 | yes |

## Result

71 of 71 checks pass.
