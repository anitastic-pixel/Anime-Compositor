# B-409: Numbers' and Timecode's words, drawn faster

D-529: each character Numbers (proportional spacing off) and Timecode draw is drawn only over the rows it reaches, and the last four word pictures are kept by their words and settings. No picture may change: the rows drawn are compared number by number with the whole frame drawn as before, and Numbers and Timecode frames are compared bit for bit, by fingerprint, with the frames B-408's engine drew (`verification/B-409_before_fingerprints.txt`, written by that engine with `--ignored b409_words_fingerprints`).

## The rows the words reach, against the whole frame (640 by 360)

| Check | The build's answer | Matches |
| --- | --- | --- |
| a 0 at size 64 in the middle: rows 130..184 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 682 pixels covered | yes |
| a date at size 40 from the left: rows 68..112 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 3830 pixels covered | yes |
| a 7 with a stroke of 6: rows 104..210 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 3195 pixels covered | yes |
| a W, faux bold and italic, a stroke of 3: rows 78..159 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 4150 pixels covered | yes |
| two lines, centred: rows 83..198 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 2186 pixels covered | yes |
| a justified box of words: rows 47..165 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 3937 pixels covered | yes |
| an 8 partly above the top, stroke 4: rows 0..28 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 880 pixels covered | yes |
| an 8 partly below the bottom, stroke 4: rows 314..360 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 1608 pixels covered | yes |
| an 8 wholly above the frame: rows 0..0 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 0 pixels covered | yes |
| a space: rows 360..360 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 0 pixels covered | yes |
| nothing: rows 0..0 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 0 pixels covered | yes |
| abc, all caps, tracking 300: rows 262..304 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 1101 pixels covered | yes |
| 88 at size 500, taller than the frame: rows 0..358 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 55656 pixels covered | yes |
| with a background (the whole frame): rows 0..360 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 7963 pixels covered | yes |
| with a shadow (the whole frame): rows 0..360 of 360, every number the same bits, nothing outside them | same inside: true; transparent outside: true; 5744 pixels covered | yes |

## Numbers and Timecode frames, bit for bit against B-408's engine

| Check | The build's answer | Matches |
| --- | --- | --- |
| Numbers as added (proportional), frame 0 | now 9408dbee1646c298, before 9408dbee1646c298 | yes |
| Numbers, monospaced long date, stroke over fill 6, size 60, frame 0 | now cdb08992446898e7, before cdb08992446898e7 | yes |
| Numbers, monospaced timecode 30, fill over stroke, frame 37 | now d67050202573d5c5, before d67050202573d5c5 | yes |
| Numbers, monospaced random with two places, stroke only, frame 9 | now e336186567d1b03a, before e336186567d1b03a | yes |
| Numbers, monospaced, stroke over fill, partly above the top, frame 0 | now 941f381da96b23e2, before 941f381da96b23e2 | yes |
| Numbers, monospaced, stroke over fill, partly below the bottom, frame 0 | now c8f4dfdd4c403fa7, before c8f4dfdd4c403fa7 | yes |
| Numbers, monospaced leading zeros, right, tracking 200, composite off, frame 0 | now fe53c477487f953e, before fe53c477487f953e | yes |
| Timecode as added, frame 50 | now 61d469f3adbbb637, before 61d469f3adbbb637 | yes |
| Timecode, custom drop frame, size 120, half-opacity box, composite off, 1920 by 1080, frame 3 | now d44a6c8d8bff6909, before d44a6c8d8bff6909 | yes |
| Timecode, feet and frames 35mm, size 120, box past the top, frame 21 | now 18e97c0506a82198, before 18e97c0506a82198 | yes |

## A kept picture is used only for the same words drawn the same way

| Check | The build's answer | Matches |
| --- | --- | --- |
| the same words drawn again (kept) are the same frame | 2263b6a7601594a4 then 2263b6a7601594a4 | yes |
| frame 7 shows the same date as frame 0: the same frame | 2263b6a7601594a4 | yes |
| a blue fill is drawn afresh, not the white words kept | 89e154ed29488935 | yes |
| white again is white | 2263b6a7601594a4 | yes |
| a counting timecode: frame 4 is not frame 3's words | 7d9a8367cf3854a8 and 57c4876af7901095 | yes |

## Result

30 of 30 checks pass.
