# B-107: the third batch on the card, by hand

Built on 2026-09-27 against the B-107 entry in document 15. **Awaiting the owner's playtest.**

`verification/B-107_gpu_fx.md` explains what was built, with the comparison pictures and the timings. This sheet covers what those cannot show: how the viewer feels with these effects on.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture, one effect at a time.** Select a layer. Add one of the new effects and give it settings that do something. Switch **Draw on** between **CPU** and **GPU** a few times: the picture should not change in any way you can see. Remove it before adding the next. At least try Black & White, Halftone, Mosaic, Emboss, Wave Warp, Twirl, Mirror, Radial Wipe, Speed Lines, Cross Glare and Rain.
2. **Hard edges.** Add **Mirror** and turn its Angle to 45, then **Venetian Blinds** at 45 with Feather 0. Zoom in on the edge on **GPU** and on **CPU**. The edge should be the same staircase on both. These two were the ones that disagreed by a few pixels before the fix.
3. **Mosaic on GPU.** Add Mosaic with Size 10 on **GPU**. You should see blocks, never an empty layer.
4. **Dragging.** On **GPU**, drag each effect's main setting slowly: Posterize's Levels, Threshold's Level, Halftone's Size, Wave Warp's Height, Ripple's Amplitude, Twirl's Angle, Bulge's Height, a wipe's Completion, Simple Choker's Choke, Cross Glare's Length. The picture should follow your hand, with no old settings showing.
5. **Playing the moving ones.** Wave Warp and Ripple (with a speed), Speed Lines, Camera Shake and Rain change over time. Press space on each, on GPU. They should move, and look the same on GPU as on CPU when paused on any one frame.
6. **Rain at Draft.** Give Rain a Spacing of 2 and switch to **Draft**. On both CPU and GPU the rain should disappear and the log should say `EFFECT_PARAMETER_INVALID`. Back at **Full resolution** it comes back on both.
7. **Motion Tile stays on the CPU.** Motion Tile works on GPU as before, but the CPU draws that layer. Nothing should look different.
8. **Play at Full.** Switch to **Full resolution**, put Cross Glare (or Ripple, or Rain) on three layers, and press space. On GPU or Auto it should play clearly more smoothly than on CPU: the timing table measured Cross Glare at 25 ms a frame on the card against 121 on the CPU.
9. **Draft.** Switch to **Draft**. Playing should feel about the same on CPU and GPU.
10. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the effect, its settings and the frame number.
- Any hard edge (Mirror, a wipe, Threshold, Posterize, Halftone) that differs by even one pixel between CPU and GPU.
- Any moment where the picture lags behind a change you made, or shows old settings.
