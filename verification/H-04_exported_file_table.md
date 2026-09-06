# H-04 â€” the picture in the file, not the picture in memory

**20 of 20 checks passed.**

Produced by `tests/h04_exported_file.rs`.

## Why this exists

H-01, H-02 and H-03 composite your shot twice and compare every pixel, but all three of them stop in the renderer's own working space. The file you actually open is one step further on: colour converted for output, transparency unwound, numbers rounded to whole ones and written into bytes.

Every other test in this project that touches that last step asks the build's own encoder what the answer should be. That catches arithmetic and cannot catch layout. If the samples were written blue-first, or a row were dropped, or the two bytes of a sixteen-bit sample were written the wrong way round, every existing table would still pass and every pixel of your export would be wrong.

So this exports frames through the real export path, reads the files back off disk with a decoder that did not write them, and compares them against numbers produced by a separate compositor and a separately written encoder.

## What is checked

Two frames at eight bits, one at sixteen, one written with transparency left folded into the colour - the option document 07 offers and almost nobody wants - and two more with the background layer taken away, for the reason in the next section but one. For each, every sample in the file is compared, plus what the file says about itself: its size, its depth, and that it has a number for every channel of every pixel.

## The tolerance

**One code value out of 255** (or out of 65535 at sixteen bits), which is what document 11 allows for an eight-bit round trip where rounding applies. But a tolerance that is right for one kind of fault is a hiding place for another: rounding a number down instead of to the nearest whole one also moves every sample by at most one code value, and moves millions of them. So **how many samples are not exactly identical** is a check in its own right, and the answer is one, and nine, out of eight and a quarter million. The rest of what this file exists to catch is worth far more than one code value anyway: a swapped channel is the difference between two colours, a dropped row is a whole row, a byte-order mistake is up to 255 in the wrong half of the number.

## Why some rows have the background taken away

Your first layer is opaque and covers the frame, so every pixel of the finished picture is fully solid. Two of the steps this file is about - unwinding the transparency out of the colour, and leaving alpha out of the colour conversion - do nothing at all to a fully solid pixel. Three deliberate breaks in those exact steps passed everything else here. The only way to ask the question is a picture that is genuinely half-transparent, so the same frame is exported again without its background, and a row counts how many pixels in it are neither solid nor empty so that this cannot quietly stop being true.

## The rows that are not comparisons

One takes frame 100's samples and compares them against what frame 0 should look like, and requires them to disagree in hundreds of thousands of places. Without it, a build where both sides produced nothing at all would pass every row above. One requires the frame to be a real picture rather than a flat colour, for the same reason. One requires the two ways of writing transparency to actually disagree on the picture they are tested on. And one counts the half-transparent pixels described above.

## What is deliberately not here

Whether the picture itself is right - whether frame 100 looks like your shot - is H-01's question and H-03's. This one asks only whether what those tests verified in memory is what ends up in the file.

As in H-01 to H-03, both sides were written from the same document by the same agent: this catches an implementation slip, not a misreading of the specification.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| frame 0 is written as a 1920 by 1080 eight-bit RGBA picture | `1920x1080 Rgba Eight` | `1920x1080 Rgba Eight` | pass |
| frame 0 carries a sample for every channel of every pixel | `8294400` | `8294400` | pass |
| frame 0: every one of the 8294400 samples in the file is within one code value of what a separate compositor and a separately written encoder say it should be | `0 samples differ by more than one` | `0 samples differ by more than one` | pass |
| frame 0: and only 1 of them are not exactly identical, so the one-code-value allowance is barely being used rather than covering a systematic difference | `fewer than 1000 of 8294400 not identical` | `fewer than 1000 of 8294400 not identical` | pass |
| frame 100 is written as a 1920 by 1080 eight-bit RGBA picture | `1920x1080 Rgba Eight` | `1920x1080 Rgba Eight` | pass |
| frame 100 carries a sample for every channel of every pixel | `8294400` | `8294400` | pass |
| frame 100: every one of the 8294400 samples in the file is within one code value of what a separate compositor and a separately written encoder say it should be | `0 samples differ by more than one` | `0 samples differ by more than one` | pass |
| frame 100: and only 9 of them are not exactly identical, so the one-code-value allowance is barely being used rather than covering a systematic difference | `fewer than 1000 of 8294400 not identical` | `fewer than 1000 of 8294400 not identical` | pass |
| asked for sixteen bits, the file says sixteen bits | `Sixteen` | `Sixteen` | pass |
| and carries one sixteen-bit number per channel of every pixel | `8294400` | `8294400` | pass |
| every sixteen-bit sample is within one code value of the independent encoder, which is also what says the two bytes are in the order the PNG specification requires | `0 samples differ by more than one` | `0 samples differ by more than one` | pass |
| and only 178 of them are not exactly identical, with the largest difference 1 out of a possible 65535 | `fewer than 1000 not identical` | `fewer than 1000 not identical` | pass |
| with the opaque background taken away, so the picture is partly transparent nearly everywhere, every sample of the Straight-alpha export matches the independent encoder | `0 samples differ by more than one` | `0 samples differ by more than one` | pass |
| and only 0 of those samples are not exactly identical | `fewer than 1000 not identical` | `fewer than 1000 not identical` | pass |
| with the opaque background taken away, so the picture is partly transparent nearly everywhere, every sample of the Premultiplied-alpha export matches the independent encoder | `0 samples differ by more than one` | `0 samples differ by more than one` | pass |
| and only 0 of those samples are not exactly identical | `fewer than 1000 not identical` | `fewer than 1000 not identical` | pass |
| and the two ways of writing transparency genuinely disagree on this picture, which is what the two rows above depend on and what the full shot cannot provide | `more than 100000 samples differ` | `more than 100000 samples differ` | pass |
| frame 106 without its background has 142892 pixels that are neither solid nor empty, which is what makes the transparency steps observable at all | `more than 100000 such pixels` | `more than 100000 such pixels` | pass |
| the comparison can fail: frame 100's samples against frame 0's expected samples disagree in hundreds of thousands of places | `more than 100000 samples differ` | `more than 100000 samples differ` | pass |
| and the frame being compared is a real picture, not a flat colour: its red channel takes many different values | `more than 32 distinct values` | `more than 32 distinct values` | pass |
