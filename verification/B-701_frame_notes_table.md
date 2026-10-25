# B-701 table: an effect on an adjustment layer that cannot use its masks says so (D-821)

Made by `an_adjustment_layer_warning_says_so_beside_the_frame` in `app/src/tests/serving.rs`. Each of D-786's fixtures is opened and the window is asked for frame 0 as the page asks for it. The note is the code part of `x-frame-notes`, which the page lists under the error details and `tools/app_snapshot.py` reads as `frame_notes`. Before D-821 the first two said nothing beside the frame.

| Check | Expected | Got | Result |
|---|---|---|---|
| Fill naming Fill Mask 1 on an adjustment layer (FX-FILLADJ-007): the note's code beside the frame | EFFECT_PATH_MISSING | EFFECT_PATH_MISSING | pass |
| Fill naming Fill Mask 1 on an adjustment layer (FX-FILLADJ-007): the note's sentence | the sentence the frame said | the sentence the frame said | pass |
| Fill with All Masks on, on an adjustment layer (FX-FILLADJ-008): the note's code beside the frame | EFFECT_PATH_MISSING | EFFECT_PATH_MISSING | pass |
| Fill with All Masks on, on an adjustment layer (FX-FILLADJ-008): the note's sentence | the sentence the frame said | the sentence the frame said | pass |
| Fill with Fill Mask 0 on an adjustment layer, which fills the frame (FX-FILLADJ-001): the note's code beside the frame | no note | no note | pass |
| Fill with Fill Mask 0 on an adjustment layer, which fills the frame (FX-FILLADJ-001): the note's sentence | none | none | pass |

6 of 6 pass.

The notes as the window sent them (the sentence, then the code and sentence):

- Fill naming Fill Mask 1 on an adjustment layer (FX-FILLADJ-007): Adjustment layer adj's Fill names a mask, which a Fill on an adjustment layer cannot use yet, so it fills nothing. Set Fill Mask to 0 and All Masks off to fill the whole frame beneath, or put the Fill on the layer itself. / EFFECT_PATH_MISSING: Adjustment layer adj's Fill names a mask, which a Fill on an adjustment layer cannot use yet, so it fills nothing. Set Fill Mask to 0 and All Masks off to fill the whole frame beneath, or put the Fill on the layer itself.
- Fill with All Masks on, on an adjustment layer (FX-FILLADJ-008): Adjustment layer adj's Fill names a mask, which a Fill on an adjustment layer cannot use yet, so it fills nothing. Set Fill Mask to 0 and All Masks off to fill the whole frame beneath, or put the Fill on the layer itself. / EFFECT_PATH_MISSING: Adjustment layer adj's Fill names a mask, which a Fill on an adjustment layer cannot use yet, so it fills nothing. Set Fill Mask to 0 and All Masks off to fill the whole frame beneath, or put the Fill on the layer itself.
