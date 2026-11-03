# B-817 page check: Add Grain's Viewing Mode

A private copy of the release app (ANIME_COMPOSITOR_BACKGROUND=1, its own WebView2 profile) opened a 560 by 332 composition, one picture with an Add Grain written before B-817 (no viewing mode); each row is what the page answered. Screens, cropped to Effect Controls: `verification/B-817 pictures/page_final_output.png` and `page_preview.png`. The hidden test window's viewer does not draw, so the frames themselves are on the sheet `grain_view_vs_tutorial.png`.

| Check | The page's answer | Matches |
| --- | --- | --- |
| Effect Controls shows Add Grain's Viewing Mode, Final Output and Preview | Add Grain Viewing Mode: Final Output, Preview / showing final_output | yes |
| the file, written before, has no viewing mode | (not written) | yes |
| Viewing Mode is the first of Add Grain's switches (the panel lists the numbers first, then the switches) | Viewing Mode | yes |
| picking Preview sets the effect's viewing mode to preview | preview | yes |
| Effect Controls, drawn again, still shows Preview | preview | yes |

5 of 5 checks pass.
