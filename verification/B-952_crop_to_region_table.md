# B-952: Crop comp to region of interest (D-1072)

Written by `b952_crop_to_region` in `app/src/tests/w12x.rs`.

After Effects' Composition > Crop Comp to Region of Interest (4GeGLgK2BBk, about 4:40). `cropSteps` is the page's own code run under Node; for the window rows its requests go to the window's own command handler on the reference shot, and the frames are drawn before and after. The picture is `verification/B-952 pictures/crop.png`.

**18 of 18 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a box from (50, 10) to (151, 90): each unparented Position, every key, moved by -50, -10, in one drag, then the size | /property.set_base?layer=a&prop=position&drag=1&value=50%2C40 then /property.set_base?layer=k&prop=position&drag=1&frame=0&value=-40%2C10 then /property.set_base?layer=k&prop=position&drag=1&frame=12&value=-20%2C30%2C5 then /property.set_base?layer=s&prop=position_x&drag=1&value=-45 then /property.set_base?layer=s&prop=position_y&drag=1&frame=3&value=50 then /property.drag_end then /composition.set_settings?width=101&height=80 | /property.set_base?layer=a&prop=position&drag=1&value=50%2C40 then /property.set_base?layer=k&prop=position&drag=1&frame=0&value=-40%2C10 then /property.set_base?layer=k&prop=position&drag=1&frame=12&value=-20%2C30%2C5 then /property.set_base?layer=s&prop=position_x&drag=1&value=-45 then /property.set_base?layer=s&prop=position_y&drag=1&frame=3&value=50 then /property.drag_end then /composition.set_settings?width=101&height=80 | pass |
| its new size, rounded to whole pixels | 101 by 80 | 101 by 80 | pass |
| a box from the top-left corner moves nothing, only the size | /composition.set_settings?width=100&height=50 | /composition.set_settings?width=100&height=50 | pass |
| a box under a pixel still leaves a 1 by 1 composition | /property.set_base?layer=a&prop=position&drag=1&value=0%2C0 then /property.set_base?layer=k&prop=position&drag=1&frame=0&value=-90%2C-30 then /property.set_base?layer=k&prop=position&drag=1&frame=12&value=-70%2C-10%2C5 then /property.set_base?layer=s&prop=position_x&drag=1&value=-95 then /property.set_base?layer=s&prop=position_y&drag=1&frame=3&value=10 then /property.drag_end then /composition.set_settings?width=1&height=1 | /property.set_base?layer=a&prop=position&drag=1&value=0%2C0 then /property.set_base?layer=k&prop=position&drag=1&frame=0&value=-90%2C-30 then /property.set_base?layer=k&prop=position&drag=1&frame=12&value=-70%2C-10%2C5 then /property.set_base?layer=s&prop=position_x&drag=1&value=-95 then /property.set_base?layer=s&prop=position_y&drag=1&frame=3&value=10 then /property.drag_end then /composition.set_settings?width=1&height=1 | pass |
| an unparented locked layer: refused, nothing sent | Unlock Sky first: a locked layer cannot be moved, so nothing is cropped. | Unlock Sky first: a locked layer cannot be moved, so nothing is cropped. | pass |
| a locked layer with a parent is not moved, so it does not stop the crop | 7 | 7 | pass |
| a composition with a camera: refused, nothing sent | This composition has a camera, which a crop does not move, so it is not cropped. | This composition has a camera, which a crop does not move, so it is not cropped. | pass |
| the reference shot: no refusal | null | null | pass |
| the 1920 by 1080 composition cropped to the box from (576, 270): its new size | 960 by 648 | 960 by 648 | pass |
| entries to undo | 2 | 2 | pass |
| frame 0: the most any pixel inside the box differs from the cropped composition's, 0 to 1 | 0 | 0 | pass |
| frame 12: the most any pixel inside the box differs from the cropped composition's, 0 to 1 | 0 | 0 | pass |
| frame 24: the most any pixel inside the box differs from the cropped composition's, 0 to 1 | 0 | 0 | pass |
| crop.png: the reference shot at frame 12 with the box drawn on it, and the cropped composition beside it at the same scale | drawn | drawn | pass |
| two Ctrl+Z: the size and the layers back | true | true | pass |
| in the Composition menu | true | true | pass |
| in the command finder | true | true | pass |
| the box is cleared and the steps sent in order | true | true | pass |
