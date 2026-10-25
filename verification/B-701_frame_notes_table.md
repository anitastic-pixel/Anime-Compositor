# B-701 table: an effect on an adjustment layer that cannot use its masks says so (D-821)

Made by `an_adjustment_layer_warning_says_so_beside_the_frame` in `app/src/tests/serving.rs`. Each of D-786's fixtures is opened and the window is asked for frame 0 as the page asks for it. The note is the code part of `x-frame-notes`, which the page lists under the error details and `tools/app_snapshot.py` reads as `frame_notes`. Before D-821 a missing mask said nothing beside the frame; since D-830 an adjustment layer's own masks are used, so only a mask it does not have warns.

| Check | Expected | Got | Result |
|---|---|---|---|
| Fill naming Fill Mask 2 with only one mask, on an adjustment layer (FX-FILLADJ-012): the note's code beside the frame | EFFECT_PATH_MISSING | EFFECT_PATH_MISSING | pass |
| Fill naming Fill Mask 2 with only one mask, on an adjustment layer (FX-FILLADJ-012): the note's sentence | the sentence the frame said | the sentence the frame said | pass |
| Fill naming Fill Mask 1 on an adjustment layer, which D-830 draws (FX-FILLADJ-007): the note's code beside the frame | no note | no note | pass |
| Fill naming Fill Mask 1 on an adjustment layer, which D-830 draws (FX-FILLADJ-007): the note's sentence | none | none | pass |
| Fill with All Masks on, on an adjustment layer, which D-830 draws (FX-FILLADJ-008): the note's code beside the frame | no note | no note | pass |
| Fill with All Masks on, on an adjustment layer, which D-830 draws (FX-FILLADJ-008): the note's sentence | none | none | pass |
| Fill with Fill Mask 0 on an adjustment layer, which fills the frame (FX-FILLADJ-001): the note's code beside the frame | no note | no note | pass |
| Fill with Fill Mask 0 on an adjustment layer, which fills the frame (FX-FILLADJ-001): the note's sentence | none | none | pass |

8 of 8 pass.

The notes as the window sent them (the sentence, then the code and sentence):

- Fill naming Fill Mask 2 with only one mask, on an adjustment layer (FX-FILLADJ-012): Layer adj's Fill has no mask 2 that is on with two points or more to fill, so it fills nothing. Draw a mask on the layer, set Fill Mask to a mask it has, or set it to 0 to fill the whole layer. / EFFECT_PATH_MISSING: Layer adj's Fill has no mask 2 that is on with two points or more to fill, so it fills nothing. Draw a mask on the layer, set Fill Mask to a mask it has, or set it to 0 to fill the whole layer.
