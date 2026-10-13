# B-514: the Corner Pins camera

Built on 2026-10-12 as D-634, part of the P0-17 group (D-600, your plan: "how AE treats
them/are done"), approved by the coordinator for your /loop request: a new choice under
**Camera System** in Card Wipe, Card Dance and Shatter, our version of After Effects' Corner
Pins. Instead of turning and placing a camera, you drag the layer's four corners where you want
them, and the effect works out the camera that would see the layer that way, so the cards or
pieces keep moving in proper 3D inside the new shape.

- **Camera System**: Camera Position (as before) or **Corner Pins**. Card Wipe gains this menu;
  in Card Dance and Shatter it sits beside Comp Camera.
- **Upper Left**, **Upper Right**, **Lower Left** and **Lower Right Corner**: where each
  corner of the layer lands, in per cent of the layer (keyable). As added they are the layer's
  own corners, which looks exactly like Camera Position as added.
- **Auto Focal Length**: off uses Focal Length as the lens; on works the lens out from the
  shape of the pins (a strongly tapered shape gives a wide lens). Where the pins show no taper
  at all, Focal Length is used.

With Corner Pins the camera's own turns, place and Z position are not used, and the light stays
where it is on the layer. Adobe gives no formula, so the rule is ours. Projects saved before
this open unchanged. Card Wipe's Comp Camera is still not built and is refused with a warning.

The check, `verification/D-634_corner_pins_table.md` (129 of 129), holds every pixel to numbers
worked out by separate programs before the code existed; Card Wipe on the graphics card gave
the same pixels. The pictures are in `verification/D-634 pictures/`.

## What to check

1. **Card Wipe pulled in.** `1_card_wipe_pulled_in.png`: the flipping cards on a layer whose
   upper corners are pulled in and lower right corner lifted, so it leans back and turns.
2. **The same with Auto Focal Length.** `2_card_wipe_pulled_in_auto_lens.png`: nearly the same
   shape, the lens worked out from the pins.
3. **A four-sided shape and a point light.** `3_card_wipe_four_sided_point_light.png`: the
   layer on an uneven four-sided shape, lit from the upper left; the light stays on the layer.
4. **Card Dance pulled in.** `4_card_dance_pulled_in_auto_lens.png`: the moving cards inside
   the leaning shape, the nearer ones larger.
5. **Card Dance on a four-sided shape.** `5_card_dance_four_sided.png`.
6. **Shatter pulled in.** `6_shatter_pulled_in_auto_lens.png`: the burst seen through the
   leaning shape.
7. **In the app.** On Card Wipe, set Camera System to Corner Pins and drag Upper Right Corner
   inward: the whole wipe should lean with it. Turn Auto Focal Length on and off.
8. **Saved and opened again.** Save, close and open the project: the same pins and picture. An
   older project with Card Wipe should open looking exactly as before.

Our own choice you may want to judge: how strong the perspective looks with Auto Focal Length
on.

## Speed

Not timed separately: Corner Pins only changes the camera, worked out once a frame, so a frame
should cost what it does with Camera Position (see the Card Wipe, Card Dance and Shatter
timing tables).
