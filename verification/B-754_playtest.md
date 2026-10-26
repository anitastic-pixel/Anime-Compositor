# B-754 playtest: Float and 32 bpc (After Effects) frames are drawn on the graphics card (D-874)

## What to try

1. Make a composition, open Composition Settings and set the working depth to **Float**.
2. Put a picture or a drawing in it and add **Cartoon**, **Bilateral Blur**, **Find Edges** or **Matte Choker** to it.
3. Play. The viewer no longer says "The CPU drew this frame: its composition works in Float depth", and it plays faster than before (see `B-754_timing.md` for the measured numbers).
4. The picture looks the same as before. Push something past white: put **Exposure** at +3 stops on a layer, set the layer above to **Add** or **Screen**. Then put an adjustment layer on top with Exposure at -3 stops: the bright parts come back with their detail, as they do on the processor.
5. Set the depth to **32 bpc (After Effects)** and repeat. The blurs (Gaussian Blur, Fast Box Blur, Directional, Radial and the others), **Glow**, **Solid Composite** and **Exposure** still run on the processor in this depth (they work in display values there); the rest of the layer's effects run on the card around them.
6. Exports (PNG, MP4, EXR) are drawn by the processor as before: nothing in an exported file changes.

## What the check found

- `tests/b754_float_card.rs` (see `B-754_float_card_table.md`): every fixture project set to Float, and again to 32 bpc (After Effects), drawn by the processor and by the card at its first and middle frames, Full and Draft. Float: 34302 frames on the card, 233 effects; 32 bpc: 34306 frames, 224 effects; about 99 million pixels compared in each. The largest difference at 8 bits is 1 level of 255, no pixel is 2 or more off. In floats the card's own frame is within 1.3e-6 of the processor's on solid areas (the bar is 1e-4) and no edge moved more than 0.0003 px (the bar is 0.05). The 118 frames the card still refuses in each depth (an adjustment layer with an effect the card does not draw there, a blend mode the card does not draw, a Light Wrap with Mix under 100, motion blur with nothing for the card) are the processor's exactly, and the viewer says so. The 34 projects saved in Float or 32 bpc open and draw as saved. 531 of 531 checks pass.
- A scene pushed from -4.6 to 13.4 (Exposure down and up, Normal, Multiply, Screen and Add at three opacities, Fractal Noise mixed by Screen and Add) comes back from the card within 3.6e-7 of the processor's value, everywhere, Full and Draft.

## Pictures (`B-754 pictures/`)

For each of Normal, Multiply, Screen and Add (opacity 100%, Fractal Noise mixed by Add, Full), four pictures, each pixel drawn 24 times as large:

- `<mode>_cpu_as_shown.png` and `<mode>_card_as_shown.png`: the frame as the viewer shows it, from the processor and from the card. Most of it is white, because it is past white.
- `<mode>_cpu_3_stops_down.png` and `<mode>_card_3_stops_down.png`: the same frames' floats three stops darker, so what is past white shows. The card kept the detail the processor kept.

Each card picture is the same file, byte for byte, as its processor picture.
