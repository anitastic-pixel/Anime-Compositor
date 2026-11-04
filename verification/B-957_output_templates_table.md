# B-957: Output templates (D-1077)

Written by `b957_output_templates` in `app/src/tests/w12x.rs`.

After Effects' Output Module > Make Template (nTpT2PELCbA, about 0:40). `keepTemplate` is the page's own code run under Node; the other rows read the page.

**8 of 8 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Make template "test" on EXR full float with the WAV ticked | test exr-float with WAV | test exr-float with WAV | pass |
| then "film" on MP4 High: both kept | test exr-float; film mp4 | test exr-float; film mp4 | pass |
| "test" made again on PNG: replaced, not doubled | film mp4; test png | film mp4; test png | pass |
| what a template keeps (never the missing-drawing tick) | exportformat, filmquality, gifdither, soundwav, cardexport | exportformat, filmquality, gifdither, soundwav, cardexport | pass |
| the Template list and Make template stand with the export choices | true | true | pass |
| kept with the window's preferences | true | true | pass |
| choosing one sets each choice as if picked by hand | true | true | pass |
| Export and Add to queue read the same choices | true | true | pass |
