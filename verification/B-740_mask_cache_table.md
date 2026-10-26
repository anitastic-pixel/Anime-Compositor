# B-740: the viewer keeps each mask's finished coverage (D-860)

Written by `tests/b740_mask_cache.rs`. Every comparison is against the same frame drawn with no cache at all, which is how an export draws it.

## The reference shot: a mask holding still over a drawing that changes

| Frames | Quality | Coverages found kept | Same as no cache |
| --- | --- | ---: | --- |
| 0 to 5 | Draft | 5 | yes |
| 0 to 5 | Full | 5 | yes |

## Each edit, drawn by the warm viewer cache

Frame 4 at Draft (the last row of each kind also at Full). "Fresh": the warm cache's picture of the edited project is the uncached one to the bit. "Shows": it differs from the picture before the edit. "Undone": the project put back draws its old picture again, from what was kept.

| Edit | Quality | Fresh | Shows | Undone |
| --- | --- | --- | --- | --- |
| a point moved | Draft | yes | yes | yes |
| a handle pulled | Draft | yes | yes | yes |
| feather | Draft | yes | yes | yes |
| expansion | Draft | yes | yes | yes |
| mode to Intersect | Draft | yes | yes | yes |
| mode to Difference | Draft | yes | yes | yes |
| inverted | Draft | yes | yes | yes |
| opacity | Draft | yes | yes | yes |
| switched off | Draft | yes | yes | yes |
| a mask added | Draft | yes | yes | yes |
| the order swapped | Draft | yes | yes | yes |
| the layer moved | Draft | yes | yes | yes |
| the layer scaled | Draft | yes | yes | yes |
| the composition's size | Draft | yes | yes | yes |
| a point moved | Full | yes | yes | yes |
| feather | Full | yes | yes | yes |
| mode to Intersect | Full | yes | yes | yes |
| quality switched | Half | yes | - | - |
| quality switched | Draft | yes | - | - |
| quality switched | Full | yes | - | - |
| quality switched | Half | yes | - | - |
| time on a moving mask (frames 3, 4, 3, 6, 7, 4) | Draft | yes | yes | - |

## Every mask fixture, viewer cache against no cache

Frames: the first three of each composition (fewer if shorter), at Full and Draft, drawn twice through one viewer cache per project. "Same" is every pixel's every channel equal to the bit and the same diagnostics.

| Folder | Projects with masks | Frames compared | Coverages found kept | Same |
| --- | ---: | ---: | ---: | --- |
| audio_spectrum | 2 | 24 | 0 | yes |
| audio_waveform | 2 | 24 | 0 | yes |
| display_mix | 2 | 8 | 6 | yes |
| energy_stroke | 12 | 144 | 0 | yes |
| energy_stroke_extras | 15 | 180 | 4 | yes |
| fill | 42 | 504 | 0 | yes |
| fill_adjustment | 3 | 36 | 0 | yes |
| inner_outer_key | 38 | 456 | 0 | yes |
| key_requests | 1 | 12 | 9 | yes |
| liquify_brushes | 3 | 36 | 0 | yes |
| mask_smear | 22 | 264 | 0 | yes |
| masks | 23 | 276 | 0 | yes |
| open_masks | 7 | 84 | 0 | yes |
| particle_mapper | 1 | 12 | 0 | yes |
| particle_mapper_mass | 1 | 12 | 0 | yes |
| particle_mapper_physics | 1 | 12 | 0 | yes |
| particle_playground | 5 | 60 | 0 | yes |
| projects | 1 | 12 | 9 | yes |
| reshape | 22 | 264 | 0 | yes |
| reshape_points | 15 | 180 | 0 | yes |
| scribble | 69 | 828 | 28 | yes |
| stroke | 49 | 588 | 0 | yes |
| stretched_values | 1 | 24 | 0 | yes |
| vegas | 56 | 672 | 0 | yes |

**Result: every check passed**
