# B-137a: Mix, on every effect (D-202)

Written on 2026-09-29, before any code. The last of the After Effects picks, A12, accepted with the rest by your "take everything". It does what After Effects' Effect Opacity does (under "Compositing Options" at the foot of every effect), by a rule of our own. It changes the project file, so this proposal says exactly how.

## What you will see

Every effect's card gets one more row at its foot, **Mix**, 0 to 100 per cent, 100 when the effect is added. It says how much of that one effect's result is kept: at 100 the effect as it is now, at 0 the picture as if the effect were switched off, and between, the two laid over each other. It has a keyframe diamond like any other setting, so an effect can be faded in or out over a few frames: a glow that swells and dies, a blur that comes and goes, an Invert flashed on for a beat.

`effect_mix.png` in this folder is worked by the rule itself: Invert at Mix 100, 50 and 25; a Gaussian Blur at Mix 100 and 50, the sharp drawing laid back over the blurred one; and a Light Wrap at Mix 0, 50 and 100.

## How it differs from what we already have

The layer's **Opacity** fades the whole layer, every effect with it. **Mix** fades one effect and leaves the others, and the drawing, as they are. Some effects already have an amount of their own (Invert's Amount, a Glow's Intensity); Mix is not the same, as it lays the finished result over what went in, where an amount changes the effect's own working. Invert at Amount 50 turns every colour a flat mid grey; at Mix 50 each colour is laid half over its opposite, which is lighter.

## What changes in the project file

An effect in the file gets one more field, `mix`, beside its switch, not among its settings. It is written only when it is not 100 or is keyed. So:

- **Every project saved before this build has no Mix, and opens with every effect at 100**, exactly as it looks now. Saved again, the file is unchanged, byte for byte.
- A Mix outside 0 to 100 in a file (typed by hand) is kept as written, and the effect is left out with the warning the other settings give, as they are today.
- A Mix that is a word, not a number, is a broken file and is refused, as a word in a number's place is today.
- An effect this build does not know keeps its Mix as written.
- Copying and pasting effects, and presets, carry the Mix with the effect.
- The file's version number does not change: nothing needs converting.

## Known limits

- **Posterize Time has no Mix**: it holds the whole layer in time and has no picture to mix. Its card has no Mix row, and a file giving it a Mix other than 100 has the effect left out, with a warning.
- An effect with Mix below 100 is drawn on the processor even where the graphics card would draw it. The card version comes later as its own unit.
- The mix is in light, as every other mix in the program is, so a Mix of 50 looks lighter than half way on dark colours.
- A project saved with a Mix below 100 and opened in an older build of this program shows that effect at full strength; the older build keeps the number and writes it back.
- It is modelled on After Effects' Effect Opacity and is not claimed to match it.

## How you will check it

The build's test draws the thirteen fixture cases and the four wrong ones and compares every pixel with the numbers `tools/effect_mix_reference.py` worked out; it opens old project files and saves them again to show them unchanged, and writes `verification/B-137_effect_mix_table.md`, with pictures. A playtest sheet walks you through fading an effect in and out.
