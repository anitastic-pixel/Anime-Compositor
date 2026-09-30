# B-161 / D-232: text for documents 00, 14, 15 and 28

Written by the G15 worktree agent on 2026-09-29, for the coordinator to fold in. Documents 00, 14 and 15 were not edited on the branch. Document 27's entry is already on the branch, in `Markdown/27_Cache_And_Invalidation_Model.md` under Memory policy. Document 28's rows are below rather than in the document, because the two IDs are session-log lines, not `DiagnosticId` values.

## Document 14 (decisions)

D-232 / Decoded drawings kept on disk for the viewer / **ACCEPTED under delegation, bit-exact; owner delegated 2026-09-29** ("let's add all of these as well into queue, but defer if not to our standard/to your discretion."). Built in B-161. Item G15 of the GPU plan.

- **What.** The first time the viewer decodes a PNG or other 8-bit drawing, the decoder's own bytes (8-bit sRGB straight RGBA, before any arithmetic) are also written to disk. A later miss for the same key, in this session or after a restart, reads them back instead of decoding the file, then runs the same colour conversion a decode runs.
- **Bit-exact.** The same pixels to the bit on every fixture image and on the reference shot. `verification/B-161_decode_cache_table.md` has 23 of 23 checks: 402 of 402 decodable fixture images, 56 of 56 reference-shot drawings, and 33,177,600 of 33,177,600 samples of four full reference frames in a first and a second session.
- **Scope.** Preview only. `CelCache::none`, which export uses, never has a disk. EXR is not kept, because it decodes straight to floats.
- **Key.** The document 27 key: path, size, modified time, interpretation and a decoder version, hashed into the file name.
- **Location and cap.** From Preferences. The default folder is "decoded drawings" in the app's local data folder, and the default cap is 5 GB. 0 GB turns it off. Least recently used copies are deleted first.
- **Damage.** A copy that fails its length, header or checksum is deleted, decoded fresh and noted in the session log.
- **Why 8-bit bytes and not the working buffer's floats.** The first build kept the floats, 16 bytes a pixel. It was measured at 49.0 ms against 44.5 ms a frame for decoding, so it was slower than decoding. The PNGs of flat anime drawings unpack fast: about 2.3 ms of a 7 ms decode for a reference-shot drawing, and reading 33 MB back took 7.5 ms. The 8-bit copy is a quarter of the size and skips only the unpacking.
- **Measured.** 32.3 ms against 45.0 ms a frame on the reference shot (72%), release build, 7 interleaved runs, median. **PROVISIONAL**: other builds ran at the same time, and a quiet re-measure is due. Copies were read from Windows' file cache. Reading them cold after a restart was not measured.
- **Owner's to change.** The 5 GB default, and on-by-default.

## Document 15 (backlog)

B-161 / Decoded drawings kept on disk, D-232 (GPU plan G15). **Built on 2026-09-29** on branch `worktree-agent-a6c0881a510098b90`, not yet merged.

- **Code.** `src/cache.rs`: `DiskCache`, and `CelCache::set_disk`, `disk_hits` and `take_disk_notes`. `CelCache::decoded` and `prewarm` go through the disk when one is set, so read-ahead uses it too. `src/media.rs`: `decode_8bit` and `from_8bit`; `decode` is now those two steps, and its bytes are unchanged.
- **App.** `app/src/main.rs`: the `/memory` request takes `disk` in GB and `diskfolder`, the viewer's cache is given the disk, and disk notes go to the session log.
- **Page.** `app/ui/index.html`: Preferences has "Decoded drawings on disk", with `prefdisk` and `prefdiskfolder`. CONTROLS went from 78 to 80.
- **Evidence.**
  - `verification/B-161_decode_cache_table.md`: 23 of 23.
  - `verification/B-161_decode_cache_timing.md`: provisional.
  - `tests/b161_decode_cache.rs`.
- **Awaits the owner.** The playtest `verification/B-161_playtest.md`.

## Document 00 (start here)

B-161 (D-232, accepted under delegation, bit-exact) is built on 2026-09-29. The viewer keeps each decoded drawing's 8-bit pixels on disk, 5 GB by default in the app's data folder, set in Preferences. The first play of a shot after a restart is quicker, and every pixel is unchanged. On the reference shot that was 32 ms against 45 ms a frame, provisional. Its playtest, `verification/B-161_playtest.md`, awaits the owner.

## Document 28 (diagnostics): two session-log IDs

| ID | Severity | Meaning | Required behavior |
|---|---|---|---|
| DECODE_CACHE_DISCARDED | INFO | A disk copy of a decoded drawing (B-161) failed its length, header or checksum check | delete the copy, decode the drawing fresh (pixels unchanged), write a whole copy again; one session-log line naming the file (name only) and the reason; nothing on screen |
| DECODE_CACHE_NOT_WRITTEN | INFO | A decoded drawing could not be written to the disk cache (disk full, folder not writable) | show the decoded drawing as usual; one session-log line; it is decoded again next time |

Both are lines in the P-19 session log's `warnings`, only while the log is switched on. They are not `DiagnosticId` values, because a preview's pixels are unchanged either way.
