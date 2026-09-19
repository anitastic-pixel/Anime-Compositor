# B-20b: reference audio in the core (D-71, ADR-018)

Written by `cargo test --test b20b_audio`. Every expected value is from
`Fixtures/audio/expected_audio.json`, which `tools/audio_reference.py` wrote before this
code existed. The numbers are whole and the match is exact.

Nothing is heard yet: playback, the waveform and the bar styles are B-20c.

**46 of 46 checks match.**

## A frame becomes a sample (FX-AUD-001 to 004)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AUD-001: 24 frames a second and 48000 samples a second: 2000 samples to every frame. | frames [0, 1, 2, 3, 4, 23, 24, 1000, 86400] begin at samples [0, 2000, 4000, 6000, 8000, 46000, 48000, 2000000, 172800000] | yes |
| FX-AUD-002: 24 frames a second and 44100 samples: 1837.5 to a frame, so frames take 1837 and 1838 in turn and no sample is played twice or dropped. | frames [0, 1, 2, 3, 4, 23, 24, 1000, 86400] begin at samples [0, 1837, 3675, 5512, 7350, 42262, 44100, 1837500, 158760000] | yes |
| FX-AUD-003: 24000/1001 frames a second and 48000 samples: 2002 to every frame, exactly. | frames [0, 1, 2, 3, 4, 23, 24, 1000, 86400] begin at samples [0, 2002, 4004, 6006, 8008, 46046, 48048, 2002000, 172972800] | yes |
| FX-AUD-004: 24000/1001 frames a second and 44100 samples: 1839.3375 to a frame. | frames [0, 1, 2, 3, 4, 23, 24, 1000, 86400] begin at samples [0, 1839, 3678, 5518, 7357, 42304, 44144, 1839337, 158918760] | yes |

## What is heard on a layer (FX-AUD-005 to 008)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AUD-005: A layer that starts at frame 10 plays the file's first sample on frame 10. | 0: silence; 9: silence; 10: 0 up to 2000; 11: 2000 up to 4000; 12: 4000 up to 6000; 13: 6000 up to 8000; 14: 8000 up to 10000; 23: 26000 up to 28000; 24: 28000 up to 30000; 33: 46000 up to 48000; 34: silence; 39: silence; 40: silence | yes |
| FX-AUD-006: A source offset of 5 starts the layer five frames into the file. | 0: silence; 9: silence; 10: 10000 up to 12000; 11: 12000 up to 14000; 12: 14000 up to 16000; 13: 16000 up to 18000; 14: 18000 up to 20000; 23: 36000 up to 38000; 24: 38000 up to 40000; 33: silence; 34: silence; 39: silence; 40: silence | yes |
| FX-AUD-007: A negative source offset: the layer is silent until the file's first sample comes round, three frames after the layer's in frame. | 0: silence; 9: silence; 10: silence; 11: silence; 12: silence; 13: 0 up to 2000; 14: 2000 up to 4000; 23: 20000 up to 22000; 24: 22000 up to 24000; 33: 40000 up to 42000; 34: 42000 up to 44000; 39: silence; 40: silence | yes |
| FX-AUD-008: A file shorter than its layer: silence from the frame after its last sample. 47000 samples end part of the way through the 24th frame, which plays what there is. | 0: 0 up to 2000; 9: 18000 up to 20000; 10: 20000 up to 22000; 11: 22000 up to 24000; 12: 24000 up to 26000; 13: 26000 up to 28000; 14: 28000 up to 30000; 23: 46000 up to 47000; 24: silence; 33: silence; 34: silence; 39: silence; 40: silence | yes |

## What each WAV file is read as (FX-AUD-010)

| Check | The build's answer | Matches |
| --- | --- | --- |
| `adpcm.wav` | {"channels":1,"encoding":"other","sample_rate":22050} | yes |
| `chunks_in_the_way.wav` | {"bits":16,"channels":1,"encoding":"pcm","frames_at_24":1,"sample_rate":48000,"samples":480} | yes |
| `cut_short.wav` | {"bits":16,"channels":1,"encoding":"pcm","frames_at_24":1,"sample_rate":48000,"samples":480,"warning":"MEDIA_AUDIO_CUT_SHORT"} | yes |
| `extensible_6ch_48k.wav` | {"bits":16,"channels":6,"encoding":"pcm","frames_at_24":1,"sample_rate":48000,"samples":480} | yes |
| `float32_stereo_48k.wav` | {"bits":32,"channels":2,"encoding":"float","frames_at_24":1,"sample_rate":48000,"samples":480} | yes |
| `float64_mono_96k.wav` | {"bits":64,"channels":1,"encoding":"float","frames_at_24":1,"sample_rate":96000,"samples":960} | yes |
| `no_data.wav` | MEDIA_AUDIO_UNREADABLE {"refused":"D-71: no data chunk. The layer and its file reference are kept."} | yes |
| `no_fmt.wav` | MEDIA_AUDIO_UNREADABLE {"refused":"D-71: no format chunk. The layer and its file reference are kept."} | yes |
| `no_sound.wav` | {"bits":16,"channels":1,"encoding":"pcm","frames_at_24":0,"sample_rate":48000,"samples":0} | yes |
| `not_a_wav.wav` | MEDIA_AUDIO_UNREADABLE {"refused":"D-71: not a RIFF WAVE file. The layer and its file reference are kept."} | yes |
| `pcm16_mono_48k.wav` | {"bits":16,"channels":1,"encoding":"pcm","frames_at_24":3,"sample_rate":48000,"samples":4800} | yes |
| `pcm24_stereo_44k.wav` | {"bits":24,"channels":2,"encoding":"pcm","frames_at_24":3,"sample_rate":44100,"samples":4410} | yes |
| `pcm32_mono_48k.wav` | {"bits":32,"channels":1,"encoding":"pcm","frames_at_24":1,"sample_rate":48000,"samples":480} | yes |
| `pcm8_mono_8k.wav` | {"bits":8,"channels":1,"encoding":"pcm","frames_at_24":3,"sample_rate":8000,"samples":801} | yes |
| `rf64.wav` | MEDIA_AUDIO_UNREADABLE {"refused":"D-71: not a RIFF WAVE file. The layer and its file reference are kept."} | yes |
| `zero_rate.wav` | MEDIA_AUDIO_UNREADABLE {"refused":"D-71: no sample rate or no channels. The layer and its file reference are kept."} | yes |

