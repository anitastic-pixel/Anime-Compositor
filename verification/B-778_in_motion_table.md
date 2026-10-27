# B-778: In motion (D-898)

Written by `b778_in_motion_is_labelled_never_kept_and_replaced_by_the_chosen_picture` in `app/src/tests/in_motion.rs`.

While the hand drags or scrubs, the page asks for each frame In motion, whatever quality is chosen: an eighth of the composition each way, masks drawn at that size, at most four moments of any shutter. Each row checks that it is labelled, never kept, and that the picture asked for once the hand stops is byte for byte the one the chosen quality makes on a second viewer given the same changes and never moved. On the reference shot.

**25 of 25 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| frame 30 In motion: labelled, not final, an eighth each way of 1920 by 1080 | In motion true 240 135 | In motion true 240 135 | pass |
| asked again, it is made again, not sent from memory | 0 | 0 | pass |
| the frame memory holds no more than before it | 8294400 bytes | 8294400 bytes | pass |
| the quality chosen is still Full | Full | Full | pass |
| Full afterwards is the Full frame remembered from before | 1 | 1 | pass |
| and is Full, the same as export, byte for byte frame 30 at Full asked for alone | Full false identical | Full false identical | pass |
| which is the Full frame from before | identical | identical | pass |
| a feathered mask added to layer 2: In motion, then Full once still, is that Full asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| the mask dragged narrower: In motion, then Full once still, is that Full asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| undone: In motion, then Full once still, is that Full asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| a frame change: In motion, then Full once still, is that Full asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| a quality change (Half): In motion, then Half once still, is that Half asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| a quality change (Draft): In motion, then Draft once still, is that Draft asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| a switch of composition: In motion, then Full once still, is that Full asked for alone; the In motion frame is the one asked for alone | In motion identical identical | In motion identical identical | pass |
| card: frame 30 asked for in motion, with the mask, is drawn at Draft there (In motion measured slower on the card), labelled so, 480 by 270 | Draft true 480 270 | Draft true 480 270 | pass |
| card: Full afterwards is byte for byte the card's Full asked for alone | identical | identical | pass |
| card: the frame asked for in motion is the card's Draft asked for alone | identical | identical | pass |
| card: the quality chosen is still Full | Full | Full | pass |
| cache: the drawing taken down an eighth is counted in the memory budget | 518400 bytes more | 518400 bytes more | pass |
| cache: asked again, it is the one held, not made again | true false | true false | pass |
| cache: a request that is not in motion still gets the whole drawing, never the eighth | true 1920 1080 | true 1920 1080 | pass |
| cache: another size is its own, never the eighth | true | true | pass |
| with motion blur switched off its 16 samples cost nothing, so nothing is copied | none | none | pass |
| the reference shot's 16 motion blur samples are cut to 4 in motion | 16 4 | 16 4 | pass |
| a shot with 4 or fewer is drawn as it is, uncopied | none | none | pass |

## What this does not cover

How quick it is and how it feels: the delay is measured by the lag harness (D-898's evidence) and judged in `verification/B-778_playtest.md`.
