# B-53: the zoom's direction, by hand

Built on 2026-09-26 against the B-53 entry in document 15. **Playtest passed on 2026-09-26:** the owner wrote "works fantastically". `verification/B-53_zoom.md` explains what changed, with before-and-after pictures. This sheet checks it in the app.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer, the picture that fills the frame.

## What to check

1. **Direction.**
   - Add a **Radial Blur**. Set **Type** to **Zoom** and **Amount** to about 30.
   - Everything should streak away from the centre, as when a camera zooms in, and nothing should streak toward it.
2. **Centre.** Drag the centre to one side. The streaks should now run away from the new centre.
3. **Amount.**
   - Raise **Amount** to 100. The streaks should grow longer.
   - Set it to 0. The picture should be sharp again.
4. **Edges.** With the centre inside the frame, the frame's edges should stay solid with either **Edges** setting.
5. **Spin.** Set **Type** to **Spin**. It should look as it did before.
6. **CPU and GPU.** Switch **Draw on** between **CPU** and **GPU**. The picture should not change in any way you can see.
7. **Compared with After Effects.**
   - If you have After Effects, set the same zoom there. The streaks should run the same way.
   - The strength for a given Amount may differ. Adobe does not publish its formula.

## What to report

- Any streak that runs toward the centre.
- Any difference between CPU and GPU.
- Whether the amount of smear for a given Amount feels far from After Effects, and roughly by how much.
