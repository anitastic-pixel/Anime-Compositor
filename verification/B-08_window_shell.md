# B-08, the window shell

The artifact is `B-08_window_shell.png`: a photograph of the application running.

![the shell window](B-08_window_shell.png)

## What it shows, and what it does not

An actual window, with a title bar, an icon, and a page rendering inside it. That is the whole
claim. **Nothing is drawn in the window yet** — no frame, no timeline, no controls. The page says
so itself, so a screenshot of it can never be mistaken for a working viewer.

That is the point of doing it this way round. The shell brought the dependency graph from 28
crates to 264, and the licence record and archive that go with them had to be rebuilt for that
size. Landing the window empty makes the licence work reviewable on its own, and makes the next
change — pixels arriving over D-36's transport — reviewable on its own too.

Captured at 1522x1016 physical pixels. The window asks for 1000x640; the display it was
photographed on runs at 150%, which is where the difference comes from.

## How this artifact is made, and why it is the odd one out

Everything else under `verification/` is written by `cargo test`, and CI runs
`git diff --exit-code -- verification/` to require that the committed copy still matches what the
build produces. This one is not, because a window is not a value a test can compare, and CI has
no screen to open one on.

It is captured by `tools/capture_window.ps1`, which starts the application, waits for the window
to appear, photographs it and stops the process again. To reproduce it:

```
cargo build -p anime_compositor_app
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1
```

The picture will differ from the committed one — window placement, display scaling and the theme
of the title bar are all properties of the machine rather than of the build — so this file is not
compared byte for byte by anything. It is evidence that the window opened on 2026-09-06, not a
fixture.

## What is checked automatically

The shell is a workspace member, so every CI gate covers it: it builds, it passes clippy with
warnings denied, and its 264 dependencies are recorded in `docs/DEPENDENCIES.md` and archived
under `Licenses/` with `tools/archive_licenses.py --check` enforcing that the archived texts are
the ones the crates actually ship. See `B-11_record_table.md` and `B-11_license_archive.md`.

The one thing no gate covers is whether the window opens, and this file is why that gap is
visible rather than silent.
