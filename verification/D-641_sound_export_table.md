# D-641: a composition's sound in its export (B-520)

Each project is exported and held to the numbers `tools/sound_export_reference.py` wrote before the code. An MP4's sound is AAC (48,000 a second, two channels, 192 kilobits a second, Windows' encoder), which changes every value, so it is checked by the click method only: each case starts a layer holding one click on chosen frames, and the loudest sample near each click, decoded by ffmpeg 8.1 and by Windows' own decoder, must be exactly that frame's first sample, floor(f x 48000 x den / num), counted from the first frame exported. A WAV beside a sequence is 24-bit and compared value for value (tolerance 2e-5 of full level). The **click table** is at the bottom.

## fx_sexp_001: 24 a second: clicks on frames 2, 12 and 30

| Check | The build's answer | Matches |
| --- | --- | --- |
| the MP4 is written | Completed, 1 file | yes |
| its warnings | [] | yes |
| one picture track and one sound track | (1, 1) | yes |
| AAC, 48,000 a second, two channels, about 192 kilobits a second (ffprobe) | aac,48000,2,192000 | yes |
| length at 48,000 a second: Windows' decoder (by the file's edit list) / ffmpeg (at least) | 72000 / 72704 | yes |
| the click on frame 2 lands on sample 4000 (ffmpeg / Windows) | 4000 / 4000 | yes |
| the click on frame 12 lands on sample 24000 (ffmpeg / Windows) | 24000 / 24000 | yes |
| the click on frame 30 lands on sample 60000 (ffmpeg / Windows) | 60000 / 60000 | yes |

## fx_sexp_002: 29.97 a second: clicks on frames 3 and 25, samples 4804 and 40040

| Check | The build's answer | Matches |
| --- | --- | --- |
| the MP4 is written | Completed, 1 file | yes |
| its warnings | [] | yes |
| one picture track and one sound track | (1, 1) | yes |
| AAC, 48,000 a second, two channels, about 192 kilobits a second (ffprobe) | aac,48000,2,192002 | yes |
| length at 48,000 a second: Windows' decoder (by the file's edit list) / ffmpeg (at least) | 64064 / 64512 | yes |
| the click on frame 3 lands on sample 4804 (ffmpeg / Windows) | 4804 / 4804 | yes |
| the click on frame 25 lands on sample 40040 (ffmpeg / Windows) | 40040 / 40040 | yes |

## fx_sexp_003: 25 a second: clicks on frames 1 and 24, samples 1920 and 46080

| Check | The build's answer | Matches |
| --- | --- | --- |
| the MP4 is written | Completed, 1 file | yes |
| its warnings | [] | yes |
| one picture track and one sound track | (1, 1) | yes |
| AAC, 48,000 a second, two channels, about 192 kilobits a second (ffprobe) | aac,48000,2,192003 | yes |
| length at 48,000 a second: Windows' decoder (by the file's edit list) / ffmpeg (at least) | 57600 / 58368 | yes |
| the click on frame 1 lands on sample 1920 (ffmpeg / Windows) | 1920 / 1920 | yes |
| the click on frame 24 lands on sample 46080 (ffmpeg / Windows) | 46080 / 46080 | yes |

## fx_sexp_004: frames 6 to 29 exported: the clicks on frames 8 and 20 are samples 4000 and 28000 of the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| the MP4 is written | Completed, 1 file | yes |
| its warnings | [] | yes |
| one picture track and one sound track | (1, 1) | yes |
| AAC, 48,000 a second, two channels, about 192 kilobits a second (ffprobe) | aac,48000,2,192000 | yes |
| length at 48,000 a second: Windows' decoder (by the file's edit list) / ffmpeg (at least) | 48000 / 48128 | yes |
| the click on frame 8 lands on sample 4000 (ffmpeg / Windows) | 4000 / 4000 | yes |
| the click on frame 20 lands on sample 28000 (ffmpeg / Windows) | 28000 / 28000 | yes |

## fx_sexp_005: an Opus layer is left out with its warning; the click beside it lands

| Check | The build's answer | Matches |
| --- | --- | --- |
| the MP4 is written | Completed, 1 file | yes |
| its warnings | ["MEDIA_AUDIO_PREVIEW_ONLY"] | yes |
| one picture track and one sound track | (1, 1) | yes |
| AAC, 48,000 a second, two channels, about 192 kilobits a second (ffprobe) | aac,48000,2,192000 | yes |
| length at 48,000 a second: Windows' decoder (by the file's edit list) / ffmpeg (at least) | 48000 / 48128 | yes |
| the click on frame 5 lands on sample 10000 (ffmpeg / Windows) | 10000 / 10000 | yes |

## fx_sexp_006: no sound layer: the MP4 has its picture track and no sound track

| Check | The build's answer | Matches |
| --- | --- | --- |
| written, with no warning | Completed, [] | yes |
| picture tracks, sound tracks | (1, 0) | yes |

## fx_sexp_007: a PNG sequence with its WAV: the sine at -6 dB and a click at -6 dB on frame 2

