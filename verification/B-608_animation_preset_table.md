# B-608: animation presets

Written by `tests/b608_animation_preset.rs` from `Fixtures/animation_preset/`, against D-739. Each line is one check: what `tools/animation_preset_reference.py` says the layer's saved record must hold once the preset is applied, and what the build's holds. Keys are [frame, value], scale in per cent.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-ANP-001 | steps to undo | 1 | 1 | yes |
| FX-ANP-001 | transform/opacity: keys | [[10,0],[22,1],[40,0.5]] | [[10,0],[22,1],[40,0.5]] | yes |
| FX-ANP-002 | steps to undo | 1 | 1 | yes |
| FX-ANP-002 | transform/scale: keys | [[10,[0,0]],[18,[110,110]],[22,[100,100]]] | [[10,[0,0]],[18,[110.0,110.0]],[22,[100,100]]] | yes |
| FX-ANP-003 | steps to undo | 1 | 1 | yes |
| FX-ANP-003 | transform/scale: keys | [[10,[0,0]],[18,[110,110]],[22,[100,100]]] | [[10,[0,0]],[18,[110.0,110.0]],[22,[100,100]]] | yes |
| FX-ANP-004 | steps to undo | 1 | 1 | yes |
| FX-ANP-004 | transform/position: keys | [[10,[-1620,200]],[22,[300,200]]] | [[10,[-1620,200]],[22,[300,200]]] | yes |
| FX-ANP-005 | steps to undo | 1 | 1 | yes |
| FX-ANP-005 | transform/position: keys | [[10,[-960,540]],[22,[960,540]]] | [[10,[-960,540]],[22,[960,540]]] | yes |
| FX-ANP-006 | steps to undo | 1 | 1 | yes |
| FX-ANP-006 | transform/position: expression | "wiggle(12, 15)" | "wiggle(12, 15)" | yes |
| FX-ANP-007 | steps to undo | 1 | 1 | yes |
| FX-ANP-007 | effects: length | 1 | 1 | yes |
| FX-ANP-007 | effects/-1/parameters/completion: keys | [[30,100],[42,0]] | [[30,100],[42,0]] | yes |
| FX-ANP-008 | steps to undo | 1 | 1 | yes |
| FX-ANP-008 | transform/opacity: keys | [[22,0],[34,1],[40,0.5]] | [[22,0],[34,1],[40,0.5]] | yes |
| FX-ANP-008 | effects: length | 1 | 1 | yes |
| FX-ANP-008 | effects/-1/parameters/completion: keys | [[20,100],[26,0]] | [[20,100],[26,0]] | yes |
| FX-ANP-009 | steps to undo | 1 | 1 | yes |
| FX-ANP-009 | shapes/0/trim/start: base | 0 | 0 | yes |
| FX-ANP-009 | shapes/0/trim/start: keys | [] | [] | yes |
| FX-ANP-009 | shapes/0/trim/end: keys | [[10,0],[34,100]] | [[10,0],[34,100]] | yes |
| FX-ANP-009 | shapes/0/trim/offset: base | 0 | 0 | yes |
| FX-ANP-009 | shapes/0/trim/offset: keys | [] | [] | yes |
| FX-ANP-009 | shapes/1/trim/start: base | 0 | 0 | yes |
| FX-ANP-009 | shapes/1/trim/start: keys | [] | [] | yes |
| FX-ANP-009 | shapes/1/trim/end: keys | [[10,0],[34,100]] | [[10,0],[34,100]] | yes |
| FX-ANP-009 | shapes/1/trim/offset: base | 0 | 0 | yes |
| FX-ANP-009 | shapes/1/trim/offset: keys | [] | [] | yes |
| FX-ANP-010 | applied | refused, naming shape layer; nothing sent | refused: "Write-on" trims the lines of a shape layer, and big is not a shape layer. Nothing was changed. | yes |
| FX-ANP-011 | steps to undo | 1 | 1 | yes |
| FX-ANP-011 | masks: length | 1 | 1 | yes |
| FX-ANP-011 | masks/0/path: frames | [10,20] | [10,20] | yes |
| FX-ANP-011 | masks/0/feather_px: keys | [[10,18],[20,149]] | [[10,18],[20,149]] | yes |
| FX-ANP-011 | masks/0/expansion_px: keys | [[10,0],[20,62]] | [[10,0],[20,62]] | yes |
| FX-ANP-011 | masks/0/tension: equals | 2 | 2 | yes |
| FX-ANP-012 | steps to undo | 1 | 1 | yes |
| FX-ANP-012 | effects: length | 1 | 1 | yes |
| FX-ANP-012 | effects/-1/parameters/start: keys | [[10,0],[34,100]] | [[10,0],[34,100]] | yes |
| FX-ANP-013 | steps to undo | 1 | 1 | yes |
| FX-ANP-013 | transform/rotation: keys | [[10,0],[16,90],[22,0]] | [[10,0],[16,90],[22,0]] | yes |
| FX-ANP-013 | transform/rotation: kinds | ["bezier","continuous","bezier"] | ["bezier","continuous","bezier"] | yes |
| FX-ANP-014 | steps to undo | 1 | 1 | yes |
| FX-ANP-014 | transform/rotation: base | 45 | 45 | yes |
| FX-ANP-014 | transform/rotation: keys | [] | [] | yes |
| FX-ANP-015 | steps to undo | 1 | 1 | yes |
| FX-ANP-015 | transform/rotation: keys | [[10,-150],[22,30]] | [[10,-150],[22,30]] | yes |
| FX-ANP-015 | transform/scale: keys | [[10,[0,0]],[22,[100,100]]] | [[10,[0,0]],[22,[100,100]]] | yes |
| FX-ANP-020 | the file | refused, PRESET_FILE_INVALID, naming skew | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Fade in" holds "skew" in its transform, which this build would lose. | yes |
| FX-ANP-021 | the file | refused, PRESET_FILE_INVALID, naming loop | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Fade in" holds "loop" in its opacity, which this build would lose. | yes |
| FX-ANP-022 | the file | refused, PRESET_FILE_INVALID, naming velocity | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Fade in" holds "velocity" in its opacity, which this build would lose. | yes |
| FX-ANP-023 | the file | refused, PRESET_FILE_INVALID, naming motion | refused, PRESET_FILE_INVALID: This preset file cannot be imported: preset 1 is of the kind "motion", which this build does not have. | yes |
| FX-ANP-024 | the file | refused, PRESET_FILE_INVALID, naming transform | refused, PRESET_FILE_INVALID: This preset file cannot be imported: preset 1 holds "transform", which a preset does not have and this build would lose. | yes |
| FX-ANP-025 | the file | refused, PRESET_FILE_INVALID, naming Nothing | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Nothing" has nothing in it. | yes |
| FX-ANP-026 | the file | refused, PRESET_FILE_INVALID, naming position | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Apart" holds its position taken apart into X and Y. Join them before saving it. | yes |
| FX-ANP-027 | the file | refused, PRESET_FILE_INVALID, naming bounce | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Write-on" holds "bounce" in its trim, which this build would lose. | yes |
| FX-ANP-028 | the file | refused, PRESET_FILE_INVALID, naming thumbnail | refused, PRESET_FILE_INVALID: This preset file cannot be imported: preset 1 holds "thumbnail", which a preset does not have and this build would lose. | yes |
| FX-ANP-029 | the file | refused, PRESET_FILE_INVALID, naming Hollow | refused, PRESET_FILE_INVALID: This preset file cannot be imported: the preset "Hollow" has nothing in it. | yes |
| FX-ANP-S01 | steps to undo | 1 | 1 | yes |
| FX-ANP-S01 | transform/opacity: keys | [[10,0],[22,1],[40,0.5]] | [[10,0],[22,1],[40,0.5]] | yes |
| FX-ANP-S02 | steps to undo | 1 | 1 | yes |
| FX-ANP-S02 | transform/opacity: keys | [[10,1],[22,0],[40,0.5]] | [[10,1],[22,0],[40,0.5]] | yes |
| FX-ANP-S03 | steps to undo | 1 | 1 | yes |
| FX-ANP-S03 | transform/scale: keys | [[10,[0,0]],[18,[110,110]],[22,[100,100]]] | [[10,[0,0]],[18,[110.0,110.0]],[22,[100,100]]] | yes |
| FX-ANP-S04 | steps to undo | 1 | 1 | yes |
| FX-ANP-S04 | transform/position: keys | [[10,[-1620,200]],[22,[300,200]]] | [[10,[-1620,200]],[22,[300,200]]] | yes |
| FX-ANP-S05 | steps to undo | 1 | 1 | yes |
| FX-ANP-S05 | transform/position: keys | [[10,[2220,200]],[22,[300,200]]] | [[10,[2220,200]],[22,[300,200]]] | yes |
| FX-ANP-S06 | steps to undo | 1 | 1 | yes |
| FX-ANP-S06 | transform/position: keys | [[10,[300,-880]],[22,[300,200]]] | [[10,[300,-880]],[22,[300,200]]] | yes |
| FX-ANP-S07 | steps to undo | 1 | 1 | yes |
| FX-ANP-S07 | transform/position: keys | [[10,[300,1280]],[22,[300,200]]] | [[10,[300,1280]],[22,[300,200]]] | yes |
| FX-ANP-S08 | steps to undo | 1 | 1 | yes |
| FX-ANP-S08 | transform/rotation: keys | [[10,-180],[22,0]] | [[10,-180],[22,0]] | yes |
| FX-ANP-S08 | transform/scale: keys | [[10,[0,0]],[22,[100,100]]] | [[10,[0,0]],[22,[100,100]]] | yes |
| FX-ANP-S09 | steps to undo | 1 | 1 | yes |
| FX-ANP-S09 | transform/position: expression | "wiggle(12, 15)" | "wiggle(12, 15)" | yes |
| FX-ANP-S10 | steps to undo | 1 | 1 | yes |
| FX-ANP-S10 | effects: length | 1 | 1 | yes |
| FX-ANP-S10 | effects/-1/parameters/start: keys | [[10,0],[34,100]] | [[10,0],[34,100]] | yes |
| FX-ANP-S11 | steps to undo | 1 | 1 | yes |
| FX-ANP-S11 | shapes/0/trim/start: base | 0 | 0 | yes |
| FX-ANP-S11 | shapes/0/trim/start: keys | [] | [] | yes |
| FX-ANP-S11 | shapes/0/trim/end: keys | [[10,0],[34,100]] | [[10,0],[34,100]] | yes |
| FX-ANP-S11 | shapes/0/trim/offset: base | 0 | 0 | yes |
| FX-ANP-S11 | shapes/0/trim/offset: keys | [] | [] | yes |
| FX-ANP-S11 | shapes/1/trim/start: base | 0 | 0 | yes |
| FX-ANP-S11 | shapes/1/trim/start: keys | [] | [] | yes |
| FX-ANP-S11 | shapes/1/trim/end: keys | [[10,0],[34,100]] | [[10,0],[34,100]] | yes |
| FX-ANP-S11 | shapes/1/trim/offset: base | 0 | 0 | yes |
| FX-ANP-S11 | shapes/1/trim/offset: keys | [] | [] | yes |
| FX-ANP-S12 | steps to undo | 1 | 1 | yes |
| FX-ANP-S12 | effects: length | 1 | 1 | yes |
| FX-ANP-S12 | effects/-1/parameters/completion: keys | [[10,100],[22,0]] | [[10,100],[22,0]] | yes |
| starters | the build carries the fixture file | the same text | the same text | yes |
| FX-ANP-S01 | the sentence the panel shows | The layer fades up from nothing over half a second. | The layer fades up from nothing over half a second. | yes |
| FX-ANP-S02 | the sentence the panel shows | The layer fades away over half a second, from the playhead. | The layer fades away over half a second, from the playhead. | yes |
| FX-ANP-S03 | the sentence the panel shows | The layer grows from nothing, overshoots a little and settles at full size. | The layer grows from nothing, overshoots a little and settles at full size. | yes |
| FX-ANP-S04 | the sentence the panel shows | The layer slides in from the left and stops where it is now. | The layer slides in from the left and stops where it is now. | yes |
| FX-ANP-S05 | the sentence the panel shows | The layer slides in from the right and stops where it is now. | The layer slides in from the right and stops where it is now. | yes |
| FX-ANP-S06 | the sentence the panel shows | The layer slides in from the top and stops where it is now. | The layer slides in from the top and stops where it is now. | yes |
| FX-ANP-S07 | the sentence the panel shows | The layer slides in from the bottom and stops where it is now. | The layer slides in from the bottom and stops where it is now. | yes |
| FX-ANP-S08 | the sentence the panel shows | The layer turns in half a turn while it grows to full size. | The layer turns in half a turn while it grows to full size. | yes |
| FX-ANP-S09 | the sentence the panel shows | The layer shakes about where it is, twelve times a second, up to fifteen pixels. An expression, so it has no keys. | The layer shakes about where it is, twelve times a second, up to fifteen pixels. An expression, so it has no keys. | yes |
| FX-ANP-S10 | the sentence the panel shows | The letters appear one by one over a second. For a text layer. | The letters appear one by one over a second. For a text layer. | yes |
| FX-ANP-S11 | the sentence the panel shows | Each line draws itself along its path over a second. For a shape layer. | Each line draws itself along its path over a second. For a shape layer. | yes |
| FX-ANP-S12 | the sentence the panel shows | The layer is wiped on from the left over half a second. | The layer is wiped on from the left over half a second. | yes |
| starters | how many | 12 | 12 | yes |
| starters | Export presets, then Import presets | the same twelve | the same twelve | yes |

B-608: 110 of 110 checks pass.
