# B-230: text animators

D-350 (EFFECTS.md P0-7, part 1): After Effects' text animator, with Position, Scale, Rotation, Opacity, Fill Color and Tracking, and its Range Selector, applied to each character by how far the selector picks it. Every expected number is `Fixtures/text_animator/expected_text_animator.json`, written by `tools/text_animator_reference.py` before this code existed and printed in document 25 as FX-TXA-001 to 023. Per character the build gives the foot it turns and scales about, the move, scale, turn, opacity and colour, and each animator's amount, compared within 1e-6, and the box of its drawn outline, within a quarter of a pixel (the build cuts curves into pieces about 2 pixels long; the reference takes each curve's exact extent).

## FX-TXA-001 to 013: every character (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TXA-001: Typewriter, hard: Opacity 0, Start 40 per cent of 15 characters, Smoothness 0. The first six show; the rest are not drawn. | 15 characters; numbers within 5.7e-14, boxes within 0.056 px | yes |
| FX-TXA-002: Typewriter, soft: Start 43.3, Smoothness 100. The seventh character is half inside the range and half faded. | 15 characters; numbers within 5.7e-14, boxes within 0.056 px | yes |
| FX-TXA-003: Fade in by character (tutorial 1): Opacity 0, Ramp Up, Ease High and Low 50, Offset -30. Left letters show, a soft edge, right letters hidden. | 17 characters; numbers within 1.2e-12, boxes within 0.023 px | yes |
| FX-TXA-004: Fade out (tutorial 1's second animator): Ramp Down, Offset 30. | 17 characters; numbers within 1.2e-12, boxes within 0.023 px | yes |
| FX-TXA-005: Word by word (tutorial 2): Position 0, 100, Opacity 0, Based On Words, Ramp Up, Ease High 25, Ease Low 100, Offset -20. Spaces are never picked. | 19 characters; numbers within 3.6e-13, boxes within 0.056 px | yes |
| FX-TXA-006: A wave (tutorial 3): Position 0, -60, Start 20, End 50, Offset 10, Triangle. The letters in the range rise, most in its middle. | 15 characters; numbers within 5.7e-14, boxes within 0.072 px | yes |
| FX-TXA-007: Smooth and Round: a smooth hump of Position 0, -40 over Start 0 to End 60, Offset 20, and a round one of Scale 150 over the whole line. | 15 characters; numbers within 2.8e-14, boxes within 0.000 px | yes |
| FX-TXA-008: Turn, stretch, room and colour: Rotation 30, Scale 150 by 50 on all; then Rotation -10, Tracking 50, Fill on in green on the second half, faux italic, centred at 640. | 12 characters; numbers within 5.7e-14, boxes within 0.073 px | yes |
| FX-TXA-009: Characters excluding spaces, Start 80 above End 20 (taken the other way), Amount -50, Position 0, 40: picked letters move up by 20. | 9 characters; numbers within 2.8e-14, boxes within 0.005 px | yes |
| FX-TXA-010: Two lines: the line break is not a character. Square, Start 25, End 75 of the four letters: B and C picked, scaled 200. | 4 characters; numbers within 0.0e0, boxes within 0.029 px | yes |
| FX-TXA-011: The timing line: 30 characters with two animators, a wave and a fade by character. | 30 characters; numbers within 6.8e-13, boxes within 0.120 px | yes |
| FX-TXA-012: A range of no width (Start and End 50): Ramp Up picks all after it, the square picks none. | 11 characters; numbers within 2.8e-14, boxes within 0.023 px | yes |
| FX-TXA-013: Room by the animator on a right-aligned line with the text's own tracking 100: Tracking 200 at Amount 50 on the first half moves the start of the line left. | 10 characters; numbers within 2.3e-13, boxes within 0.011 px | yes |

## The pictures the numbers make

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TXA-001: left of x = 430 the line as with no animator, pixel for pixel; right of it nothing | left the same, right empty | yes |
| FX-TXA-002: the seventh letter, at opacity 0.4950, has that share of its ink | strongest alpha 0.4950 of 1.0000 | yes |
| FX-TXA-008: the last letter's solid pixels are its Fill Color [0.0, 1.0, 0.25] | 649 solid pixels, largest difference 0.0e0 | yes |
| FX-TXA-006: each letter's top edge is raised by its own move | largest difference 0 px | yes |

## FX-TXA-020 to 023: the files

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TXA-020 frame 0: Typewriter keyed: Start 0 at frame 0 to 100 at frame 15, linear, Smoothness 0, on "Typewriter text"; frame 6 is Start 40, FX-TXA-001. | characters shown []; the frame is the animated line | yes |
| FX-TXA-020 frame 15: Typewriter keyed: Start 0 at frame 0 to 100 at frame 15, linear, Smoothness 0, on "Typewriter text"; frame 6 is Start 40, FX-TXA-001. | characters shown [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]; the frame is the animated line | yes |
| FX-TXA-020 frame 6: Typewriter keyed: Start 0 at frame 0 to 100 at frame 15, linear, Smoothness 0, on "Typewriter text"; frame 6 is Start 40, FX-TXA-001. | characters shown [0, 1, 2, 3, 4, 5]; the frame is the animated line | yes |
| FX-TXA-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TXA-021 frame 0: A setting this build does not know ("selector_mode") is kept as written and the animator still works. | characters shown [0, 1, 2, 3, 4, 5]; the frame is the animated line | yes |
| FX-TXA-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TXA-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TXA-023: what opening it warns of, and what frame 4 warns of | [] and ["TEXT_ANIMATOR_NO_TEXT"] | yes |
| fx_txa_022.json: with a shape it does not know, the line is drawn as with no animator | the same | yes |
| fx_txa_023.json: the solid is drawn as with no animator | the same | yes |
| fx_txa_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_txa_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_txa_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_txa_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_txa_021.json: the unknown "selector_mode" is saved as written | "add" | yes |
| a file with a Start written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Position of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no End is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands: keys, an expression, a refusal, undo

| Check | The build's answer | Matches |
| --- | --- | --- |
| Position keyed 0, 0 at frame 0 to 0, -80 at frame 8: at frame 4 it is 0, -40 | taken ; TextAnimator { position: [0.0, -40.0], scale: [100.0, 100.0], rotation: 0.0, opacity: 0.0, fill: "off", color: [1.0, 0.0, 0.0], tracking: 0.0, start: 40.0, end: 100.0, offset: 0.0, amount: 100.0, based_on: "characters", shape: "square", smoothness: 0.0, ease_high: 0.0, ease_low: 0.0 } | yes |
| Fill colour keyed red at 0 to blue at 8: at frame 4 it is half of each | taken ; TextAnimator { position: [0.0, -40.0], scale: [100.0, 100.0], rotation: 0.0, opacity: 0.0, fill: "off", color: [0.5, 0.0, 0.5], tracking: 0.0, start: 40.0, end: 100.0, offset: 0.0, amount: 100.0, based_on: "characters", shape: "square", smoothness: 0.0, ease_high: 0.0, ease_low: 0.0 } | yes |
| Offset keyed -100 at 0 to 100 at 8: at frame 2 it is -50 | taken ; TextAnimator { position: [0.0, -20.0], scale: [100.0, 100.0], rotation: 0.0, opacity: 0.0, fill: "off", color: [0.75, 0.0, 0.25], tracking: 0.0, start: 40.0, end: 100.0, offset: -50.0, amount: 100.0, based_on: "characters", shape: "square", smoothness: 0.0, ease_high: 0.0, ease_low: 0.0 } | yes |
| Start given the expression "time*24*10": at frame 3 it is 30 | taken ; TextAnimator { position: [0.0, -30.0], scale: [100.0, 100.0], rotation: 0.0, opacity: 0.0, fill: "off", color: [0.625, 0.0, 0.375], tracking: 0.0, start: 30.0, end: 100.0, offset: -25.0, amount: 100.0, based_on: "characters", shape: "square", smoothness: 0.0, ease_high: 0.0, ease_low: 0.0 } | yes |
| Opacity keyed 150 (past its 0 to 100) is refused with a sentence | Text Animator's opacity runs from 0 to 100, and this is 150. | yes |
| the shape "wiggly" is refused with a sentence | Text Animator's shape is "square", "ramp_up", "ramp_down", "triangle", "round" or "smooth", and this is "wiggly". | yes |
| undo 4 times: frame 0 is what it was | byte-identical | yes |

## The card: the letters drawn on the processor, the frame shown through the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_txa_020.json frame 0: the card's frame against the processor's | largest 0 level(s); warnings [] | yes |
| fx_txa_020.json frame 6: the card's frame against the processor's | largest 0 level(s); warnings [] | yes |
| fx_txa_020.json frame 15: the card's frame against the processor's | largest 0 level(s); warnings [] | yes |
| fx_txa_023.json frame 0: the card's frame against the processor's | largest 0 level(s); warnings ["TEXT_ANIMATOR_NO_TEXT"] | yes |
| fx_txa_023.json frame 6: the card's frame against the processor's | largest 0 level(s); warnings ["TEXT_ANIMATOR_NO_TEXT"] | yes |
| fx_txa_023.json frame 15: the card's frame against the processor's | largest 0 level(s); warnings ["TEXT_ANIMATOR_NO_TEXT"] | yes |

## Pictures, in `verification/D-350 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| typewriter_frame_00.png: the typewriter at frame 0 of 15, Start 0 | written | yes |
| typewriter_frame_03.png: the typewriter at frame 3 of 15, Start 20 | written | yes |
| typewriter_frame_06.png: the typewriter at frame 6 of 15, Start 40 | written | yes |
| typewriter_frame_10.png: the typewriter at frame 10 of 15, Start 66 | written | yes |
| typewriter_frame_15.png: the typewriter at frame 15 of 15, Start 100 | written | yes |
| typewriter_soft.png (FX-TXA-002): Smoothness 100: the seventh letter half faded | written | yes |
| fade_in_by_character.png (FX-TXA-003): Ramp Up, eased: bright on the left, fading to nothing on the right | written | yes |
| fade_out_by_character.png (FX-TXA-004): Ramp Down: the other way round | written | yes |
| word_by_word.png (FX-TXA-005): Based On Words: whole words lowered and faded together | written | yes |
| wave.png (FX-TXA-006): Triangle: the letters in the range raised, most in its middle | written | yes |
| humps.png (FX-TXA-007): Smooth and Round: a smooth hump and a swelling in the middle | written | yes |
| turn_scale_colour.png (FX-TXA-008): turned, squashed, spaced and the second half green | written | yes |
| thirty_characters.png (FX-TXA-011): the timing line: a wave and a fade together | written | yes |

## Result

61 of 61 checks pass.
