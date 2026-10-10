# B-333: Dust & Scratches

Built on 2026-10-10 as D-453, under your /loop request: After Effects' Dust & Scratches, in
**Blur** (beside Median). It is Median (D-203) that leaves alone whatever is close to its
neighbours, so specks and dust go and fine grain stays.

- **Radius** (1): how far round each pixel it looks, in pixels, 0 to 10. Below 1 it does nothing.
- **Threshold** (0): how far, in steps of 0 to 255, a colour may be from those round it and still
  stay. At 0 every difference goes (just as Median); at 255 nothing changes. Each of red, green
  and blue is judged on its own, so a speck can keep one and lose the others.
- **Operate on Alpha** (Off): also fill small holes and nicks in the layer's covering.

After Effects publishes no formula, ranges or starting values for this effect, so how each pixel
is worked out is our own rule, written down in document 21. Every one of After Effects' settings
is offered.

The check, `verification/D-453_dust_scratches_table.md` (144 of 144), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-453 pictures/`
(`1_before.png` is the street with nothing on it).

## What to check

1. **Radius.** `2_radius_2.png` and `3_radius_3.png`: small details (windows, thin edges)
   evened into what surrounds them, more at 3; the outline of the picture unchanged.
2. **Threshold.** `4_radius_3_threshold_40.png` against `3_radius_3.png`: only what stands out
   strongly is evened; softer detail is kept (1330 pixels changed against 3793).
3. **Threshold 255.** `5_threshold_255.png` is the same as `1_before.png`.
4. **In the app.** Put Dust & Scratches (Blur) on a picture with fine grain or noise and a few
   specks (add Noise first if you have none). Raise Radius to 2: specks and grain both smooth
   away. Raise Threshold slowly: the grain comes back while the strongest specks stay gone. Cut
   a small hole in the layer with a mask or use a picture with pinholes, turn Operate on Alpha
   on: the small holes fill.
5. **Out of range.** Type 11 in Radius or 256 in Threshold: it is refused with a sentence saying
   the range.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (After Effects says nothing on them): Radius runs 0 to 10
as Median's, starting at 1; Threshold starts at 0; each colour channel is judged on its own.

## Speed

Measured only provisionally: the card was fully used by other lanes' tests the whole time (lane A's Curl Noise test alone held 4.8 GB of it, the card at 96 to 100 per cent before and after, 15 GB of its 16.8 in use), and earlier waits found no quiet minute
(`verification/B-333_dust_scratches_timing_table.md`). Three Dust & Scratches on a 1920 by 1080 shot, card / processor,
milliseconds a frame played again: Noise alone 201.1 / 49.9 ms a frame, as added (Radius 1, Threshold 0) 656.9 / 92.9, Radius 3, Threshold 20 604.6 / 144.4, Radius 5, Threshold 8, Operate on Alpha on 975.1 / 241.1; no verdict against Target P2 (4 ms or less a layer): the card's times measure the other tests sharing it, not the effect (Noise alone 201.1 ms on the card against 12.9 in B-331's quiet run); on the processor three layers add 43.0 to 191.2 ms a frame; to be measured again.
