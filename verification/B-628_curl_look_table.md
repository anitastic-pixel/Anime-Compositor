# B-628: Curl Noise's look (D-749)

Audit rows 8, 13 and 14: After Effects' large, smooth marbling that follows Size and swirls further with Sample Radius; Flow Softness crisper as it goes up; the speeds added up frame by frame, so a keyed Speed drifts from where it was. The rule is document 21's and `tools/curl_noise_reference.py`'s; FX-CURL-001 to 077 hold it in B-326, B-461 and B-525's tables. Here: FX-CURL-078 (a keyed Speed), the drift at other frame rates, Draft against Full, and the street.

## FX-CURL-078: a keyed Speed adds up

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CURL-078 frame 0: Speed keyed from 0 at frame 0 to 40 at frame 4, Direction 90, Turbulence Speed 0, Input Noise: the drift adds up 0, 1, 2 and 3 pixels, so frame 4 is frame 0 moved 6 right (not the 16 that speed 40 times 4 frames would jump to). | largest difference 3.0e-8 | yes |
| FX-CURL-078 frame 2: Speed keyed from 0 at frame 0 to 40 at frame 4, Direction 90, Turbulence Speed 0, Input Noise: the drift adds up 0, 1, 2 and 3 pixels, so frame 4 is frame 0 moved 6 right (not the 16 that speed 40 times 4 frames would jump to). | largest difference 3.7e-8 | yes |
| FX-CURL-078 frame 4: Speed keyed from 0 at frame 0 to 40 at frame 4, Direction 90, Turbulence Speed 0, Input Noise: the drift adds up 0, 1, 2 and 3 pixels, so frame 4 is frame 0 moved 6 right (not the 16 that speed 40 times 4 frames would jump to). | largest difference 3.0e-8 | yes |
| FX-CURL-078: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_curl_078.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## The drift at other frame rates (fixture 011, Speed 20, Direction 90, Input Noise)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 24 fps: frame 1 is frame 0 moved 2 pixels right, within 1 level, and not the same as frame 0 | largest difference 0 of 255 on 0 pixels; 115 pixels differ from frame 0; warnings [] | yes |
| 12 fps: frame 1 is frame 0 moved 4 pixels right, within 1 level, and not the same as frame 0 | largest difference 0 of 255 on 0 pixels; 122 pixels differ from frame 0; warnings [] | yes |
| 48 fps: frame 1 is frame 0 moved 1 pixels right, within 1 level, and not the same as frame 0 | largest difference 0 of 255 on 0 pixels; 109 pixels differ from frame 0; warnings [] | yes |

## Draft against Full (the street, Curl Noise as added, frame 12)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Draft's pixels against Full's averaged in blocks: the same marbling, a mean difference under 8 levels | Draft 120 by 68, Full 480 by 270, blocks 4 by 4; mean difference 0.17 levels | yes |

## Pictures: the street, in `verification/B-628 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png draws cleanly | warnings []; crispness (mean step across) 0.96 levels | yes |
| 2_size_300.png draws cleanly | warnings []; crispness (mean step across) 0.53 levels | yes |
| 3_flow_softness_0.png draws cleanly | warnings []; crispness (mean step across) 0.83 levels | yes |
| 4_flow_softness_100.png draws cleanly | warnings []; crispness (mean step across) 1.08 levels | yes |
| 5_sample_radius_0.png draws cleanly | warnings []; crispness (mean step across) 0.66 levels | yes |
| 6_sample_radius_80.png draws cleanly | warnings []; crispness (mean step across) 1.84 levels | yes |
| Row 8: Size 300's marbling is larger and smoother than Size 100's (smaller steps across) | 0.53 against 0.96 levels | yes |
| Row 13: Flow Softness 100 is crisper than Flow Softness 0 (larger steps across) | 1.08 against 0.83 levels | yes |
| Row 8: Sample Radius 80 carries the noise further than Sample Radius 0, so most pixels differ | 113411 of 129600 pixels differ | yes |

## Result

18 of 18 checks pass.
