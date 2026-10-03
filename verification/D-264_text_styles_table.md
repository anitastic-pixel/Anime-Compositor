# D-264: text styles

Written by `tests/d264_text_styles.rs`. The D-263 pictures were taken on 2edc62d and the fixture hash on e395d8b.

**57 of 57 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| A text layer using none of the new settings saves only D-263's six lines | align, at, color, font, size, text | align, at, color, font, size, text | pass |
| Every new setting at once is taken | taken | taken | pass |
| Saved, they read back as set | justify, 50, 160, true, true, true, true, 600, {"color":[0,0,0],"width":6}, {"color":[0,0.1,0.4],"opacity":0.75,"padding":24,"roundness":12}, {"angle":135,"color":[0,0,0],"distance":8,"opacity":0.5,"softness":6} | justify, 50, 160, true, true, true, true, 600, {"color":[0,0,0],"width":6}, {"color":[0,0.1,0.4],"opacity":0.75,"padding":24,"roundness":12}, {"angle":135,"color":[0,0,0],"distance":8,"opacity":0.5,"softness":6} | pass |
| Opened again, the text is the same | the same | the same | pass |
| Saved again, the file is the same, byte for byte | the same | the same | pass |
| A line inside the stroke or the shadow that no build writes yet is kept | kept | kept | pass |
| Stroke, background and shadow taken off are no longer written | none written | none written | pass |
| Undo puts them back | back | back | pass |
| Tracking of 5000 is refused (-1000 to 1000) | refused | refused | pass |
| Leading below 0 is refused | refused | refused | pass |
| A box narrower than 0 is refused | refused | refused | pass |
| A stroke 0 pixels wide is refused | refused | refused | pass |
| A stroke colour above 1 is refused | refused | refused | pass |
| A background opacity above 1 is refused | refused | refused | pass |
| A background padding below 0 is refused | refused | refused | pass |
| A shadow opacity below 0 is refused | refused | refused | pass |
| A shadow distance below 0 is refused | refused | refused | pass |
| A shadow softness of 1000 is refused (0 to 500) | refused | refused | pass |
| A shadow angle that is not a number is refused | refused | refused | pass |
| A file aligning "justified", which is no alignment, is refused | refused | refused | pass |
| A file with tracking "wide" is refused | refused | refused | pass |
| A file with a stroke with no width is refused | refused | refused | pass |
| A file with kerning 1 rather than true or false is refused | refused | refused | pass |
| Tracking 100 (thousandths of the size) puts 10 pixels after each of HHHH's first three letters: 30 wider | yes | yes | pass |
| With kerning, A and V close up by the font's own 100 units of 1000: 10 pixels | yes | yes | pass |
| Kerning changes nothing between letters the font does not kern | the same | the same | pass |
| Leading 200 puts the second line's baseline 200 pixels below the first | yes | yes | pass |
| All caps draws "abc" byte for byte as "ABC" | the same | the same | pass |
| Faux italic leans an I: its top is at least 10 pixels right of its foot | yes | yes | pass |
| Faux bold thickens an I: a fifth more ink or better, its middle where it was | yes | yes | pass |
| In a 400 pixel box from x 100, words wrap onto three lines or more and stay inside it | yes | yes | pass |
| A word longer than the box is broken inside it | yes | yes | pass |
| Japanese, which has no spaces, wraps between any two characters | yes | yes | pass |
| Centred in the box, a line's middle is the box's middle, 300 | yes | yes | pass |
| Aligned right in the box, a line ends at the box's right side, 500 | yes | yes | pass |
| Justified, the first line reaches both sides of the box; aligned left it falls short | yes | yes | pass |
| Justified, the last line stays aligned left, as the editors' Justify Last Left does | the same | the same | pass |
| In a box, the place is the box's top left corner: an H's top is a little below y 200, not above it | yes | yes | pass |
| A stroke 8 pixels wide reaches 8 pixels beyond the letter on each side | yes | yes | pass |
| Outside the letter the stroke is its own colour, black and solid; inside, the fill is on top | [0.0, 0.0, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0] | [0.0, 0.0, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0] | pass |
| The words' box holds every inked pixel | yes | yes | pass |
| A background 20 pixels out from the words' box, half seen: blue at half inside it, nothing beyond it | [0.0, 0.0, 0.5, 0.5], [0.0, 0.0, 0.0, 0.0] | [0.0, 0.0, 0.5, 0.5], [0.0, 0.0, 0.0, 0.0] | pass |
| With roundness 30 the background's corner is cut away; square, it is not | 0 rounded, 1 square | 0 rounded, 1 square | pass |
| A shadow at 135 degrees, 20 pixels away, lies down and to the right: black just right of the letter | [0.0, 0.0, 0.0, 1.0] | [0.0, 0.0, 0.0, 1.0] | pass |
| Nothing of the shadow up and to the left | [0.0, 0.0, 0.0, 0.0] | [0.0, 0.0, 0.0, 0.0] | pass |
| With softness 10 the shadow's edge is part seen | yes | yes | pass |
| All three at once, from the bottom: background, shadow, stroke, fill | [0.0, 0.0, 1.0, 1.0], [0.0, 1.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0] | [0.0, 0.0, 1.0, 1.0], [0.0, 1.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0] | pass |
| Drawn a second time, every pixel is the same | the same | the same | pass |
| A D-263 text layer, "Text あ\nCut 012" left, is drawn byte for byte as on 2edc62d | d87262ee1bfc63b078b0e2e930113843efc92b8fc94bee1907a42cab23eeb845 | d87262ee1bfc63b078b0e2e930113843efc92b8fc94bee1907a42cab23eeb845 | pass |
| A D-263 text layer, "AV To\nWAVE" center, is drawn byte for byte as on 2edc62d | bb6976575e3dee8c49101f80f5981b9b16a23b24272b0e8e95e05f780dc604df | bb6976575e3dee8c49101f80f5981b9b16a23b24272b0e8e95e05f780dc604df | pass |
| A D-263 text layer, "Right" right, is drawn byte for byte as on 2edc62d | adbbacf2ffcf3bc6f2257c1852275352caa9b4e89e5f3efc3e40fd3479d21b85 | adbbacf2ffcf3bc6f2257c1852275352caa9b4e89e5f3efc3e40fd3479d21b85 | pass |
| The fonts are listed by the names they give themselves: the one that comes with the program is Rounded Mplus 1c, Regular | Rounded Mplus 1c, Regular | Rounded Mplus 1c, Regular | pass |
| And this machine's own, such as Arial, Bold in arialbd.ttf | Arial, Bold | Arial, Bold | pass |
| A listed font's file can be read for the window to type in | the same | the same | pass |
| A styled text layer goes into exported frame 10, which differs from the unstyled one, and nothing is said | different, nothing said | different, nothing said | pass |
| Fixture projects that open | 2297 | 2297 | pass |
| Their saved text, all of it, is byte for byte as before (SHA-256) | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | pass |
