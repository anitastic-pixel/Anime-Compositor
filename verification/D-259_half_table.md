# D-259: Half beside Full and Draft

Written by `tests/d259_half.rs` from `verification/B-08a_project.json`. The Full and Draft hashes were taken on the build before Half existed (258a0de). Times are this machine's, on the CPU, with nothing remembered between runs.

| Check | Expected | Actual | Result |
|---|---|---|---|
| Full frame 10, 1920 by 1080, is byte for byte as before Half | 06ad045e059c4f492c71d761f034f56aaf6ec034931bc731e33f33a20711d284 | 06ad045e059c4f492c71d761f034f56aaf6ec034931bc731e33f33a20711d284 | pass |
| Full frame 100, 1920 by 1080, is byte for byte as before Half | 833970a6fab53413553b4cdb2a350f1048c2fa039c6f8af2684700a3d19e5c8d | 833970a6fab53413553b4cdb2a350f1048c2fa039c6f8af2684700a3d19e5c8d | pass |
| Draft frame 10, 480 by 270, is byte for byte as before Half | 6c32af698fb03b107625be15a9b3e4ab7a901ac4da8ac73a2d69c6806288505e | 6c32af698fb03b107625be15a9b3e4ab7a901ac4da8ac73a2d69c6806288505e | pass |
| Draft frame 100, 480 by 270, is byte for byte as before Half | 46bc3314f00bc927f67c5108563e4bc9b4f580b12acfc9de2c6aa5c4525b575e | 46bc3314f00bc927f67c5108563e4bc9b4f580b12acfc9de2c6aa5c4525b575e | pass |
| Half is half the composition each way | 960 by 540 | 960 by 540 | pass |
| Half says it differs from an export (the orange "not final") | true | true | pass |
| Half frame 10 is the recorded picture | 1888530bc056fd7456c2302f1842539f954e48b0788ec33056d90ba4458a3df8 | 1888530bc056fd7456c2302f1842539f954e48b0788ec33056d90ba4458a3df8 | pass |
| Half is at least as quick as Full (median of 7, frame 10, CPU: Full 52.6 ms, Half 42.8 ms, Draft 39.5 ms) | true | true | pass |
