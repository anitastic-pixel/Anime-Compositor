# B-161: decoded drawings kept on disk, and what they must not change

D-232 (owner delegated 2026-09-29): the first time the viewer decodes a drawing it also keeps the decoded pixels on disk, and later - this session or after a restart - it reads that copy instead of decoding the PNG again. That is only allowed if the copy is the decode, exactly. This table is produced by `tests/b161_decode_cache.rs` and asks that question on every image under `Fixtures/`, the reference shot's 56 drawings among them.

Read the first rows first: they compare every bit of every sample of a fresh decode with the copy read back. The damaged-copy rows break a copy five different ways and check that the viewer notices, throws the copy away, decodes the drawing again with the same bits, writes one line for the session log, and writes a whole copy back. The last rows render real frames of the reference shot with the copies on and off.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| Image files under Fixtures/ that were read (listed from the folder) | 412 | 412 | pass |
| A drawing read back from its disk copy is the fresh decode, every bit of every sample | 403 of 403 | 403 of 403 | pass |
| The read that wrote the copy hands back the fresh decode too | 403 of 403 | 403 of 403 | pass |
| The second read was answered by the disk copy, not by decoding (every image but the EXRs) | 361 | 361 | pass |
| The EXRs among them are decoded every time and no copy is written for them | 42 EXRs, 0 copies | 42 EXRs, 0 copies | pass |
| The reference shot's 56 drawings, of those: read back bit for bit | 56 of 56 | 56 of 56 | pass |
| A file the decoder refuses is refused the same way with copies on, and no copy is written for it | 9 of 9 | 9 of 9 | pass |
| A copy cut to half its length: decoded fresh with the same bits, one session-log line, written whole again | decoded, not from disk, 1 line, whole | decoded, not from disk, 1 line, whole | pass |
| A copy one byte of one pixel changed: decoded fresh with the same bits, one session-log line, written whole again | decoded, not from disk, 1 line, whole | decoded, not from disk, 1 line, whole | pass |
| A copy its first bytes overwritten: decoded fresh with the same bits, one session-log line, written whole again | decoded, not from disk, 1 line, whole | decoded, not from disk, 1 line, whole | pass |
| A copy emptied to nothing: decoded fresh with the same bits, one session-log line, written whole again | decoded, not from disk, 1 line, whole | decoded, not from disk, 1 line, whole | pass |
| A copy a width in its header that its length disagrees with: decoded fresh with the same bits, one session-log line, written whole again | decoded, not from disk, 1 line, whole | decoded, not from disk, 1 line, whole | pass |
| A drawing repainted under the same name is decoded again, and the new drawing is what is shown | decoded, the new drawing's bits | decoded, the new drawing's bits | pass |
| The same drawing given a later time is decoded again | 0 | 0 | pass |
| The same file read under a second interpretation has a copy of its own, read back bit for bit | decoded, then from disk, same bits as fresh | decoded, then from disk, same bits as fresh | pass |
| One 16x16 copy is 32 + 16 x 16 x 4 bytes | 1056 | 1056 | pass |
| With a cap of two copies, a third leaves two | 2 copies, 2112 bytes | 2 copies, 2112 bytes | pass |
| The one let go is the one used longest ago (B), not the one written first (A) | A kept, C kept | A kept, C kept | pass |
| A file in the folder that is not a copy is left alone | true | true | pass |
| Four full frames of the reference shot, first session (copies written): same bits as no copies | 33177600 | 33177600 | pass |
| The same four frames in a second session (copies read): same bits as no copies | 33177600 | 33177600 | pass |
| The second session decoded nothing: every drawing it needed came from a copy, read ahead or not | 12 decodes asked for, 12 answered by copies | 12 decodes asked for, 12 answered by copies | pass |
| The cache export uses has no disk copies to read (CelCache::none) | 0 | 0 | pass |

23 of 23 checks pass.

## What is not checked here

- The speed. It is a measurement, in `verification/B-161_decode_cache_timing.md`.
- A drawing replaced by a different one of exactly the same length and exactly the same modified time. Document 27 accepts length and time as the identity for interactive work, as B-08b's memory cache does; a copy tool that keeps times, putting back an older drawing of exactly the same length, would be missed.
