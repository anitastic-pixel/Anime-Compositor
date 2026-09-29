# B-147: Extract, by hand

Built on 2026-09-29 against D-212, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Extract does, by a rule of our own: it takes out
of a layer every part too dark or too bright, measured by one channel, as the white paper is
taken from behind a scanned pencil drawing, or the black from behind a flash filmed on black.
The layer does not grow. It sits in the **Lines & Mattes** group after HSV Key.

The generated halves are `verification/B-147_extract_table.md`, 121 of 121 checks passing,
which renders every FX-EXTRACT case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks in the
window.

**One question for you first, D-215.** Fixture FX-EXTRACT-027 was written expecting a file
whose Black Point is written as a word ("60") to open with a warning; you already decided in
D-164 that such a file is refused whole. The table shows that one case as in dispute. D-215
proposes changing the fixture to expect the refusal, as with D-164. "D-215 accepted" is enough.

## The pictures

`verification/B-147 pictures/`, three times enlarged, a grey and white check where nothing is:

- `drawing.png`: the drawing at its own size, 160 by 100: a made-up scan, a face in black line
  on cream paper, a red scarf, a patch of blue sky and a yellow star; `before.png` is the same
  enlarged.
- `paper_gone.png`: White Point 220: the cream paper is gone, the face, scarf, sky and star
  left on the check.
- `paper_soft.png`: the same with White Softness 40: the paper gone, the pale skin all but gone
  too and the yellow star a third left, as they lie inside the fade.
- `line_gone.png`: Black Point 40: the black line is gone, everything else kept.
- `line_only.png`: the same inverted: only the black line is left.
- `middle.png`: Black Point 60 and White Point 200: the line and the paper both gone.
- `red_channel.png`: Channel Red and White Point 100: every part with much red goes, the paper,
  skin, scarf and star, and the line and blue sky stay.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-147 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Extract** in **Add effect...**, under **Lines &
   Mattes**, after HSV Key; typing "extract", "luma" or "paper" in the search finds it too. The
   card shows Channel Luminance, Black Point 0, White Point 255, Black Softness 0, White Softness
   0 and Invert Off, and the picture is unchanged.
2. **White point.** White Point 220: as `paper_gone.png`.
3. **Softness.** White Softness 40: as `paper_soft.png`. Back to 0 and White Point 255.
4. **Black point.** Black Point 40: as `line_gone.png`.
5. **Invert.** Invert On: as `line_only.png`. Back to Off.
6. **The middle.** Black Point 60 and White Point 200: as `middle.png`. Back to 0 and 255.
7. **Channel.** Channel Red and White Point 100: as `red_channel.png`. Try Green, Blue and Alpha
   too: each takes out by its own brightness. Back to Luminance and 255.
8. **Keys.** Key White Point from 255 at the first frame to 0 at frame 24 and play: the drawing
   is taken away from its brightest parts down to its darkest.
9. **Moved.** Move the layer: the keyed drawing moves with it, the same.
10. **Out of range.** Type 256 or -1 in any of the four numbers: it is refused with a sentence
    saying what it runs to, and the card keeps its old number.
11. **Draft.** Press **Draft**: the picture is smaller and looks the same.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- After Effects shows a histogram of the layer on the card, to set the points by; this card has
  the numbers only.
- It runs on the processor; a graphics card version is its own later unit.
- It is modelled on After Effects' Extract and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 50 to 52 ms without it; as it starts, 54 to 57 ms; White Point 220, 53 to 60 ms; with
  White Softness 40, 55 to 58 ms; Channel Red, 53 to 54 ms. **Machine:** AMD Ryzen 9 9900X
  with 24 threads; Windows 11; release build; timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did; and "D-215 accepted" or not.
