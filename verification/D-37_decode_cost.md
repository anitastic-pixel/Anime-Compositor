# D-37: where a preview frame's decoding time goes

`verification/B-08_preview_latency.md` measures a preview frame at about 82 ms in draft, of which about 75 ms is reading and decoding the four cels the frame needs. That fired document 23's revisit trigger for the parked preview cache, which is registered as D-37. This file is what D-37 needs before it can be answered: which of the four cels that time belongs to, and how often the shot asks for a drawing it has already asked for.

**Nothing here builds a cache.** The cache is B-08b, PARKED under D-12, and a fired trigger is a reason to ask the owner rather than a permission to build. This is the arithmetic the owner would want in front of them when answering.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: every cel of `Fixtures/reference_shot`, each decoded once untimed and then once timed, by `tests/b08_preview.rs`

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the decoder.

## The cost of one cel, by layer

Every cel of this shot decodes to the same 1920x1080 buffer. The files differ enormously - the background is over two hundred times the size of a character cel - so if decoding cost what the file weighed, this table would span two hundredfold too.

| Layer | Cels on disk | Mean file size | Median decode ms | Slowest decode ms |
|---|---|---|---|---|
| layer1 | 1 | 2112 KB | 15.22 | 15.22 |
| layer2 | 24 | 47 KB | 6.89 | 7.61 |
| layer3 | 11 | 9 KB | 6.69 | 7.09 |
| layer4 | 20 | 9 KB | 6.19 | 6.77 |

One cel from each layer is 34.98 ms, which is what a frame of this shot costs to decode when nothing is remembered between frames. The latency table's decoding column is higher than that, and the difference is honest rather than explained away: this test decodes each cel with the file already fetched once, while a preview frame pays to find and read the file as well.

## How often the shot repeats itself

Counted from `Fixtures/reference_shot/exposure_sheet.json` by `verification/derive_d37_reuse.py`, which never runs the compositor. Run it to check these figures:

```text
python verification/derive_d37_reuse.py
```

One playthrough of the 240-frame shot makes 960 decode requests and uses 57 distinct drawings. Each layer repeats itself at a different rate:

| Layer | Distinct drawings | Requests repeating the previous frame |
|---|---|---|
| layer1 | 1 | 239 of 240 |
| layer2 | 24 | 0 of 240 |
| layer3 | 12 | 120 of 240 |
| layer4 | 20 | 160 of 240 |

Layer 3 asks for twelve drawings and the table above counts eleven cels on disk. That is not an error in either table: the reference shot is missing layer 3's drawing 7 on purpose, so twenty of the 960 requests find nothing and are diagnosed rather than decoded. Those twenty are counted as requests throughout, because a cache would be asked for them too.

| Cels a cache may hold | Decodes per playthrough | Of the 960 requests, avoided |
|---|---|---|
| 1 | 960 | 0.0% |
| 4 | 441 | 54.1% |
| 8 | 441 | 54.1% |
| 24 | 440 | 54.2% |
| 48 | 116 | 87.9% |
| 56 | 97 | 89.9% |
| 57 | 57 | 94.1% |
| unbounded | 57 | 94.1% |

## How to read the two tables together

**Decoding costs what comes out, not what goes in.** The background file is over two hundred times the size of a character cel and decodes in roughly twice the time. A cel is 1920 x 1080 pixels whatever it compressed to, and writing eight megabytes of samples is most of the work. The practical consequence is that there is no cheap cel: even the smallest file on disk costs six milliseconds, so a preview frame cannot be made fast by simplifying the drawings in it.

**The most expensive drawing in the shot is also the one that never changes.** Layer 1 is the background: one drawing, held for all 240 frames, and the slowest of the four to decode. It is decoded 240 times per playthrough and 239 of those produce a buffer identical to the one before. One remembered buffer would remove all of that, and it is the single largest saving available for the smallest possible cache.

**A cache of one cel is nevertheless worthless here, and the middle of the table says why.** The four layers are asked for in rotation, so a cache holding one entry is evicted before it is asked for again and avoids nothing at all. Holding four - one per layer - avoids 54% of decodes. Between four and forty-eight the figure barely moves, because layer 2 is drawn on ones and cycles through all 24 of its drawings before repeating any: nothing short of holding the whole layer helps it. Holding all 57 avoids 94%.

**What 57 cels costs is 473 MB**, because a decoded cel is 1920 x 1080 x 4 bytes whatever its file compresses to. That is the real shape of the decision: the cheap end of the curve is a handful of megabytes for half the saving, and the far end is most of a gigabyte for the rest.

**This shot is deliberately awkward and a real one may not be.** Layer 2 cycling 24 drawings on ones, ten times over, is a stress case the fixture was built to be, not a description of how the owner's work is drawn. The layer-1 finding is the one that generalises: a held background is the common case, it is the expensive case, and it is the one a small cache is enough for.

None of this decides D-37. It says what each answer would buy.