## The picture does not change (FX-AUD-020)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Both projects open with no warning | 0 and 0 warnings | yes |
| Frame 0 with the audio layer is frame 0 without it | 48 samples compared, 0 differ | yes |
| Frame 1 with the audio layer is frame 1 without it | 48 samples compared, 0 differ | yes |
| Frame 2 with the audio layer is frame 2 without it | 48 samples compared, 0 differ | yes |
| Saved again, the audio asset and the audio layer are what the file held | {"id":"asset-sound","kind":"audio","name":"sound","path":"media/pcm16_mono_48k.wav"} / {"asset_id":"asset-sound","enabled":true,"gain_db":0,"id":"sound","in_frame":0,"kind":"audio","locked":false,"name":"sound","out_frame":3,"source_offset_frames":0} | yes |

## What a file may not say (FX-AUD-030 to 035)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AUD-030: An audio layer with a transform: it has no place to be. | PROJECT_SCHEMA_INVALID. At /compositions/0/layers/1/transform: expected no transform on an audio layer, which draws nothing (D-71). | yes |
| FX-AUD-031: An audio layer with effects. | PROJECT_SCHEMA_INVALID. At /compositions/0/layers/1/effects: expected no effects on an audio layer, which draws nothing (D-71). | yes |
| FX-AUD-032: An audio layer with a blend mode. | PROJECT_SCHEMA_INVALID. At /compositions/0/layers/1/blend_mode: expected no blend_mode on an audio layer, which draws nothing (D-71). | yes |
| FX-AUD-033: An audio layer whose asset is a picture. | PROJECT_SCHEMA_INVALID. At /compositions/comp-main/layers: expected layer sound to name a sound file if it is an audio layer and a drawing if it is not (D-71); it names asset-bg. | yes |
| FX-AUD-034: A level above +12 dB. | PROJECT_SCHEMA_INVALID. At /compositions/0/layers/1/gain_db: expected a level from -96 to +12 decibels (D-71). | yes |
| FX-AUD-035: A level that is not a number. | PROJECT_SCHEMA_INVALID. At /compositions/0/layers/1/gain_db: expected a number. | yes |

## Editing (D-71; no fixture, the rule is a sentence)

| Check | The build's answer | Matches |
| --- | --- | --- |
| SET_AUDIO_GAIN to -6 sets the level, as one entry to undo | accepted; level -6; 1 new entry | yes |
| Undo puts the level back | level 0 | yes |
| A level of +12.5 is refused | COMMAND_INVALID_VALUE: A level cannot be set to 12.5 dB. | yes |
| A level on a picture layer is refused | COMMAND_INVALID_VALUE: "bg" is not an audio layer, so it has no level. | yes |
| A position on the audio layer is refused | COMMAND_INVALID_VALUE: "sound" is an audio layer, which draws nothing, so there is nothing there to set. | yes |
| A blend mode on the audio layer is refused | COMMAND_INVALID_VALUE: "sound" is an audio layer, which draws nothing, so there is nothing there to set. | yes |
| The audio layer as a matte is refused | COMMAND_INVALID_VALUE: "sound" is an audio layer, so it cannot be a parent or a matte. | yes |
| The audio layer as a parent is refused | COMMAND_INVALID_VALUE: "sound" is an audio layer, so it cannot be a parent or a matte. | yes |
| Moving the audio layer two frames later is the command every layer has | accepted; in_frame 2 | yes |
| Relinking the sound to another file changes the record and leaves the layer on it | accepted; path media/pcm8_mono_8k.wav; the layer still names it: true | yes |
| Collect files copies the sound file into the package | accepted; pcm8_mono_8k.wav, byte for byte the same: true | yes |
