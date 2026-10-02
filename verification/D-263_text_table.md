# D-263: text layers

Written by `tests/d263_text.rs`. The fixture hash was taken on e395d8b, before D-253.

**39 of 39 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| A text layer is added on top of the reference shot | taken | taken | pass |
| A text layer with no words, font or size is refused | refused | refused | pass |
| New words, on two lines and in Japanese, are taken | taken | taken | pass |
| A size of 0 is refused | refused | refused | pass |
| A size of 5000 pixels is refused | refused | refused | pass |
| A colour above 1 is refused | refused | refused | pass |
| A place that is not a number is refused | refused | refused | pass |
| A font named by a path rather than a file name is refused | refused | refused | pass |
| A font that is not a .ttf, .otf or .ttc file is refused | refused | refused | pass |
| Words on a drawn layer, which is not a text layer, are refused | refused | refused | pass |
| The words now | Cut 012\nあいう | Cut 012\nあいう | pass |
| Undo puts the words back | Cut 012 | Cut 012 | pass |
| Saved, the layer is kind text with its words, font, size, colour, place and alignment | text, Cut 012\nあいう, MPLUSRounded1c-Regular.ttf, 120, [1,0.9,0.2], [200,600], left | text, Cut 012\nあいう, MPLUSRounded1c-Regular.ttf, 120, [1,0.9,0.2], [200,600], left | pass |
| It has no drawing, footage offset or exposures written | none | none | pass |
| Opened again, the layer is the same | the same | the same | pass |
| Saved again, the text is the same, byte for byte | the same | the same | pass |
| A line inside the text record and one on the layer that no build writes yet are kept | kept | kept | pass |
| A file with a text record on a drawn layer is refused | refused | refused | pass |
| A file with a text layer and no text record is refused | refused | refused | pass |
| A file with a text layer holding exposures is refused | refused | refused | pass |
| A file with a text size of 0 is refused | refused | refused | pass |
| The font that comes with the program is found | found | found | pass |
| "Text あ" at 120 pixels, placed at 200, 600: ink right of 188, left of 920, below 456 and above 642, and more than 3000 pixels of it | yes | yes | pass |
| Drawn a second time, every pixel is the same | the same | the same | pass |
| Aligned right at 1700, the ink ends between 1676 and 1702 | yes | yes | pass |
| Centred on 960, the ink's middle is within 24 pixels of it | yes | yes | pass |
| A second line is drawn wholly below the first | yes | yes | pass |
| Two outlines turning the same way that overlap are filled where they overlap | 1 | 1 | pass |
| An outline turning the other way inside another is a hole, as in an O | hole 0, ring 1 | hole 0, ring 1 | pass |
| A 20 by 20 square on whole pixels covers exactly 400 pixels | 400 | 400 | pass |
| Frame 10 of the reference shot is written | written | written | pass |
| With the text layer on top, frame 10 changes and nothing is said | changed, nothing said | changed, nothing said | pass |
| With the text layer switched off, frame 10 is byte for byte the frame without it | the same | the same | pass |
| A font this machine does not have is taken: the project may go to a machine that has it | taken | taken | pass |
| With a font this machine does not have, frame 10 says TEXT_FONT_MISSING and is marked incomplete | said, incomplete | said, incomplete | pass |
| Nothing is drawn in its place: every pixel is the frame without the layer's | the same | the same | pass |
| The font that comes with the program has its licence beside the others | there | there | pass |
| Fixture projects that open | 2297 | 2297 | pass |
| Their saved text, all of it, is byte for byte as before (SHA-256) | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | pass |
