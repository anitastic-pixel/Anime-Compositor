# B-777: drawings kept at 8 bits, widened when asked for

Written by `tests/b777_compact_cels.rs` (`cargo test --test b777_compact_cels`), for D-897. **12 of 12 pass.**

"Identical" is every bit of every sample (`f32::to_bits`), against `CelCache::none()`, which keeps nothing and decodes every time, as export does.

| Check | Expected | Actual | |
|---|---|---|---|
| Every fixture image that decodes, read four ways, twice: both answers are a fresh decode's, bit for bit | 3620 of 3620 | 3620 of 3620 | pass |
| and an 8-bit drawing's second answer came from memory with room for it at 8 bits only | 3316 of 3316 | 3316 of 3316 | pass |
| An EXR is floats already and stays floats, so with room for 8 bytes a pixel it is not kept | 0 of 304 | 0 of 304 | pass |
| All 65,536 colour and alpha byte pairs, Srgb Straight: widened from memory, bit for bit a fresh decode | from memory, identical | from memory, identical | pass |
| All 65,536 colour and alpha byte pairs, Srgb Premultiplied: widened from memory, bit for bit a fresh decode | from memory, identical | from memory, identical | pass |
| All 65,536 colour and alpha byte pairs, LinearLight Straight: widened from memory, bit for bit a fresh decode | from memory, identical | from memory, identical | pass |
| All 65,536 colour and alpha byte pairs, LinearLight Premultiplied: widened from memory, bit for bit a fresh decode | from memory, identical | from memory, identical | pass |
| Reference shot frames 0, 12, 14, 25, 100, 101 at Full, twice round, from a cache of four 8-bit drawings: the uncached frame, bit for bit | 12 of 12 | 12 of 12 | pass |
| and that cache answered from memory on the second round | hits > 0 | hits > 0 | pass |
| The same frames from a cache with room for some drawings at floats too: the uncached frame, bit for bit | 12 of 12 | 12 of 12 | pass |
| A drawing widened again after its floats were let go keeps the name the card holds it under | named, the same name | named, the same name | pass |
| and while a frame still holds those floats, asking again hands back the same floats, not a second copy | the same floats | the same floats | pass |
