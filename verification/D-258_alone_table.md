# D-258: one layer alone, for the onion skin

Written by `d258_one_layer_alone_is_that_layer_soloed` in `app/src/main.rs`. `alone` is the answer to the page's `/alone`; the soloed frames are the window's own frame answers with the layer really soloed.

**6 of 6 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| asking for layer2 alone solos nothing in the window | no layer soloed, no undo step | no layer soloed, no undo step | PASS |
| the ordinary frame 10 afterwards is the ordinary frame 10 before | 1920 by 1080, every byte the same | 1920 by 1080, every byte the same | PASS |
| layer2 alone at frame 10, Full, is the frame sent with layer2 soloed | 1920 by 1080, every byte the same | 1920 by 1080, every byte the same | PASS |
| and at Draft too | 480 by 270, every byte the same | 480 by 270, every byte the same | PASS |
| and it is not the whole picture | different from the ordinary frame | different from the ordinary frame | PASS |
| a layer that is not in the composition is refused | refused | refused | PASS |
