# D-256: recovery copies of a project that was never saved

Written by `d256_a_never_saved_project_is_copied_and_offered_again` in `app/src/main.rs`. The app's own folder is stood in for by `target/d256_unsaved`; the window uses `%LOCALAPPDATA%\dev.anitastic.anime-compositor\Unsaved`.

**16 of 16 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| with no folder for unsaved copies, nothing is written, as before | nothing | nothing | PASS |
| the first look at the new, unsaved project writes nothing yet | nothing | nothing | PASS |
| nor 119 seconds later | nothing | nothing | PASS |
| two minutes after the first change, a copy is written into the app's own folder | Recovery snapshot written to /target/d256_unsaved/Unsaved project.autosave-0.json | Recovery snapshot written to /target/d256_unsaved/Unsaved project.autosave-0.json | PASS |
| the copy is on the disk | true | true | PASS |
| nothing is written beside the drawings | 0 copies there | 0 copies there | PASS |
| the project is still unsaved: the copy is not a save | true | true | PASS |
| seven copies later, five are kept | 5 | 5 | PASS |
| the next start offers the copies on the recovery list, newest first | 5, newest /target/d256_unsaved/Unsaved project.autosave-1.json | 5, newest /target/d256_unsaved/Unsaved project.autosave-1.json | PASS |
| and says so | true | true | PASS |
| a snapshot from anywhere else is still refused, as before | There is no project to recover into. | There is no project to recover into. | PASS |
| recovering the newest copy | Recovered /target/d256_unsaved/Unsaved project.autosave-1.json | Recovered /target/d256_unsaved/Unsaved project.autosave-1.json | PASS |
| gives an unsaved project: no file of its own, and work outstanding | no file, unsaved | no file, unsaved | PASS |
| saved into the same folder, it is the same project as the original, byte for byte | 3297 bytes, the same | 3297 bytes, the same | PASS |
| and its frame 10 is the original's, every byte | 8294400 bytes, the same | 8294400 bytes, the same | PASS |
| once it has been saved, the copies are removed | 0 | 0 | PASS |
