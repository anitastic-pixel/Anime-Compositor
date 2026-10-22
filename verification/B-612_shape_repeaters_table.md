# B-612: repeaters

Written by `tests/b612_shape_repeaters.rs` from `Fixtures/shape_repeaters/`, against D-738 (c). Each line is one check: what `tools/shape_repeater_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-SHPR-001 | frame 0: how many shapes are drawn | 19 | 19 | yes |
| FX-SHPR-001 | drawn 1 | "Dot" {"Row":3}: first point [285.0,135.0], fill opacity 0.25 | "Dot" {"Row":3}: first point [285.0,135.0], fill opacity 0.25 | yes |
| FX-SHPR-001 | drawn 2 | "Pip" {"Row":3,"Stack":0}: first point [265.0,180.0], fill opacity 0.125 | "Pip" {"Row":3,"Stack":0}: first point [265.0,180.0], fill opacity 0.125 | yes |
| FX-SHPR-001 | drawn 3 | "Pip" {"Row":3,"Stack":1}: first point [265.0,200.0], fill opacity 0.25 | "Pip" {"Row":3,"Stack":1}: first point [265.0,200.0], fill opacity 0.25 | yes |
| FX-SHPR-001 | drawn 4 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.5 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.5 | yes |
| FX-SHPR-001 | drawn 5 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.25 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.25 | yes |
| FX-SHPR-001 | drawn 6 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.5 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.5 | yes |
| FX-SHPR-001 | drawn 7 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.75 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.75 | yes |
| FX-SHPR-001 | drawn 8 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.375 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.375 | yes |
| FX-SHPR-001 | drawn 9 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.75 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.75 | yes |
| FX-SHPR-001 | drawn 10 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | yes |
| FX-SHPR-001 | drawn 11 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | yes |
| FX-SHPR-001 | drawn 12 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | yes |
| FX-SHPR-001 | drawn 13 | "Plain" {}: first point [170,270], fill opacity 0.8 | "Plain" {}: first point [170.0,270.0], fill opacity 0.8 | yes |
| FX-SHPR-001 | drawn 14 | "Petal" {"Fan":0}: first point [292.0637477567531,147.36511900412503], fill opacity 0.9 | "Petal" {"Fan":0}: first point [292.0637477567531,147.36511900412503], fill opacity 0.9 | yes |
| FX-SHPR-001 | drawn 15 | "Petal" {"Fan":1}: first point [300.0,145.0], fill opacity 0.9 | "Petal" {"Fan":1}: first point [300.0,145.0], fill opacity 0.9 | yes |
| FX-SHPR-001 | drawn 16 | "Petal" {"Fan":2}: first point [307.25,146.53589838486224], fill opacity 0.9 | "Petal" {"Fan":2}: first point [307.25,146.53589838486224], fill opacity 0.9 | yes |
| FX-SHPR-001 | drawn 17 | "Petal" {"Fan":3}: first point [312.2096614865054,150.5], fill opacity 0.9 | "Petal" {"Fan":3}: first point [312.2096614865054,150.5], fill opacity 0.9 | yes |
| FX-SHPR-001 | drawn 18 | "Petal" {"Fan":4}: first point [314.29148931702997,155.23027475611596], fill opacity 0.9 | "Petal" {"Fan":4}: first point [314.29148931702997,155.23027475611596], fill opacity 0.9 | yes |
| FX-SHPR-001 | drawn 19 | "Petal" {"Fan":5}: first point [313.7854898855635,159.3402363728671], fill opacity 0.9 | "Petal" {"Fan":5}: first point [313.7854898855635,159.3402363728671], fill opacity 0.9 | yes |
| FX-SHPR-001 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPR-002 | frame 6: how many shapes are drawn | 19 | 19 | yes |
| FX-SHPR-002 | drawn 1 | "Dot" {"Row":3}: first point [285.0,135.0], fill opacity 0.125 | "Dot" {"Row":3}: first point [285.0,135.0], fill opacity 0.125 | yes |
| FX-SHPR-002 | drawn 2 | "Pip" {"Row":3,"Stack":0}: first point [265.0,180.0], fill opacity 0.0625 | "Pip" {"Row":3,"Stack":0}: first point [265.0,180.0], fill opacity 0.0625 | yes |
| FX-SHPR-002 | drawn 3 | "Pip" {"Row":3,"Stack":1}: first point [265.0,200.0], fill opacity 0.125 | "Pip" {"Row":3,"Stack":1}: first point [265.0,200.0], fill opacity 0.125 | yes |
| FX-SHPR-002 | drawn 4 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.5 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.5 | yes |
| FX-SHPR-002 | drawn 5 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.25 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.25 | yes |
| FX-SHPR-002 | drawn 6 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.5 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.5 | yes |
| FX-SHPR-002 | drawn 7 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.75 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.75 | yes |
| FX-SHPR-002 | drawn 8 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.375 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.375 | yes |
| FX-SHPR-002 | drawn 9 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.75 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.75 | yes |
| FX-SHPR-002 | drawn 10 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | yes |
| FX-SHPR-002 | drawn 11 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | yes |
| FX-SHPR-002 | drawn 12 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | yes |
| FX-SHPR-002 | drawn 13 | "Plain" {}: first point [170,270], fill opacity 0.8 | "Plain" {}: first point [170.0,270.0], fill opacity 0.8 | yes |
| FX-SHPR-002 | drawn 14 | "Petal" {"Fan":0}: first point [296.0606890177159,146.00048997800985], fill opacity 0.9 | "Petal" {"Fan":0}: first point [296.0606890177159,146.00048997800985], fill opacity 0.9 | yes |
| FX-SHPR-002 | drawn 15 | "Petal" {"Fan":1}: first point [303.729390464237,145.6533337816992], fill opacity 0.9 | "Petal" {"Fan":1}: first point [303.729390464237,145.6533337816992], fill opacity 0.9 | yes |
| FX-SHPR-002 | drawn 16 | "Petal" {"Fan":2}: first point [309.8627719926299,148.4802974922387], fill opacity 0.9 | "Petal" {"Fan":2}: first point [309.8627719926299,148.4802974922387], fill opacity 0.9 | yes |
| FX-SHPR-002 | drawn 17 | "Petal" {"Fan":3}: first point [313.3711361161087,152.89222801471902], fill opacity 0.9 | "Petal" {"Fan":3}: first point [313.3711361161087,152.89222801471902], fill opacity 0.9 | yes |
| FX-SHPR-002 | drawn 18 | "Petal" {"Fan":4}: first point [314.12026659198517,157.35224879387042], fill opacity 0.9 | "Petal" {"Fan":4}: first point [314.12026659198517,157.35224879387042], fill opacity 0.9 | yes |
| FX-SHPR-002 | drawn 19 | "Petal" {"Fan":5}: first point [312.69714666193937,160.74189402114231], fill opacity 0.9 | "Petal" {"Fan":5}: first point [312.69714666193937,160.74189402114231], fill opacity 0.9 | yes |
| FX-SHPR-002 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPR-003 | frame 12: how many shapes are drawn | 16 | 16 | yes |
| FX-SHPR-003 | drawn 1 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.25 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.25 | yes |
| FX-SHPR-003 | drawn 2 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.125 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.125 | yes |
| FX-SHPR-003 | drawn 3 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.25 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.25 | yes |
| FX-SHPR-003 | drawn 4 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.625 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.625 | yes |
| FX-SHPR-003 | drawn 5 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.3125 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.3125 | yes |
| FX-SHPR-003 | drawn 6 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.625 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.625 | yes |
| FX-SHPR-003 | drawn 7 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | yes |
| FX-SHPR-003 | drawn 8 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | yes |
| FX-SHPR-003 | drawn 9 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | yes |
| FX-SHPR-003 | drawn 10 | "Plain" {}: first point [170,270], fill opacity 0.8 | "Plain" {}: first point [170.0,270.0], fill opacity 0.8 | yes |
| FX-SHPR-003 | drawn 11 | "Petal" {"Fan":0}: first point [300.0,145.0], fill opacity 0.9 | "Petal" {"Fan":0}: first point [300.0,145.0], fill opacity 0.9 | yes |
| FX-SHPR-003 | drawn 12 | "Petal" {"Fan":1}: first point [307.25,146.53589838486224], fill opacity 0.9 | "Petal" {"Fan":1}: first point [307.25,146.53589838486224], fill opacity 0.9 | yes |
| FX-SHPR-003 | drawn 13 | "Petal" {"Fan":2}: first point [312.2096614865054,150.5], fill opacity 0.9 | "Petal" {"Fan":2}: first point [312.2096614865054,150.5], fill opacity 0.9 | yes |
| FX-SHPR-003 | drawn 14 | "Petal" {"Fan":3}: first point [314.29148931702997,155.23027475611596], fill opacity 0.9 | "Petal" {"Fan":3}: first point [314.29148931702997,155.23027475611596], fill opacity 0.9 | yes |
| FX-SHPR-003 | drawn 15 | "Petal" {"Fan":4}: first point [313.7854898855635,159.3402363728671], fill opacity 0.9 | "Petal" {"Fan":4}: first point [313.7854898855635,159.3402363728671], fill opacity 0.9 | yes |
| FX-SHPR-003 | drawn 16 | "Petal" {"Fan":5}: first point [311.5416196322701,161.98530153522884], fill opacity 0.9 | "Petal" {"Fan":5}: first point [311.5416196322701,161.98530153522884], fill opacity 0.9 | yes |
| FX-SHPR-003 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPR-004 | frame 18: how many shapes are drawn | 16 | 16 | yes |
| FX-SHPR-004 | drawn 1 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.125 | "Dot" {"Row":2}: first point [215.0,135.0], fill opacity 0.125 | yes |
| FX-SHPR-004 | drawn 2 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.0625 | "Pip" {"Row":2,"Stack":0}: first point [195.0,180.0], fill opacity 0.0625 | yes |
| FX-SHPR-004 | drawn 3 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.125 | "Pip" {"Row":2,"Stack":1}: first point [195.0,200.0], fill opacity 0.125 | yes |
| FX-SHPR-004 | drawn 4 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.625 | "Dot" {"Row":1}: first point [145.0,135.0], fill opacity 0.625 | yes |
| FX-SHPR-004 | drawn 5 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.3125 | "Pip" {"Row":1,"Stack":0}: first point [125.0,180.0], fill opacity 0.3125 | yes |
| FX-SHPR-004 | drawn 6 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.625 | "Pip" {"Row":1,"Stack":1}: first point [125.0,200.0], fill opacity 0.625 | yes |
| FX-SHPR-004 | drawn 7 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | "Dot" {"Row":0}: first point [75.0,135.0], fill opacity 1 | yes |
| FX-SHPR-004 | drawn 8 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | "Pip" {"Row":0,"Stack":0}: first point [55.0,180.0], fill opacity 0.5 | yes |
| FX-SHPR-004 | drawn 9 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | "Pip" {"Row":0,"Stack":1}: first point [55.0,200.0], fill opacity 1 | yes |
| FX-SHPR-004 | drawn 10 | "Plain" {}: first point [170,270], fill opacity 0.8 | "Plain" {}: first point [170.0,270.0], fill opacity 0.8 | yes |
| FX-SHPR-004 | drawn 11 | "Petal" {"Fan":0}: first point [303.729390464237,145.6533337816992], fill opacity 0.9 | "Petal" {"Fan":0}: first point [303.729390464237,145.6533337816992], fill opacity 0.9 | yes |
| FX-SHPR-004 | drawn 12 | "Petal" {"Fan":1}: first point [309.8627719926299,148.4802974922387], fill opacity 0.9 | "Petal" {"Fan":1}: first point [309.8627719926299,148.4802974922387], fill opacity 0.9 | yes |
| FX-SHPR-004 | drawn 13 | "Petal" {"Fan":2}: first point [313.3711361161087,152.89222801471902], fill opacity 0.9 | "Petal" {"Fan":2}: first point [313.3711361161087,152.89222801471902], fill opacity 0.9 | yes |
| FX-SHPR-004 | drawn 14 | "Petal" {"Fan":3}: first point [314.12026659198517,157.35224879387042], fill opacity 0.9 | "Petal" {"Fan":3}: first point [314.12026659198517,157.35224879387042], fill opacity 0.9 | yes |
| FX-SHPR-004 | drawn 15 | "Petal" {"Fan":4}: first point [312.69714666193937,160.74189402114231], fill opacity 0.9 | "Petal" {"Fan":4}: first point [312.69714666193937,160.74189402114231], fill opacity 0.9 | yes |
| FX-SHPR-004 | drawn 16 | "Petal" {"Fan":5}: first point [310.0625940988206,162.5210611504313], fill opacity 0.9 | "Petal" {"Fan":5}: first point [310.0625940988206,162.5210611504313], fill opacity 0.9 | yes |
| FX-SHPR-004 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPR-005 | the groups and their repeaters, blend_mode included | as the reference | as the reference | yes |
| FX-SHPR-005 | saved, then read again | the same repeaters | the same repeaters | yes |
| FX-SHPR-020 | refused_copies.json | refused, PROJECT_SCHEMA_INVALID, naming copies | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Row"'s repeater copies from 0 to 1000, not -1. | yes |
| FX-SHPR-021 | refused_copies_key.json | refused, PROJECT_SCHEMA_INVALID, naming copies | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Fan"'s repeater copies from 0 to 1000, not 1200. | yes |
| FX-SHPR-022 | refused_offset.json | refused, PROJECT_SCHEMA_INVALID, naming offset | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Fan"'s repeater offset from -1000 to 1000, not 1001. | yes |
| FX-SHPR-023 | refused_opacity.json | refused, PROJECT_SCHEMA_INVALID, naming start opacity | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Row"'s repeater start opacity from 0 to 1, not 1.5. | yes |
| FX-SHPR-024 | refused_composite.json | refused, PROJECT_SCHEMA_INVALID, naming composite | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups/0/repeater/composite: expected one of below, above. | yes |
| FX-SHPR-025 | refused_no_rotation.json | refused, PROJECT_SCHEMA_INVALID, naming rotation | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Stack"'s repeater rotation, which is missing. | yes |
| FX-SHPR-026 | refused_expression.json | refused, PROJECT_SCHEMA_INVALID, naming expression | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected no expression on group "Fan"'s repeater offset: it takes keys only. | yes |

B-612: 87 of 87 checks pass.
