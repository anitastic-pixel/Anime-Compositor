# D-1140: export into a named folder, only in a window started for testing

Written by `d1140_export_into_only_in_a_window_started_for_testing` in `app/src/main.rs`. `export_into` is what `/export` reads `into=` with; the window is started for testing when `ANIME_COMPOSITOR_TEST_EXPORT` is set. Without `into=` the route asks for the folder in the system's dialog, as before. Folder names below are this machine's scratch folder.

**15 of 15 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a person's window: into= is refused | refused | refused | PASS |
| a person's window: the refusal says why | only in a window started for testing | only in a window started for testing | PASS |
| a person's window: the folder named is not made | no | no | PASS |
| a person's window without into=: the folder dialog, as before | the folder dialog | the folder dialog | PASS |
| a testing window without into=: the folder dialog, as before | the folder dialog | the folder dialog | PASS |
| a testing window, a folder not there yet: accepted | into <scratch>\fresh | into <scratch>\fresh | PASS |
| a testing window: the window makes the folder | yes | yes | PASS |
| a testing window, an empty folder: accepted | into <scratch>\empty | into <scratch>\empty | PASS |
| a testing window, a folder with a file in it: refused | refused | refused | PASS |
| a testing window: the file already there is untouched | kept | kept | PASS |
| a testing window, a relative folder: refused | refused | refused | PASS |
| a testing window, into= with nothing after it: refused | refused | refused | PASS |
| a testing window, a folder name sent escaped (%20): read as the page sends it | into <scratch>\with a space | into <scratch>\with a space | PASS |
| two frames of the reference shot exported into the folder the window made | Exported 2 frames | Exported 2 frames | PASS |
| the files in it | the reference shot_0000.png, the reference shot_0001.png | the reference shot_0000.png, the reference shot_0001.png | PASS |
