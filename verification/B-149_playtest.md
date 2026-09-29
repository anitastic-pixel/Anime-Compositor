# B-149: Block Dissolve, by hand

Built on 2026-09-29 against D-214, which you accepted with the rest of the After Effects picks
("take everything"), as B12. It is After Effects' **Block Dissolve**: a transition that makes a
layer vanish in random blocks, or, keyed the other way, appear in them. The blocks go in a
scattered order, and a block once gone stays gone as the transition goes on, so it never
flickers. It sits in the **Transition** group after Iris Wipe.

The generated halves are `verification/B-149_block_dissolve_table.md`, 103 of 103 checks
passing, which renders every FX-BDISSOLVE case against the numbers written before the code and
draws the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card
sends every setting the command reads. This sheet covers what the tables cannot: how it looks in
the window.

## The pictures

`verification/B-149 pictures/`, three times enlarged; where nothing is, the picture is clear:

- `card.png`: a made-up title card at its own size, 160 by 100: a navy panel with a pink band
  and a yellow disc, on nothing.
- `card_start.png`: Block Dissolve as it starts: the card as it was.
- `card_quarter.png`: Transition Completion 25, blocks 10 by 10: about a quarter of the card
  gone, in square blocks.
- `card_half.png`: 50: about half gone, every block gone at 25 still gone.
- `card_three_quarters.png`: 75: a few blocks left.
- `card_bricks.png`: 50 in blocks 40 wide by 5 tall: long flat bricks.
- `card_pixels.png`: 50 in blocks 1 by 1: single pixels, a fine speckle.
- `card_feather.png`: 50 in blocks 10 by 10 with Feather 8: the same kind of blocks with soft
  edges.
- `card_gone.png`: 100: nothing left.

## Before you start

Make the composition 160 by 100 and import `card.png` from `verification/B-149 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** Pick **Block Dissolve** in **Add effect...**, under **Transition**, after Iris
   Wipe; typing "dissolve", "blocks" or "random" in the search finds it. The card shows
   Transition Completion 0, Block Width 1, Block Height 1 and Feather 0, and the card looks as it
   was.
2. **Blocks.** Block Width 10 and Block Height 10, then Transition Completion 25, 50 and 75: as
   `card_quarter.png`, `card_half.png` and `card_three_quarters.png`. Each block goes whole, and
   a block gone at 25 is still gone at 50 and 75.
3. **Bricks.** Block Width 40 and Block Height 5 at 50: as `card_bricks.png`.
4. **Pixels.** Block Width 1 and Block Height 1 at 50: as `card_pixels.png`.
5. **Feather.** Blocks 10 by 10 at 50 and Feather 8: as `card_feather.png`, the blocks' edges
   soft and their colour unchanged.
6. **All gone.** Transition Completion 100: nothing left, as `card_gone.png`.
7. **Keys.** Key Transition Completion from 0 at the first frame to 100 at frame 24 and play:
   the card breaks up block by block and is gone at frame 24; no block comes back. Key it from 100
   to 0 instead: the card appears block by block.
8. **Moved.** Move and scale the layer: the blocks move and scale with it, the same blocks gone.
9. **Out of range.** Type 101 in Transition Completion, 0.5 in Block Width or -1 in Feather: it is
   refused with a sentence saying what it runs to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller and looks the same, the blocks the same
    ones.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- After Effects' Soft Edges switch is left out: Feather is the one way to soften the blocks.
- There is no random seed: two Block Dissolves with the same block sizes go in the same order.
- It runs on the processor; a graphics card version is its own later unit.
- It is modelled on After Effects' Block Dissolve and is not claimed to match it; its blocks go
  in a different order.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 50 to 55 ms without it; as it starts, 50 to 52 ms; half gone in blocks 16 by 16, 59 to
  61 ms; half gone in blocks 1 by 1, 61 to 64 ms; blocks 16 by 16 with Feather 8, 59 to 64 ms;
  with Feather 100, 60 to 64 ms. **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build;
  timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
