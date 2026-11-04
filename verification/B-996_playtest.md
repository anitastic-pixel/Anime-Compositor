# B-996 playtest: autosave writes its copy without holding up frames (D-1120)

What changed: every two minutes while a project has unsaved changes, the program writes a
recovery copy. Before, it held the lock every frame needs for the whole write: the project as
text and then to disk. Now it takes a copy of the project under the lock and writes after letting
go. The copy written is the same file, byte for byte.

## Before and after

Busy machine (other lanes' builds running): these timings are not final. AMD Ryzen 9 9900X,
RTX 4070 Ti SUPER, Windows 11 Education 10.0.26300; release build, `CARGO_TARGET_DIR=target/lanes/p27b`,
`CARGO_INCREMENTAL=0`, `-j 4`; p20 (`tests/p20_edit_cost.rs`) at commit a0e2fa02, the same run as
B-995's, medians of 21 to 51 runs.

| Frames held back by one autosave | 4 layers | 50 layers | 200 layers |
|---|---|---|---|
| Before: writing the text under the lock (p20 "save text"), plus the disk write | 0.80 ms | 36.1 ms | 143.0 ms + disk |
| After: copying the project under the lock (p20 "copy of the project") | 0.009 ms | 0.053 ms | **0.213 ms** |

p21 does not wait the two minutes an autosave needs, so it has no row here.

## Try it

1. Open a big project (100 or more layers), change something and do not save.
2. Start playback or a RAM preview and let it run for five minutes, or drag a layer around now
   and then over that time.
   - Expect: no short freeze every two minutes; before, playback or the drag hitched each time
     the status line said "Recovery snapshot written to ...".
3. Close the program without saving, open it again and open the project.
   - Expect: it offers the recovery copy, and the copy holds your change, as before.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `app/src/tests/p27_speed.rs`, `d1120_autosave_writes_the_same_file_outside_the_lock`:
on the 200-layer copy the file written outside the lock is byte for byte the text the old code
wrote, and a rename made while it was being written is not in it (the lock was free). The
existing autosave tests (`recovery_and_autosave`, `unsaved_recovery_check`) pass unchanged.
