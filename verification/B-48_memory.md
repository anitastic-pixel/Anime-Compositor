# B-48: Auto drawing, and memory sized to the machine

Built on 2026-09-26 at the owner's "accept D-105", against the B-48 entry in document 15. D-105, accepted before it was built, is the decision it implements.

## What it does

**Draw on: Auto, GPU or CPU.** The viewer's old button, "Draw on GPU" / "Draw on CPU", is now a choice of three in the same place. The window remembers the choice between launches, and it starts on **Auto**.

- **Auto** draws on the graphics card whenever it can.
  - A frame the card does not draw yet, such as one with an adjustment layer, is drawn on the CPU on its own, as before, and the picture's label says "drawn on CPU, not GPU".
  - If the card itself fails a frame, Auto stops using it for the rest of the session and says so once in the status line: "The graphics card failed, so Auto draws the preview on the CPU for the rest of this session…". The choice's tooltip says so too. Choosing GPU tries the card again.
- **GPU** uses the card on every frame, even after it fails. This is how the button behaved before.
- **CPU** never uses the card.

Exports are drawn on the CPU whatever is chosen. That is ADR-006, and it is what keeps them byte-exact.

**Memory: Automatic or Custom**, in **Preferences**. Each amount is the most the preview may keep, not memory taken away from other programs.

| | Before | Automatic | Custom may go up to |
|---|---:|---:|---:|
| RAM, for drawings and effect results | 1.07 GB (D-40's 1 GiB) | a quarter of the machine's memory, never less than before: **16.5 GB** here | three quarters: **49.6 GB** here |
| Graphics card, for drawings | half its memory: 8.4 GB | the same: **8.4 GB** | 85%: **14.3 GB** here |

- The Preferences box also shows how much of each is in use at that moment.
- Custom starts from the Automatic numbers, so choosing it changes nothing until a number is changed.
- A number below 1 GiB or above the most is kept at the nearest limit.
- These belong to the window on this computer. A project file never holds them.

## The checks

`verification/B-48_memory_table.md`, written by `tests/b48_memory.rs`. **10 of 10 pass.**

- Windows reports this machine's memory: 66.1 GB, which is 61.5 GiB.
- Automatic gives the preview 16.5 GB of RAM and the card 8.4 GB. The card opens with that.
- The cache every earlier table measured is unchanged at 1 GiB, so those tables still compare with each other.
- A cache made smaller while in use lets go of what no longer fits. Holding 2.1 GB and then made 64 MiB, it held no more than 64 MiB.
- **The picture does not change.** Frames 0, 100 and 239 at Full, asked for twice from a 1 GiB cache and twice from an Automatic one, are the same bytes as the first time. Document 27 requires this: a cache may change the time, never the picture.

`verification/B-44_gpu_window_table.md`, written by the app's tests, checks the choice through the same function every frame on screen comes from. **16 of 16 pass.**

- Choosing Auto says the card is used whenever it can be, and the next frame says "drawn on GPU".
- After a failed frame, Auto gives the card up, says so once and not again, the next frame says "drawn on CPU", and the tooltip says why.
- Choosing CPU goes back to the CPU. Its picture is the same bytes as before the card was used.

**A limit of that check:** no real card failure was produced. The test tells Auto the card failed a frame, then checks what Auto does. The card has not failed on this machine.

## Speed

`verification/B-48_memory_timing_table.md`, written on 2026-09-26 by `cargo test --release --test b48_memory -- --ignored`.

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD processor with 24 threads; 66.1 GB of memory; Windows 11; release build.
- **What was timed:** the reference shot with B-47's three Blooms. Every frame is asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture.
- **How:** each row starts with empty memory and plays the shot twice. The table is the second loop, what playing it again costs, as medians over all 240 frames in ms. The first loop is in the table too, and within 2 ms of these figures.

| Quality | Drawn on | 1 GiB (before) | Automatic (16.5 GB) |
|---|---|---:|---:|
| Draft | CPU | 17.4 | 4.7 |
| Draft | GPU | 17.4 | 4.2 |
| Full | CPU | 171.0 | 16.7 |
| Full | GPU | 28.2 | **3.5** |

- **At Full on the card, a frame takes 3.5 ms instead of 28:** eight times faster.
- **On the CPU it is ten times faster:** 17 ms instead of 171.
- **Why:** the shot needs about 3.3 GB to hold every drawing it reads and every effect result it makes. 1 GiB could not hold that, so each frame threw away what the next one needed and read and computed it all again. That is why playing it a second time used to be no faster than the first.
- **Auto:** the card is still faster than the CPU at both qualities, so Auto preferring it is right on this machine.

**What these numbers are not:**

- They come from a shot whose drawings repeat, as held drawings do in animation. A shot with more distinct drawings than the memory holds gains less.
- The preview now uses more memory while it plays: up to 3.3 GB for this shot, and never more than the ceiling.

## Limits

- The memory in use shows in Preferences, not in the session log.
- Checked on this machine only.
