# B-44b: sending drawings to the card, smaller and once

Built on 2026-09-26 at the owner's word ("proceed with B-44b first; I want to setup this app to utilize everything my pc has to offer"). It is B-44 made faster. **It changes no export, no fixture and no CPU picture.** The switch still starts on CPU.

## What changed

- **A drawing goes to the card at half the size.** Before, each number went in 32 bits, the size the CPU works in. Now each goes in 16 bits. The conversion is done by every processor thread at once, straight into the memory the card copies from.
- **The card keeps a drawing after the CPU lets go of it.** B-44 forgot a drawing as soon as the CPU's own store of drawings dropped it. That store holds about 17 drawings, so on a long shot the card re-sent almost everything. Now the card remembers each drawing by the same name the CPU's store uses: the file, its size and date, and how it is read. If the CPU reads the same file again, the card already has it and nothing is sent.
- **The card may use half of its own memory.** B-44 allowed 1 GiB. Windows now reports how much memory the card has, and the switch uses half of it: 8.4 GB on this machine. The card's name in the tables now ends with its memory, for example "16.8 GB of its own memory". If Windows cannot say, the limit stays 1 GiB.
- **A fault in B-44, found and fixed.** Once the card's memory was full, a frame that brought two new drawings could draw one layer with the other layer's drawing. The first new drawing's place in the card's list was noted. Then, to make room for the second, an old drawing was thrown out, and the list was re-shuffled. The noted place then pointed at the wrong drawing. The owner has not yet played B-44, and the fault needed a long shot at Full to appear. Each layer now holds its drawing directly, so throwing another out cannot move it. A new check (below) runs the shot with the card's memory full.

## How to judge it

`B-44_gpu_preview_table.md`: **107 of 107 comparisons pass**, each within 1 level of 255 of the CPU's picture (D-100). It has three new rows since B-44:

- **the ten-layer fixture, frame 100, read again into a new CPU store after the first one let go.** It must send no drawing again. It sent **0**.
- **the reference shot, every fifth frame at Full, with the card allowed three drawings.** Here the card's memory is full, so a frame bringing two new drawings must throw one out between them. This is the case B-44 got wrong.
- The budget-of-one row from B-44 is still there.

**What 16-bit costs, honestly.** More pixels now differ from the CPU's picture by 1 level, and none by more:

- On P-07's hard case, 11,007 pixels differ, where B-44 had 396.
- The worst comparison is now the ten-layer fixture at Draft with every layer in Multiply: 14,780 pixels, each 1 level.

The pictures are in `B-44 pictures/`. A 1-level difference in 255 cannot be seen. It is the difference D-100 allows.

## How fast it is now

Measured, not assumed: `B-44b_gpu_timing_table.md`, written by `cargo test --release --test b44_gpu_preview -- --ignored`. B-44's own table, `B-44_gpu_timing_table.md`, is kept as the before.

- Machine: AMD Ryzen 9 9900X, 24 threads, Windows 11 Education 10.0.26200
- Card: NVIDIA GeForce RTX 4070 Ti SUPER, 16.8 GB, driver 610.74, through Vulkan
- Build: rustc 1.89.0, release profile

Medians in ms per frame, for the layering alone:

| Shot | Quality | CPU | GPU in B-44, again | GPU in B-44b, first pass | GPU in B-44b, again | Drawings sent, again |
|---|---|---|---|---|---|---|
| the reference shot | Draft | 2.43 | 11.45 | 0.88 | 1.02 | 0 |
| the reference shot | Full | 15.31 | 13.19 | 5.69 | 5.17 | 0 |
| the ten-layer fixture | Draft | 4.03 | 16.76 | 3.60 | 2.57 | 0 |
| the ten-layer fixture | Full | 27.91 | 35.83 | 9.07 | 9.46 | 0 |

- **The card is now faster than the CPU in every row.**
  - At Full it is about 3 times faster: 15.3 ms against 5.2 on the reference shot, and 27.9 against 9.5 on the ten-layer fixture.
  - At Draft it goes from 3 to 4 times slower to faster.
- **The second pass sends nothing.** Every drawing the shot uses stays on the card.
- **Correcting B-44's write-up.** It said a pass met "about 430 different drawings". The reference shot has 56, and the ten-layer fixture 166. The 430 were the same drawings sent again and again, because the card forgot them.
- **Why the first pass is also fast.** The first pass is the same speed as the second because the median frame sends nothing: a drawing is sent once, then held for the frames it is exposed on. That first send is roughly half the size it was.

These are still the layering alone. A whole frame on screen also includes reading drawings from disk, running effects on the CPU, and handing the picture to the window. The session log (P-19) shows each part.

## What was not done, and why

- **Draft does not shrink drawings before sending them.** The CPU's Draft picture is made from full-size drawings. A card working from shrunk ones would draw a different picture, more than 1 level off. Drawings kept on the card at full size serve both Draft and Full, so switching quality sends nothing.
- **Eight-bit drawings are not sent.** The program does not keep a drawing's original eight-bit numbers once it has turned them into working colour. Keeping them would mean a second copy of every drawing in memory. Sixteen-bit works for every drawing, including EXR files and effect results.

## Limits

- **Measured on one machine and one card.**
- **The card keeps drawings only while the window is open.** Switching back to CPU does not empty it, so switching to GPU again is instant. The memory it holds, up to half the card's, is let go when the app closes. A game or another program run beside it at the same time has the other half.
- **Effects still run on the CPU.** They are the next units: Radial Blur first.
