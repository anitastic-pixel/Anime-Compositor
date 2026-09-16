# T-10, the other half: W-01 with the adapter off

`verification/B-11_offline_table.md` reads the build and says what could reach a network, nine
checks. `verification/B-11_offline_run.md` watches the running program and says what did.
Neither of them can disconnect the machine, and R-11 asks for something neither answers: that a
person can create, edit, save and export a shot **when there is nothing to talk to**. A program
that is quiet on a connected machine can still be one that waits, retries or refuses on a
disconnected one, and no test in this repository can tell the difference.

This is the sheet for doing it by hand. It is written and has not been performed. Nobody should
record a result here from reading the build.

## Before you start

1. Build the program: `cargo build --release -p anime_compositor_app`. Do this **while still
   connected**, so that a failure to build is not mistaken for a failure to run offline.
2. Note the build and the date at the bottom of this sheet.
3. **Now disconnect.** Pull the cable, or disable the adapter in Windows: Settings, Network and
   Internet, Advanced network settings, and Disable on every adapter listed, wireless included.
   Turning wireless off at a hardware switch is enough; airplane mode is enough.
4. Confirm it is really off before the program is started, in whatever way you would normally
   confirm it - a browser that now fails to load a page is a fine check, and is the last thing
   to do before starting.

The point of the order is that the program is started for the first time **after** the machine
is already offline. A program that reaches the network once at startup and caches the answer
would pass a test that disconnects afterwards.

## The run

Run W-01 as `verification/B-12b_w01_walkthrough.md` describes it, and answer these as you go.
Each one is a thing that either happened or did not.

| # | What to do | What should happen | What happened |
|---|---|---|---|
| 1 | Start the program | The window opens, in no more time than it usually takes. A noticeably slower start is a result, not a nuisance: it means something waited for a timeout | |
| 2 | Watch for anything asking to sign in, register, activate, or check for updates | Nothing does, at any point in this run | |
| 3 | Import the reference shot's drawings | They import, with the same warnings about the deliberate gap as always and no others | |
| 4 | Scrub the timeline and play | The picture appears and plays as it does connected | |
| 5 | Edit: move a layer, change an exposure, add an effect | Every edit takes, and undo puts it back | |
| 6 | Save, with Ctrl+S | The file is written where it says it is | |
| 7 | Close the program and open the saved file again | It opens and holds what was saved | |
| 8 | Export a frame range to PNG | The files are written, and the progress does not stall | |
| 9 | Look at everything the window said during the run | No message mentions a network, a server, a connection, an account, or being offline. **A message saying the program is working offline is a failure of this test**, because the program should not have known | |
| 10 | Anything that felt slow, stuck, or different from the connected run | Nothing did | |

## Afterwards

Reconnect, and write what happened in the last column. If every row holds, T-10 is RUN and
PASSING and the plan should say so. If any row does not, the row is the defect report: what was
being done, what was expected, what happened instead.

One thing this run cannot settle, and should not be blamed for: Microsoft's web view component
lives inside this window and is not this project's code. `verification/B-11_offline_run.md`
records that it held connections of its own on a connected machine, which is registered as D-39
for the owner. With the adapter off it has nothing to hold. If the window is slow to appear at
step 1, that component waiting is the likeliest cause and is worth writing down as such rather
than as a defect in the compositor.

---

Build: _not yet run_
Date: _not yet run_
Performed by: _not yet run_
Result: _not yet run_
