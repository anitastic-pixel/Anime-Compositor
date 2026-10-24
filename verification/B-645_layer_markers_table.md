# B-645: layer markers against the reference

Written by `tests/b645_layer_markers.rs`. Each case is `Fixtures/layer_markers/`'s, worked by `tools/layer_markers_reference.py` from D-765's rule. Each file is opened, slid where the case says, and saved; what the program holds, what it warns and what it writes must all be the reference's.

| Fixture | Layer markers held | Warning | Layer `markers` written | Composition markers written | Pass |
| --- | --- | --- | --- | --- | --- |
| fx_lmark_001.json | [{"duration_frames":0,"frame":12,"name":"hit"}] | none | [{"frame":12,"name":"hit"}] | [] | yes |
| fx_lmark_002.json | [{"duration_frames":0,"frame":4,"name":"start"},{"duration_frames":6,"frame":30,"name":"end"}] | none | [{"frame":4,"name":"start"},{"duration_frames":6,"frame":30,"name":"end"}] | [] | yes |
| fx_lmark_003.json | [] | none | "absent" | [] | yes |
| fx_lmark_004.json | [] | PROJECT_FEATURE_UNSUPPORTED | "hit" | [] | yes |
| fx_lmark_005.json | [] | PROJECT_FEATURE_UNSUPPORTED | [{"frame":"twelve","name":"hit"}] | [] | yes |
| fx_lmark_006.json | [{"duration_frames":0,"frame":2,"name":"in"}] | none | [{"frame":2,"name":"in"}] | [{"duration_frames":4,"frame":10,"name":"chorus"},{"frame":30,"name":"out"}] | yes |
| fx_lmark_007.json | [{"duration_frames":3,"frame":17,"name":"hit"}] | none | [{"duration_frames":3,"frame":17,"name":"hit"}] | [] | yes |
| fx_lmark_008.json | [] | none | null | [] | yes |
| fx_lmark_009.json | [] | none | [] | [] | yes |

9 of 9 cases match the reference.
