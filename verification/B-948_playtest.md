# B-948 playtest: Text Animator, the Wiggly Selector (D-1068)

The tutorial (SbTfu, 1:40) flickers a neon sign with an Opacity animator whose only selector is a
Wiggly Selector: Mode Add, Max Amount 100, Min Amount -100, Wiggles/Second 3, Correlation 50,
Random Seed 0.

## Steps

1. Make a text layer that says `Neon Sign` and add Text Animator to it. Set Opacity to 25.
2. Set End to 0. This makes the Range Selector pick nothing, as if it were deleted.
3. Set Wiggly Selector to **On** and Wiggly Mode to **Add**. Set Wiggles/Second to 3. Leave
   Max 100, Min -100, Correlation 50 and Random Seed 0.
4. Play. Each letter should dim and brighten on its own, smoothly, about three times a second.
5. Set Correlation to 100. Now every letter dims and brightens together.
6. Set Wiggles/Second to 0. The letters stop changing.
7. Set Random Seed to 1. The letters land on other brightnesses.
8. Set Wiggly Selector to **Off**. The text is fully bright again (the range picks nothing).
9. Save, close and open the project with Wiggly on. The flicker is the same as before saving.

## What you should see

`verification/B-948 pictures/neon_frames.png` shows frames 0, 6, 12, 18 and 24 from the top. Some
letters are a little dimmer in each row, and which ones changes from row to row.

Wrong if: in step 4 the letters jump instead of drifting; in step 5 they still differ; in step 6
they still move; in step 8 anything stays dim; or a title saved before this change looks
different.

Not the same as After Effects: our noise is our own, so the exact pattern differs from the
tutorial's; the selector has no Based On or Lock Dimensions of its own; and an animator holds one
Range Selector and at most one Wiggly Selector.
