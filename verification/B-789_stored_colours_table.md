# B-789: stored colours, display-value rules and depths with colour management

Written by `tests/b789_stored_colours.rs` (D-910, ADR-023 step C4). The expected numbers were written by `tools/c4_reference.py` into `Fixtures/colour/c4/expected_c4.json` before the code: the published rules in double precision in linear Rec.709, and OpenColorIO 2.5.1's Linear Rec.709 to ACEScg matrix for the same picture working in ACEScg. The bar: |build - expected| at most 1e-5 in linear light, at every pixel, and the two working spaces' pictures the same bytes through today's view.

**14 of 14 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| gradient: Rec.709 and ACEScg within 1e-5 of expected, both the expected bytes through today's view, Rec.709 on = off bit for bit | pass | pass | pass |
| luma_matte: Rec.709 and ACEScg within 1e-5 of expected, both the expected bytes through today's view, Rec.709 on = off bit for bit | pass | pass | pass |
| overlay: Rec.709 and ACEScg within 1e-5 of expected, both the expected bytes through today's view, Rec.709 on = off bit for bit | pass | pass | pass |
| solid: Rec.709 and ACEScg within 1e-5 of expected, both the expected bytes through today's view, Rec.709 on = off bit for bit | pass | pass | pass |
| refused_eight_bpc.json is refused at opening, the composition named | COLOR_DEPTH_REFUSED, true | COLOR_DEPTH_REFUSED, true | pass |
| refused_ae_32bpc.json is refused at opening, the composition named | COLOR_DEPTH_REFUSED, true | COLOR_DEPTH_REFUSED, true | pass |
| float_acescg.json (Float, ACEScg) opens | true | true | pass |
| the composition settings asking for 8 bpc (AE) with colour management on are refused and the project is unchanged | COLOR_DEPTH_REFUSED, true | COLOR_DEPTH_REFUSED, true | pass |
| the composition settings asking for 32 bpc (AE) with colour management on are refused and the project is unchanged | COLOR_DEPTH_REFUSED, true | COLOR_DEPTH_REFUSED, true | pass |
| and Display is accepted | true | true | pass |
| and Float is accepted | true | true | pass |
| with colour management off, 8 bpc (AE) is accepted as before | true | true | pass |
| an asset tagged with a word this build does not have, saved with no record of the file it came from, keeps the word | true | true | pass |
| the card hands an ACEScg frame to the processor, says so (GPU_PREVIEW_ON_CPU), and the bytes are the processor's | true, true | true, true | pass |

## The four pictures, case by case

The worst difference from the expected numbers working in linear Rec.709 and in ACEScg; whether today's view (sRGB curve, 8 bits) gives the expected bytes from the Rec.709 picture and from the ACEScg picture taken back to Rec.709; and whether colour management on in Rec.709 draws the same bits as off. The pictures are in `verification/B-789 pictures/`, each pixel shown 16 times as wide and tall.

| Case | Worst, Rec.709 | Worst, ACEScg | View, Rec.709 | View, ACEScg back | On = off | Result |
|---|---|---|---|---|---|---|
| gradient | 2.0e-8 | 3.0e-8 | same | same | same bits | pass |
| luma_matte | 4.6e-8 | 6.0e-8 | same | same | same bits | pass |
| overlay | 4.3e-8 | 6.0e-8 | same | same | same bits | pass |
| solid | 1.2e-8 | 3.4e-13 | same | same | same bits | pass |

COLOR_DEPTH_REFUSED (document 28) is the refusal both at opening and in the composition settings.
