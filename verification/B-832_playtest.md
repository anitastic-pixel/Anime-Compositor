# B-832 playtest: the viewer's resolution list (D-952)

What changed: the quality menu under the viewer now lists After Effects' resolutions, **Full, Half, Third, Quarter and Auto**, with the one in use lit. Third is new: the picture drawn at a third of the size each way, quicker than Half and sharper than Quarter.

## Steps

1. Open the reference shot. Click the quality button under the viewer. The menu lists Full, Half, Third, Quarter and Auto above the Full/Draft line.
2. Choose **Third**. The picture goes a little soft, the corner says it is not final, and Third is lit when you open the menu again.
3. Zoom in and out (the mouse wheel over the picture). At every zoom it stays at Third.
4. Choose **Auto**. At 100% zoom the picture is sharp. Zoom out to about 30%: it is drawn at Third; at about 45%, Half; at 20%, Quarter. On screen it should look as sharp as before at each zoom.
5. Choose **Full**. It is sharp at every zoom, and the Full pixels tick in the menu is ticked. Untick it: Auto is lit.
6. Export a short piece (or one frame) while Third is chosen. The export is full size and sharp: the list never changes what is exported.

## What you should not see

- A picture that is a third of the size on screen rather than a third of the detail.
- An export that looks soft.

The computer's checks are in `verification/B-832_resolution_list_table.md`; the check in a private copy of the app is `verification/B-832_private_app_table.md`.
