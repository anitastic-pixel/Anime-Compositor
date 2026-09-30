# D-230 rejected: entries for documents 00, 14 and 15

Written by B-160b for the coordinator to fold in. Documents 00, 14, 15 and 28 were not edited in the worktree. Every number comes from `verification/B-160_faster_export_table.md` and `verification/B-12b_diagnostic_catalogue.md` as B-160b regenerated them.

## Document 14 (decisions)

Replace D-230's status, **PROPOSED on 2026-09-29, for the owner to decide.**, with:

**REJECTED on 2026-09-29 by the owner ("trust your recommendation to reject it"); the box, the note and the card encoder are removed in B-160b.**

The rest of the entry stays as the record of what was built and measured: the card was not faster (15931 ms against 13467 ms for software, provisional) and wrote a bigger file (12.3 MB against 8.1 MB). The "For the owner" sentence can go. The proposed document 28 row, `EXPORT_VIDEO_ENCODER`, is not added, since it went in only on acceptance.

## Document 15 (backlog)

Add after B-160:

B-160b / D-230 rejected: hardware video encoding removed. **BUILT on 2026-09-29.** The owner rejected D-230 ("trust your recommendation to reject it"). Removed: the card encoder in `src/mp4_out.rs` (the file is back to what it was before B-160), the `hardware_video` choice and its MP4 branch in `src/export.rs`, `hardware=on` in `app/src/main.rs`, the `EXPORT_VIDEO_ENCODER` note in `src/diagnostics.rs`, and Preferences' **Export** heading with its **Hardware video encoding** box in `app/ui/index.html`. D-231 is untouched. `verification/B-160_faster_export_table.md` is now 18 of 18 (the four D-230 checks are gone), still 0 of 28684 files and reports changed against the build before B-160, so every MP4 is the software encoder's file, as before. `verification/B-12b_diagnostic_catalogue.md` is 197 of 197 (was 200) and `verification/B-12c_keyboard_table.md` no longer lists `prefhwvideo` (81 controls). The playtest sheet `verification/B-160_playtest.md` keeps steps 1 to 3 only.

In B-160's own entry, "and hardware video encoding (D-230, proposed)" and the sentences about the card encoder and the Preferences box describe what B-160b removed; "22 of 22" becomes "18 of 18 since B-160b", and "**D-230 awaits the owner's decision, and the playtest awaits the owner.**" becomes "**D-230 was rejected (B-160b); the playtest awaits the owner.**"

## Document 00 (start here)

Replace "with hardware video encoding a Preferences box, off by default, awaiting the owner's decision (D-230)" with:

"hardware video encoding rejected by the owner and removed, since the card was no faster and wrote bigger files (D-230, B-160b)"
