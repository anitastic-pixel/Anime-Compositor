# B-997 playtest: after an edit the window sends the page only what changed (D-1116)

What changed: after every edit the page used to read the whole project back from the window,
3.4 MB of text at 200 layers, for a change to one value. Now the page says which copy it holds
and the window answers with only the layers (and composition settings) that changed since that
copy. If the page's copy is not the last one sent, or anything outside the compositions changed
(assets, folders, a composition added or removed), the window sends the whole project, as before.

## Before and after

Busy machine (other lanes' builds running): these timings are not final. AMD Ryzen 9 9900X,
RTX 4070 Ti SUPER, Windows 11 Education 10.0.26300; release build, `CARGO_TARGET_DIR=target/lanes/p27b`,
`CARGO_INCREMENTAL=0`, `-j 4`; `tools/p21_page_cost.py`, background copy with its own web view
profile, 7 runs each, medians. Before: the build at 2c754ed5 (B-996). After: this commit's build.
Both run back to back with nothing else of this lane running.

| After one rename | 4 layers before | 4 layers after | 200 layers before | 200 layers after |
|---|---|---|---|---|
| Text the page is sent (`delta_bytes`) | 33,303 | **1,070** | 3,437,378 | **20,743** |
| Time to fetch it (`delta_fetch`) | 2.9 ms | 1.3 ms | 159.6 ms | **116.6 ms** |
| The page's whole edit (`edit_total`) | 11.6 ms | 12.4 ms | 1,992 ms | **1,627 ms** |

What is left of the fetch at 200 layers is the window still writing the whole project to see
what changed; that is follow-up D-1135. The rest of the edit is the page redrawing every panel,
which is D-1117 (B-998).

## Try it

1. Open a big project (100 or more layers).
2. Rename a layer, change a value in the Effect Controls, move a layer up or down, duplicate one,
   delete one, then press Ctrl+Z a few times and Ctrl+Shift+Z once.
   - Expect: after each, every panel shows what it showed before this change (names, values,
     order, the History list), and the program answers a little sooner than before.
3. Import a picture, then add a layer from it.
   - Expect: the Project panel and the timeline show it at once, as before.

Pass if every "Expect" holds. If any panel shows an old name or value after an edit, say which
step.

Checked by `app/src/tests/p27_speed.rs`, `d1116_the_page_copy_built_from_changes_equals_a_full_read`:
on the reference shot and the 200-layer copy, 14 edits (rename, value, label, solid, move,
duplicate, delete, undo, redo, solo), and after each the page's own `mergeState`, run under Node,
builds a copy equal, as JSON, to a full read. `B-12b_state_fields_table.md` gains one row: the
page now reads the answer's `token`, and it is there (659 of 659).
