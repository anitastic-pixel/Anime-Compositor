# Read me first — what this build is, and what it is not

Anime Compositor 0.1.0, a portable folder. Copy it anywhere and run **Anime Compositor.exe**.
There is no installer; the section at the bottom says why.

This file is the supported envelope: the honest boundary of what this program does today. It is
written to be read before use, not after something goes wrong.

## What it needs

- **Windows 11 on a 64-bit AMD or Intel machine.** Nothing else has ever been built or tested.
  ADR-001 records that decision; the dependency record beside this file describes the Windows
  build only, for the same reason.
- **The Microsoft Edge WebView2 Runtime**, which Windows 11 ships with. The window is drawn by
  it. If it has been removed, the application will start and no window will appear.
- Nothing else. No account, no sign-in, no licence key, no installation, no registry entries.
  Deleting the folder removes the program.

## What it does

- Opens and saves projects — the toolbar, `Ctrl+O`, `Ctrl+S`, `Ctrl+Shift+S`, the recent list,
  or dropping a project file onto the window.
- Shows any frame of the shot, steps through frames, and plays in real time.
- Switches between a draft preview and full resolution, and says on screen which one you are
  looking at. A draft preview is never described as final pixels.
- Says out loud when something is wrong with a project — a missing drawing, an effect this build
  does not have — and keeps what it does not understand rather than dropping it. A project saved
  by this build still contains everything the project had when it arrived, including parts this
  version cannot read.

## What it does not do yet

- **No export from the window.** The renderer exports whole shots, and it is checked frame by
  frame — but there is no button for it here yet. Exporting is available only to the tests.
- **No masks and no effects.** Both are deliberately parked, not missing by accident. A project
  that has them keeps them and renders without them, and says so.
- **No autosave and no crash recovery in the window.** Save often.
- **No editing.** This is a viewer: it opens, shows, plays and saves. Nothing in the window
  changes a project.
- **No GPU rendering, no video files.** Frames are composited on the processor and written as
  images.
- **No screen-reader support has been checked.** Display scaling, the keyboard and non-English
  text have been; `verification/B-11_display_and_keyboard.md` shows how far that goes.

## About the network

**This program does not use the network.** It contains no web client, nothing it reads or writes
leaves the machine, and nothing about it needs an account. It was watched while running to
confirm that, and its own process held no connection at all.

**The window is a different matter, and this has to be said plainly.** The window is drawn by
Microsoft's WebView2, which is part of Windows and not part of this program. While the
application sat idle for thirty seconds, that component held four encrypted connections of its
own — one pair to this machine's internet provider's DNS service, one pair to an unnamed address.
Six switches are set at startup to quieten it and they did not stop it. It is Microsoft's code,
it runs inside this window, and nothing this project can write will prove what does or does not
travel over those connections.

The measurements are in `verification/B-11_offline_run.md` and what to do about it is an open
decision, D-39, in `Markdown/14_Decisions_Risks.md`. If it matters to you, block
`msedgewebview2.exe` outbound in the Windows firewall; the application itself does not need the
network and will not notice.

## Licences

This program is offered under **MIT OR Apache-2.0** — take either. Both texts travel with it as
`LICENSE-MIT` and `LICENSE-APACHE`.

It also contains 271 open-source crates. `DEPENDENCIES.md` lists every one of them with its
version and its licence, generated from the build rather than written by hand, and the full
licence and notice text of each is in `Licenses/`. Nothing there has been reviewed by a lawyer:
the record deliberately reaches no legal conclusion, and the entries that would need one —
copyleft terms, conjunctive licences, crates that declare a licence and ship no text — are
flagged in `DEPENDENCIES.md` for a reviewer rather than decided.

## Why there is no installer

An installer is a different problem from a program: it needs a package format, a signing
certificate, an upgrade story and somewhere to be downloaded from, and none of those exist yet.
`app/tauri.conf.json` leaves bundling switched off rather than producing an unsigned installer
that Windows would warn about.

What that costs you: no Start menu entry, no uninstall entry, no file association, and Windows
will most likely warn once about an unrecognised program the first time it runs. What it buys is
that the folder is the whole program and nothing has been put anywhere else on the machine.
