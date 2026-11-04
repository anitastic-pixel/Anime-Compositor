# B-999 playtest: undo steps share the layers an edit did not change (D-1118, part a)

What changed: every undo step used to keep a whole copy of the project. Now each layer is held
once and shared: a step keeps only the layer (or layers) its edit changed, plus the composition
around them. Undo and redo give back exactly the same project as before.

The memory ceiling (D-1118 part b) is split off as follow-up D-1137.

## Before and after

Memory counts do not depend on how busy the machine is; times do (busy machine, not final).
AMD Ryzen 9 9900X, RTX 4070 Ti SUPER, Windows 11 Education 10.0.26300; release build,
`CARGO_TARGET_DIR=target/lanes/p27b`, `CARGO_INCREMENTAL=0`, `-j 4`; p20 (`tests/p20_edit_cost.rs`),
counting allocator, 20 edits; times are medians of 51 runs. Before: a0e2fa02 (B-995's run).
After: this commit.

| | 4 layers | 50 layers | 200 layers |
|---|---|---|---|
| Memory one undo step keeps, before | 0.03 MB | 0.42 MB | 1.70 MB |
| Memory one undo step keeps, after | **0.01 MB** | **0.02 MB** | **0.04 MB** |
| One edit (`Document::apply`), before | 0.022 ms | 0.131 ms | 0.532 ms |
| One edit (`Document::apply`), after | 0.016 ms | 0.025 ms | **0.062 ms** |
| One drag step (`update_drag`), before | 0.008 ms | 0.053 ms | 0.216 ms |
| One drag step (`update_drag`), after | 0.007 ms | 0.013 ms | **0.030 ms** |

A day of 2,000 edits on the 200-layer project would keep about 74 MB of undo, not 3.4 GB.

## Try it

1. Open a big project (100 or more layers) and look at the program's memory in Task Manager.
2. Make 50 small edits (nudge a layer with the arrow keys, change a value, rename).
   - Expect: memory hardly grows; before, it grew by about 1.7 MB an edit at 200 layers.
3. Press Ctrl+Z 50 times, then Ctrl+Shift+Z 50 times.
   - Expect: every step goes back and forward exactly as before, and the picture after the
     last redo is the picture you had after step 2.

Pass if every "Expect" holds. If an undo gives back a different value, say which step.

Checked by: `tests/p03_byte_equality.rs` (the 960-line picture manifest byte for byte the one
from before this lane's picture changes), the document 26 undo tests `tests/b05_model.rs` and
`tests/b09_persistence.rs`, the core's own tests, and the app suite.
