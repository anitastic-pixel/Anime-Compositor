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

**Projects.** Opens and saves them — the toolbar, `Ctrl+O`, `Ctrl+S`, `Ctrl+Shift+S`, the recent
list, or dropping a project file onto the window.

**Compositions.** Makes one — **New composition…** or `Ctrl+Shift+N` — with a name, a size, a
frame rate and a length, and puts the window into it. The project panel lists the compositions a
project holds and clicking one puts the window there, so a project can hold several shots and you
can move between them. Making a composition is undoable; moving between them is not, because
looking somewhere else does not change the project. The sizes this build will make stop at 16384
on a side and at 67,108,864 pixels in all, which is 8192 by 8192, or 16384 by 4096, and not 16384
square; a composition stops at 10,000 frames. A request past any of those is refused in a sentence
that says which limit it crossed and, where the width is legal, the tallest height that width
allows.

**Drawings.** Imports a folder of numbered PNGs as one sequence — **Import drawings…** or
`Ctrl+I` — and tells you how many drawings it found, which numbers they run between, and which
numbers are missing from the run. **Relink drawings…** points a sequence at a folder that has
moved, and shows you what the relink would do before it does it.

**Building a shot.** Add and delete layers, rename one (`F2`), move one forward or back
(`Ctrl+]`, `Ctrl+[`), hide one, lock one. Give a layer its exposure sheet — which drawing is on
screen for which frames. Set a layer's anchor, position, scale, rotation and opacity, by typing
a number or by dragging its label; a whole drag is one thing to undo, not thirty. Use one layer
as another's track matte, showing the matte layer or not. Add, bypass and delete effects, which
run in the order they were added: this build has three, an exposure adjustment, a Gaussian blur
and a tint.

**Undo.** Everything above, backwards and forwards, `Ctrl+Z` and `Ctrl+Shift+Z`, with the window
naming what it is about to undo rather than saying "undo".

**Watching it.** Shows any frame, steps through frames, and plays in real time. Switches between
a draft preview and full resolution, and says on screen which one you are looking at — a draft
preview is never described as final pixels. Shows the alpha channel on its own, and turns the
transparency grid off.

**Autosave.** Saves a copy of itself in the background while a project has unsaved changes, and
says when it last did. The copies rotate through five slots beside the project and never touch
the file you saved yourself. If the program stops without saving, the next start offers those
copies newest first; choosing one opens it as unsaved work against the project on disk, so the
file is only overwritten if you save it.

**Export.** Writes the shot as a PNG sequence — **Export…** or `Ctrl+M`, into a folder you
choose. Every frame is written at full size, whatever resolution the preview happens to be
showing, named for the project and the frame number. It exports the shot as it was when you
asked, so changing something while it runs does not change what is being written, and you can
stop it: the frame being written is finished and the rest are not started.

**Saying when something is wrong.** A missing drawing, an effect this build does not have, a
project written by a newer version — it says so in words, and keeps what it does not understand
rather than dropping it. A project saved by this build still contains everything the project had
when it arrived, including the parts this version cannot read.

## What it does not do yet

- **A composition's size, rate and length are fixed once it is made.** There is a control that
  makes one and no control that changes one afterwards, so a shot that needs a different size is
  a new composition rather than an edited one.
- **It cannot draw a mask or make a keyframe.** Both exist in the file format, both are drawn
  correctly when a project arrives with them, and both survive a save. There is simply no
  control in this window that creates one, so everything you set here holds for the whole shot.
- **It cannot change a blend mode or the order of a layer's effects.** The layer panel shows
  which blend mode a layer has — normal, multiply, screen or add — and all four are composited
  correctly, but the value comes from the file. Effects run in the order they were added and
  there is no way to move one.
- **Three effects.** An exposure adjustment, a Gaussian blur and a tint. A project using any
  other effect keeps it, renders without it, and says which one it skipped.
- **An export stops on a missing drawing, and there is no progress bar.** If any frame in the
  range needs a drawing that is not on disk, nothing is written and you are told which frames and
  what to do about it; the checkbox beside the button writes them anyway, with the missing
  drawings still reported. While an export runs the window says so and offers to stop it, but it
  does not count the frames as they are written.
- **Autosave is not a save.** It waits two minutes after a change and then keeps up to five
  copies; anything newer than the last copy is not in it, and nothing it writes replaces your
  file. Save deliberately.
- **Previews are drawn on the graphics card when there is one; exports are drawn on the
  processor.** Video is exported as MP4 (H.264) over black, with no sound.
- **Whether a shot previews at full speed depends on whether its drawings fit in memory.** The
  speed this project measures itself against is 24 frames a second, which allows 41.7 ms a
  frame. The viewer keeps decoded drawings and effect results in memory, up to a quarter of the
  machine's memory unless **Preferences...** says otherwise, and never less than 1 GiB. Nine
  sixteenths of that is for drawings. It is a ceiling, not a reservation: a light shot holds far
  less, and nothing is held while exporting. Measured in draft on a machine with 64 GB and an
  NVIDIA RTX 4070 Ti SUPER, where the default is 15.4 GiB:
  - The four-layer reference shot needs 1.7 GiB of drawings. It plays at 0.5 ms a frame, and a
    jump anywhere in it takes 1.3 ms.
  - The shot the 24 fps target is written against (ten drawings a frame, two track mattes and
    three effects, at 1920 by 1080) needs 5.1 GiB. It plays at about 5 ms a frame, and a jump
    takes about 7 ms.
  - At 1 GiB, which is all a machine with 4 GB or less gets, the four-layer shot still keeps up,
    at 14 ms a frame. The ten-layer shot takes 36 ms a frame, and 48 to 63 of its 240 frames
    miss the deadline each time round, because its drawings are read from disk again.

  A shot whose drawings do not all fit sits between those two rows. On a 16 GB machine the
  default holds 2.25 GiB of drawings; on a 32 GB machine, 4.5 GiB. Raise the setting if a shot
  drops frames. What you see is always a shot playing at the right *speed*, with frames missing
  if it cannot keep up, never a shot playing slowly, and the window counts the dropped frames on
  screen. `verification/B-114_viewer_envelope.md` has the measurements, and D-174 in
  `Markdown/14_Decisions_Risks.md` is the decision.
- **No screen-reader support has been checked.** Display scaling, the keyboard and non-English
  text have been; `verification/B-11_display_and_keyboard.md` and
  `verification/B-12a_window_and_keyboard.md` show how far that goes.

## About the network

**This program does not use the network.** It contains no web client, nothing it reads or writes
leaves the machine, and nothing about it needs an account. It was watched while running to
confirm that, and its own process held no connection at all.

**The window is a different matter, and this has to be said plainly.** The window is drawn by
Microsoft's WebView2, which is part of Windows and not part of this program. While the
application sat idle for thirty seconds, that component held two encrypted connections of its
own, to a Microsoft address. Eight switches are set at startup to quieten it. They removed the
other two connections it used to hold, but they cannot reach this pair, which is made outside the
part of the web view the switches control. It is Microsoft's code, it runs inside this window,
and nothing this project can write will prove what does or does not travel over those
connections.

The measurements are in `verification/B-11_offline_run.md` and `verification/B-113_offline_run.md`.
What to do about it is an open decision, D-39, in `Markdown/14_Decisions_Risks.md`. If it matters
to you, block `msedgewebview2.exe` outbound in the Windows firewall. The application itself does
not need the network and will not notice. Other programs that use WebView2 run the same file,
though, and they will be blocked too.

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