| Check | The build's answer | Matches |
| --- | --- | --- |
| written, the WAV among the files, as shot.wav | Completed, 25 of 25 files, WAV there: true | yes |
| values past full level: 0, and a warning only then | 0 EXPORT_SOUND_CLIPPED warning(s):  | yes |
| format, channels, rate, bits | 1 2 48000 24 | yes |
| length in samples | 48000 | yes |
| the 55 places in both channels, within 0.00002 of full level | largest difference 0 steps of 8,388,607; 110 of 110 exact | yes |

## fx_sexp_008: an animated PNG at 29.97 with its WAV: the ramp from frame 1, frames 0 to 19

| Check | The build's answer | Matches |
| --- | --- | --- |
| written, the WAV among the files, as film.wav | Completed, 2 of 2 files, WAV there: true | yes |
| values past full level: 0, and a warning only then | 0 EXPORT_SOUND_CLIPPED warning(s):  | yes |
| format, channels, rate, bits | 1 2 48000 24 | yes |
| length in samples | 32032 | yes |
| the 44 places in both channels, within 0.00002 of full level | largest difference 0 steps of 8,388,607; 88 of 88 exact | yes |

## fx_sexp_009: an EXR sequence of frames 4 to 15 with its WAV: the ramp, from sample 8000

| Check | The build's answer | Matches |
| --- | --- | --- |
| written, the WAV among the files, as plate.wav | Completed, 13 of 13 files, WAV there: true | yes |
| values past full level: 0, and a warning only then | 0 EXPORT_SOUND_CLIPPED warning(s):  | yes |
| format, channels, rate, bits | 1 2 48000 24 | yes |
| length in samples | 24000 | yes |
| the 33 places in both channels, within 0.00002 of full level | largest difference 0 steps of 8,388,607; 66 of 66 exact | yes |

## fx_sexp_010: a level of +12 dB goes past full level: those values are written at full level, with a warning

| Check | The build's answer | Matches |
| --- | --- | --- |
| written, the WAV among the files, as loud.wav | Completed, 13 of 13 files, WAV there: true | yes |
| values past full level: 30000, and a warning only then | 1 EXPORT_SOUND_CLIPPED warning(s): The sound goes past full level in 30000 places, from frame 0 on, and was written at full level there. | yes |
| format, channels, rate, bits | 1 2 48000 24 | yes |
| length in samples | 24000 | yes |
| the 31 places in both channels, within 0.00002 of full level | largest difference 0 steps of 8,388,607; 62 of 62 exact | yes |

## fx_sexp_011: a GIF with the WAV asked for writes no WAV

| Check | The build's answer | Matches |
| --- | --- | --- |
| the folder holds the GIF and nothing else | Completed, ["preview.gif"] | yes |

## The owner's film: `verification/D-641 film/flash_and_click.mp4`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 72 frames at 24 a second, with its sound | Completed, tracks (1, 1) | yes |
| the click with the flash on frame 12 lands on sample 24000 | 24000 | yes |
| the click with the flash on frame 24 lands on sample 48000 | 48000 | yes |
| the click with the flash on frame 36 lands on sample 72000 | 72000 | yes |
| the click with the flash on frame 48 lands on sample 96000 | 96000 | yes |
| the click with the flash on frame 60 lands on sample 120000 | 120000 | yes |

## Every click

| Check | The build's answer | Matches |
| --- | --- | --- |
| every click lands | see below | yes |

## Result

65 of 65 checks pass.

## The click table

Each click's frame, the sample it must land on, and the loudest sample found there (with its height) by ffmpeg 8.1 and by Windows' own decoder.

| Case | Frame | Must land on | ffmpeg | Windows | Lands |
| --- | --- | --- | --- | --- | --- |
| fx_sexp_001 | 2 | 4000 | 4000 (0.72) | 4000 (0.72) | yes |
| fx_sexp_001 | 12 | 24000 | 24000 (0.71) | 24000 (0.71) | yes |
| fx_sexp_001 | 30 | 60000 | 60000 (0.73) | 60000 (0.73) | yes |
| fx_sexp_002 | 3 | 4804 | 4804 (0.72) | 4804 (0.72) | yes |
| fx_sexp_002 | 25 | 40040 | 40040 (0.73) | 40040 (0.73) | yes |
| fx_sexp_003 | 1 | 1920 | 1920 (0.76) | 1920 (0.76) | yes |
| fx_sexp_003 | 24 | 46080 | 46080 (0.76) | 46080 (0.76) | yes |
| fx_sexp_004 | 8 | 4000 | 4000 (0.72) | 4000 (0.72) | yes |
| fx_sexp_004 | 20 | 28000 | 28000 (0.73) | 28000 (0.73) | yes |
| fx_sexp_005 | 5 | 10000 | 10000 (0.72) | 10000 (0.72) | yes |
| flash_and_click | 12 | 24000 | 24000 (0.71) | | yes |
| flash_and_click | 24 | 48000 | 48000 (0.76) | | yes |
| flash_and_click | 36 | 72000 | 72000 (0.71) | | yes |
| flash_and_click | 48 | 96000 | 96000 (0.76) | | yes |
| flash_and_click | 60 | 120000 | 120000 (0.71) | | yes |
