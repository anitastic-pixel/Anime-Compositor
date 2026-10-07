# B-216: Glow and Solid Composite on display values

D-337, from P-26's tutorial 2. Every expected pixel is `Fixtures/glow_display/expected_glow_display.json`, written by `tools/glow_display_reference.py` before this code existed and printed in document 25 as FX-GLDISP-001 to 008. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-AE32-001 and 002, which D-337 changed, are checked by B-214.

## FX-GLDISP-001 to 008 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLDISP-001 frame 0: 32 bpc (After Effects), Glow in After Effects units at tutorial 2's kind of setting, threshold 0, radius 4, intensity 0.1: worked in display values, the colour at 1.6 times the blurred light and the covering as blurred, so the halo is brighter and wider than Float's. | largest difference 2.2e-6 | yes |
| FX-GLDISP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-002 frame 0: 32 bpc (After Effects), tutorial 2's glow layer in small: Solid Composite on black, then the same Glow. Opaque, the glow lifts the black round the patches with their own colours. | largest difference 2.2e-6 | yes |
| FX-GLDISP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-003 frame 0: 32 bpc (After Effects), Glow with tint #ff4000, threshold 0, intensity 0.2: the tint is used as written, a display value, so read straight in display values the halo is 3.2 times #ff4000. | largest difference 1.5e-6 | yes |
| FX-GLDISP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-004 frame 0: 32 bpc (After Effects), Glow in classic units, FX-GLOW-001's settings: D-89's strength, worked in display values. | largest difference 3.9e-7 | yes |
| FX-GLDISP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-005 frame 0: 32 bpc (After Effects), Glow in After Effects units with Screen: held inside 0 to 1 and screened, in display values. | largest difference 7.5e-6 | yes |
| FX-GLDISP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-006 frame 0: 8 bpc, Glow in After Effects units at After Effects' defaults, threshold 60, radius 10, intensity 1: in display values, then held to 8 bits, so nothing passes white. | largest difference 9.8e-8 | yes |
| FX-GLDISP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-007 frame 0: Float, FX-GLDISP-001's Glow: linear light and D-331's rule as before, FX-GLOW-AE-004 exactly. | largest difference 1.9e-7 | yes |
| FX-GLDISP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLDISP-008 frame 0: 32 bpc (After Effects), the half-covering patches with Solid Composite #3c286e at opacity 50 and nothing else: laid on in display values, the purple used as written. | largest difference 6.1e-8 | yes |
| FX-GLDISP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_gldisp_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gldisp_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gldisp_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gldisp_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gldisp_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: tutorial 2's glow layer, a quarter size, in `verification/D-337 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_float.png, Float, linear light: a thin line with a faint halo; draws cleanly | [] | yes |
| 2_ae_32bpc.png, 32 bpc (After Effects), D-337: the halo laid on in display values; draws cleanly | [] | yes |
| 30 pixels from the line the 32 bpc (After Effects) halo is brighter than Float's and still gold, green under red, blue under green | Float [5, 4, 1], 32 bpc (After Effects) [55, 46, 18] | yes |

## Result

24 of 24 checks pass.
