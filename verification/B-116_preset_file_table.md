# B-116: effect presets in a file

Written by `tests/b116_preset_file.rs` from `Fixtures/preset_file/`, against D-180. Each line is one preset file: what `tools/preset_file_reference.py` says the build must do with it, and what the build did. A refused file must import nothing, and its message must name what was wrong.

| Case | Says | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-PRE-001 | One preset holding one effect: the smallest file. | read "Soft glow": core.glow | read "Soft glow": core.glow | yes |
| FX-PRE-001 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-002 | Three presets, read in the file's order, each with its effects in their order. | read "Night": core.invert, core.vignette; "Poster": core.posterize; "Lifted": core.drop_shadow, core.glow, core.posterize | read "Night": core.invert, core.vignette; "Poster": core.posterize; "Lifted": core.drop_shadow, core.glow, core.posterize | yes |
| FX-PRE-002 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-003 | A keyed setting: the keys come with the preset, at the frames they were at. | read "Flash": core.exposure | read "Flash": core.exposure | yes |
| FX-PRE-003 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-004 | An effect switched off stays off. | read "Off for now": core.glow, core.invert | read "Off for now": core.glow, core.invert | yes |
| FX-PRE-004 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-005 | A name in Japanese, with inner spaces, is kept exactly as written. | read "撃ち合い  光 2": core.glow | read "撃ち合い  光 2": core.glow | yes |
| FX-PRE-005 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-006 | A setting outside its range is read and kept, as a project file's is (D-46); the effect is not drawn where the preset is applied, and the layer says why. | read "Wrong radius": core.glow | read "Wrong radius": core.glow | yes |
| FX-PRE-006 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-007 | The same effect id in two presets does not matter: pasting gives each new ids. | read "A": core.glow; "B": core.glow | read "A": core.glow; "B": core.glow | yes |
| FX-PRE-007 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-008 | Names differing only by capitals are two presets. | read "glow": core.glow; "Glow": core.glow | read "glow": core.glow; "Glow": core.glow | yes |
| FX-PRE-008 | Exported and imported again. | the same presets | the same presets | yes |
| FX-PRE-009 | Not JSON: a text file chosen by mistake. | refuse, `PRESET_FILE_INVALID`, naming JSON | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: it is not JSON text, so it is not a preset file. | yes |
| FX-PRE-010 | A project file chosen by mistake is named as one. | refuse, `PRESET_FILE_INVALID`, naming project | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: it is a project file, not a preset file. Open it with Open instead. | yes |
| FX-PRE-011 | A newer version is refused, never guessed at. | refuse, `PRESET_FILE_INVALID`, naming newer | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: it was written by a newer version of this program, whose presets this one does not read. | yes |
| FX-PRE-012 | Something the rule does not name, beside the presets, would be lost, so the file is refused. | refuse, `PRESET_FILE_INVALID`, naming author | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: it holds "author", which a preset file does not have and this build would lose. | yes |
| FX-PRE-013 | Something the rule does not name, in a preset, likewise. | refuse, `PRESET_FILE_INVALID`, naming thumbnail | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: preset 1 holds "thumbnail", which a preset does not have and this build would lose. | yes |
| FX-PRE-013 | The same presets handed to Export. | write nothing | wrote nothing: This preset file cannot be exported: preset 1 holds "thumbnail", which a preset does not have and this build would lose. | yes |
| FX-PRE-014 | A name of spaces only. | refuse, `PRESET_FILE_INVALID`, naming name | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: preset 1 has no name. | yes |
| FX-PRE-014 | The same presets handed to Export. | write nothing | wrote nothing: This preset file cannot be exported: preset 1 has no name. | yes |
| FX-PRE-015 | Two presets with one name: which one was meant is not known. | refuse, `PRESET_FILE_INVALID`, naming Soft glow | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: two presets are named "Soft glow". | yes |
| FX-PRE-015 | The same presets handed to Export. | write nothing | wrote nothing: This preset file cannot be exported: two presets are named "Soft glow". | yes |
| FX-PRE-016 | A preset with no effects. | refuse, `PRESET_FILE_INVALID`, naming Empty | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: the preset "Empty" has no effects. | yes |
| FX-PRE-016 | The same presets handed to Export. | write nothing | wrote nothing: This preset file cannot be exported: the preset "Empty" has no effects. | yes |
| FX-PRE-017 | A file with no presets. | refuse, `PRESET_FILE_INVALID`, naming presets | refused, `PRESET_FILE_INVALID`: This preset file cannot be imported: it holds no presets. | yes |
| FX-PRE-017 | The same presets handed to Export. | write nothing | wrote nothing: This preset file cannot be exported: it holds no presets. | yes |
| FX-PRE-018 | An effect this build does not have, in the last of three presets, refuses the whole file: the first two are not taken either. | refuse, `EFFECT_UNSUPPORTED`, naming core.lens_sparkle | refused, `EFFECT_UNSUPPORTED`: The effect core.lens_sparkle cannot be imported: this build does not have it. | yes |
| FX-PRE-018 | The same presets handed to Export. | write nothing | wrote nothing: The effect core.lens_sparkle cannot be exported: this build does not have it. | yes |
| FX-PRE-019 | A setting this build does not have would be lost, so the file is refused. | refuse, `EFFECT_UNSUPPORTED`, naming sparkle | refused, `EFFECT_UNSUPPORTED`: The effect core.glow cannot be imported: it carries sparkle, which this build does not understand and would not keep. | yes |
| FX-PRE-019 | The same presets handed to Export. | write nothing | wrote nothing: The effect core.glow cannot be exported: it carries sparkle, which this build does not understand and would not keep. | yes |

**B-116: 34 of 34 checks pass.**
