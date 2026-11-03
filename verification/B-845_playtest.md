# B-845 playtest: Remove Grain is added with three passes (D-965)

What changed: a new Remove Grain now starts with Passes 3, as After Effects adds it (the
playlist's Remove Grain tutorial shows 3 at 2:00). Before, it started with 1. Projects you
already saved keep the Passes they had.

## Steps

1. Open the program and a composition with a grainy picture or video on a layer.
2. Select the layer and add Effect > Noise & Grain > Remove Grain.
3. Look at the Effect Controls.
4. Open a project saved before this change that has a Remove Grain on Passes 1.

## What to see

- Step 3: Passes reads 3 (Noise Reduction 1, Mode Multichannel, Unsharp Mask 0 / 1 / 0, as
  before).
- Step 3: the grain is smoothed a little more than a one-pass Remove Grain would; flat areas
  look calmer, hard edges stay hard.
- Step 4: that older Remove Grain still reads Passes 1 and looks exactly as it did.

If any of these is not so, B-845 is wrong.
