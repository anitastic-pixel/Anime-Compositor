# B-969: Convert Audio to Keyframes (D-1089)

Each project in `Fixtures/audio_keyframes/` is turned into keys by the build (`tests/b969_audio_keyframes.rs`) and held to the numbers `tools/audio_keyframes_reference.py` wrote before the code: on each frame of the work area, Left Channel is 100 times the mean of the left samples' sizes, Right Channel the same for the right, Both Channels their mean (document 21). Tolerance 0.002.

| Case | What | Frames | Frame | Expected L, R, Both | Build L, R, Both | Result |
|---|---|---|---|---|---|---|
| fx_akf_001 | the beat: Left Channel jumps on the beats, Right Channel steady, Both their mean | 0 to 47 | 0 | 50.402, 18.638, 34.520 | 50.402, 18.638, 34.520 | pass |
| fx_akf_001 | the beat: Left Channel jumps on the beats, Right Channel steady, Both their mean | 0 to 47 | 6 | 0.000, 18.638, 9.319 | 0.000, 18.638, 9.319 | pass |
| fx_akf_001 | the beat: Left Channel jumps on the beats, Right Channel steady, Both their mean | 0 to 47 | 47 | 0.000, 18.645, 9.323 | 0.000, 18.645, 9.323 | pass |
| fx_akf_001 | every frame | 0 to 47 | all 48 | | largest difference 0.000000 | pass |
| fx_akf_002 | the beat at -6 dB starting at frame 6: nothing before frame 6, then half as loud | 0 to 47 | 0 | 0.000, 0.000, 0.000 | 0.000, 0.000, 0.000 | pass |
| fx_akf_002 | the beat at -6 dB starting at frame 6: nothing before frame 6, then half as loud | 0 to 47 | 6 | 25.261, 9.341, 17.301 | 25.261, 9.341, 17.301 | pass |
| fx_akf_002 | the beat at -6 dB starting at frame 6: nothing before frame 6, then half as loud | 0 to 47 | 47 | 25.278, 9.345, 17.312 | 25.278, 9.345, 17.312 | pass |
| fx_akf_002 | every frame | 0 to 47 | all 48 | | largest difference 0.000000 | pass |
| fx_akf_003 | the beat and a 44,100 mono hum together: the hum adds to both channels | 0 to 23 | 0 | 53.046, 35.100, 44.073 | 53.046, 35.100, 44.073 | pass |
| fx_akf_003 | the beat and a 44,100 mono hum together: the hum adds to both channels | 0 to 23 | 6 | 31.978, 34.494, 33.236 | 31.978, 34.494, 33.236 | pass |
| fx_akf_003 | the beat and a 44,100 mono hum together: the hum adds to both channels | 0 to 23 | 23 | 31.999, 35.130, 33.564 | 31.999, 35.130, 33.564 | pass |
| fx_akf_003 | every frame | 0 to 23 | all 24 | | largest difference 0.000000 | pass |
| fx_akf_004 | a work area of frames 12 to 36: keys on those 24 frames only | 12 to 35 | 12 | 50.402, 18.638, 34.520 | 50.402, 18.638, 34.520 | pass |
| fx_akf_004 | a work area of frames 12 to 36: keys on those 24 frames only | 12 to 35 | 18 | 0.000, 18.638, 9.319 | 0.000, 18.638, 9.319 | pass |
| fx_akf_004 | a work area of frames 12 to 36: keys on those 24 frames only | 12 to 35 | 35 | 0.000, 18.645, 9.323 | 0.000, 18.645, 9.323 | pass |
| fx_akf_004 | every frame | 12 to 35 | all 24 | | largest difference 0.000000 | pass |
| fx_akf_005 | 29.97 frames a second: each frame's own samples, 1601 or 1602 of them | 0 to 29 | 0 | 51.555, 18.618, 35.086 | 51.555, 18.618, 35.086 | pass |
| fx_akf_005 | 29.97 frames a second: each frame's own samples, 1601 or 1602 of them | 0 to 29 | 6 | 51.912, 18.767, 35.340 | 51.912, 18.767, 35.340 | pass |
| fx_akf_005 | 29.97 frames a second: each frame's own samples, 1601 or 1602 of them | 0 to 29 | 29 | 0.714, 18.071, 9.392 | 0.714, 18.071, 9.392 | pass |
| fx_akf_005 | every frame | 0 to 29 | all 30 | | largest difference 0.000000 | pass |
| fx_akf_006 | the only audio layer switched off: nothing to hear, so the command is refused | none | - | refused | Main has no audio layer that is on, so there is nothing to turn into keys. | pass |
