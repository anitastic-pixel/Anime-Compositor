# D-639: reading sound (B-518)

Each test file is read by the road its name picks (WAV by hand as D-71 reads it; FLAC, MP3 and Ogg Vorbis by symphonia; AAC by Windows' own decoder) and held to the numbers `tools/sound_read_reference.py` wrote before the code: ffmpeg 8.1's decode of the same file. Every test sound has a click on the first sample of frames 2 and 12 at 24 frames a second (B-112's method), so a click on the right sample means the sound is in step with the picture.

## Each file against ffmpeg's decode

| Check | The build's answer | Matches |
| --- | --- | --- |
| `stereo_48k.wav`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.wav`: length 72000 samples | 72000 | yes |
| `stereo_48k.wav`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.wav`: 57 named places within 2e-5 | largest difference 0.00e0 | yes |
| `stereo_48k.flac`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.flac`: length 72000 samples | 72000 | yes |
| `stereo_48k.flac`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.flac`: 57 named places within 2e-5 | largest difference 0.00e0 | yes |
| `stereo_48k.mp3`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.mp3`: length 72000 samples | 72000 | yes |
| `stereo_48k.mp3`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.mp3`: 57 named places within 0.02 (lossy, before measuring) | largest difference 2.98e-7 | yes |
| `stereo_48k.ogg`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.ogg`: length 72000 samples | 72000 | yes |
| `stereo_48k.ogg`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.ogg`: 57 named places within 0.02 (lossy, before measuring) | largest difference 5.96e-8 | yes |
| `stereo_48k.m4a`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.m4a`: length 72000 to 72704 samples | 72000 | yes |
| `stereo_48k.m4a`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.m4a`: 57 named places within 0.02 (lossy, before measuring) | largest difference 8.05e-7 | yes |
| `stereo_48k.aac`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.aac`: length 73728 samples | 73728 | yes |
| `stereo_48k.aac`: clicks on samples 5024 and 25024 | 5024 and 25024 | yes |
| `stereo_48k.aac`: 59 named places within 0.02 (lossy, before measuring) | largest difference 9.83e-7 | yes |
| `stereo_48k.mp4`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.mp4`: length 72000 to 72704 samples | 72000 | yes |
| `stereo_48k.mp4`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.mp4`: 57 named places within 0.02 (lossy, before measuring) | largest difference 8.05e-7 | yes |
| `stereo_48k.mov`: 48000 a second, 2 channels | 48000 a second, 2 channels | yes |
| `stereo_48k.mov`: length 72000 to 72704 samples | 72000 | yes |
| `stereo_48k.mov`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_48k.mov`: 57 named places within 0.02 (lossy, before measuring) | largest difference 8.05e-7 | yes |
| `mono_44k.wav`: 44100 a second, 1 channel | 44100 a second, 1 channel | yes |
| `mono_44k.wav`: length 66150 samples | 66150 | yes |
| `mono_44k.wav`: clicks on samples 3675 and 22050 | 3675 and 22050 | yes |
| `mono_44k.wav`: 55 named places within 2e-5 | largest difference 0.00e0 | yes |
| `mono_44k.wav` working sound: 72000 samples at 48,000 | 72000 and 72000 | yes |
| `mono_44k.wav` working sound: clicks on 4000 and 24000 | 4000 and 24000 | yes |
| `mono_44k.wav` working sound: named places within 2e-5 | largest difference 1.67e-16 | yes |
| `mono_44k.flac`: 44100 a second, 1 channel | 44100 a second, 1 channel | yes |
| `mono_44k.flac`: length 66150 samples | 66150 | yes |
| `mono_44k.flac`: clicks on samples 3675 and 22050 | 3675 and 22050 | yes |
| `mono_44k.flac`: 55 named places within 2e-5 | largest difference 0.00e0 | yes |
| `mono_44k.flac` working sound: 72000 samples at 48,000 | 72000 and 72000 | yes |
| `mono_44k.flac` working sound: clicks on 4000 and 24000 | 4000 and 24000 | yes |
| `mono_44k.flac` working sound: named places within 2e-5 | largest difference 1.67e-16 | yes |
| `stereo_96k.flac`: 96000 a second, 2 channels | 96000 a second, 2 channels | yes |
| `stereo_96k.flac`: length 144000 samples | 144000 | yes |
| `stereo_96k.flac`: clicks on samples 8000 and 48000 | 8000 and 48000 | yes |
| `stereo_96k.flac`: 105 named places within 2e-5 | largest difference 0.00e0 | yes |
| `stereo_96k.flac` working sound: 72000 samples at 48,000 | 72000 and 72000 | yes |
| `stereo_96k.flac` working sound: clicks on 4000 and 24000 | 4000 and 24000 | yes |
| `stereo_96k.flac` working sound: named places within 2e-5 | largest difference 5.55e-17 | yes |
| `mono_8k.wav`: 8000 a second, 1 channel | 8000 a second, 1 channel | yes |
| `mono_8k.wav`: length 12000 samples | 12000 | yes |
| `mono_8k.wav`: clicks on samples 666 and 4000 | 666 and 4000 | yes |
| `mono_8k.wav`: 18 named places within 2e-5 | largest difference 0.00e0 | yes |
| `mono_8k.wav` working sound: 72000 samples at 48,000 | 72000 and 72000 | yes |
| `mono_8k.wav` working sound: clicks on 3996 and 24000 | 3996 and 24000 | yes |
| `mono_8k.wav` working sound: named places within 2e-5 | largest difference 2.78e-17 | yes |
| `six_48k.flac`: 48000 a second, 6 channels | 48000 a second, 6 channels | yes |
| `six_48k.flac`: length 72000 samples | 72000 | yes |
| `six_48k.flac`: clicks on samples 4000 and 24000 | 4000 and 24000 | yes |
| `six_48k.flac`: 57 named places within 2e-5 | largest difference 0.00e0 | yes |
| `six_48k.flac` working sound: 72000 samples at 48,000 | 72000 and 72000 | yes |
| `six_48k.flac` working sound: clicks on 4000 and 24000 | 4000 and 24000 | yes |
| `six_48k.flac` working sound: named places within 2e-5 | largest difference 0.00e0 | yes |

## Refused, and what an older project already had

| Check | The build's answer | Matches |
| --- | --- | --- |
| `stereo_48k.opus` refused with MEDIA_AUDIO_PREVIEW_ONLY, the window still plays it | MEDIA_AUDIO_PREVIEW_ONLY: The sound file "stereo_48k.opus" plays in the window, but exports and sound effects leave it out. | yes |
| a WAV reads exactly as D-71's reader reads it | every sample the same | yes |
| a 48,000 stereo file's working sound is the file | every sample the same | yes |
| a file that is not a WAV inside is still MEDIA_AUDIO_UNREADABLE | Some("MEDIA_AUDIO_UNREADABLE") | yes |

## Measured: the lossy files against ffmpeg's decode, held to 2e-5

| Check | The build's answer | Matches |
| --- | --- | --- |
| `stereo_48k.mp3`: largest difference at the named places | 2.98e-7 | yes |
| `stereo_48k.ogg`: largest difference at the named places | 5.96e-8 | yes |
| `stereo_48k.m4a`: largest difference at the named places | 8.05e-7 | yes |
| `stereo_48k.aac`: largest difference at the named places | 9.83e-7 | yes |
| `stereo_48k.mp4`: largest difference at the named places | 8.05e-7 | yes |
| `stereo_48k.mov`: largest difference at the named places | 8.05e-7 | yes |

## Result

77 of 77 checks pass.
