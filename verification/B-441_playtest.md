# B-441: CC Glass, its full control set

Built on 2026-10-11 as D-561, under your /loop request. CC Glass (Stylize) bends the picture as if
seen through thick glass shaped by a bump map, and lights the glass. Until now it had a reduced
light: one direction and a strength. It now has the rest of CC Glass's light: Light Type (a
distant light or a point light), Light Height, Light Position, Ambient, Diffuse, Specular,
Roughness and Metal. Older projects open with the old light (shown as "Bevel (older projects)"
in Light Type) and draw exactly as before.

The check, `verification/D-561_glass_table.md` (175 of 175), holds every pixel
to numbers a separate program worked out before the code was written, and holds the graphics
card's picture to the processor's (48 of 48 within 1 level). The pictures are in
`verification/D-561 pictures/`, made from the town street.

## What to check

1. **Before.** `1_before.png`: the street.
2. **As added.** `2_as_added.png`: the picture's edge bent like the rim of thick glass, lit from
   the top left by a white light; the top and left rims lighter, the bottom and right darker.
3. **The old light.** `3_classic_light.png`: the same bend with the old light: a harder light and
   shade on the rims, the middle exactly as the street.
4. **Metal and plastic.** `4_luminance_metal.png` and `5_luminance_orange_light.png`: the street's
   light and dark parts as glass ripples under an orange light. With Metal 100 (4) the shine
   takes the street's own colours; with Metal 0 (5) the shine is the light's orange.
5. **A point light.** `6_point_light.png`: a light hanging over the top left: brightest there,
   fading toward the bottom right.
6. **Flat.** `7_flat.png`: Displacement 0, Ambient 100, Diffuse 0, Specular 0: the street
   unchanged.
7. **In the app.** Add CC Glass to a drawing. Set Light Type to Point Light and drag Light
   Position: the bright spot follows. Raise Specular and lower Roughness: a small sharp shine.
   Open an older project that uses CC Glass: it looks as it did.

Not settled: CC Glass's real starting values could not be read (Adobe's page refused the
request), so the values it starts with here are this program's choice. After Effects' own light
layers are not offered; the effect's own light is.

## Speed

`verification/B-441_glass_timing_table.md`: provisional (another lane's GPU test was running throughout): on the card the reference shot with a moving Noise took 40.7 ms a frame, 47.4 with CC Glass as added and 48.7 with a point light on Luminance; on the processor 109.8, 174.0 and 158.3.
