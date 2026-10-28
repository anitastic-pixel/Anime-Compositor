# B-833: F4, Ctrl+Shift+Y, angles in turns, and a wiggle with sliders (D-953)

Written by `b833_small_shortcuts` in `app/src/tests/w12r.rs`.

The first rows run the page's own code under Node; the window rows apply the page's own Wiggle position preset to the reference shot's front drawn layer and read Position after the expression on frames 0 to 47; the rest pin the wiring. The private-app check is `verification/B-833_private_app_table.md`.

**18 of 18 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| angles shown as turns and degrees: 390, -30, -390, 0, 720, 45.5 | 1x+30°, 0x-30°, -1x-30°, 0x+0°, 2x+0°, 0x+45.5° | 1x+30°, 0x-30°, -1x-30°, 0x+0°, 2x+0°, 0x+45.5° | pass |
| typed as turns and degrees: 1x+30, -1x-30, -1x30, 2x0, 0x+45.5°; a plain 45 is left to the sum reader | 390, -390, -390, 720, 45.5,  | 390, -390, -390, 720, 45.5,  | pass |
| F4 three times: the switches alone, the modes alone, both | nomodes, noswitches, none | nomodes, noswitches, none | pass |
| Solid Settings OK on a solid: its size and colour as one drag | /solid.set?layer=s1&drag=1&width=640&height=360&color=1,0,0 then /property.drag_end | /solid.set?layer=s1&drag=1&width=640&height=360&color=1,0,0 then /property.drag_end | pass |
| and a new name in the same drag | /solid.set?layer=s1&drag=1&width=640&height=360&color=0,0,1 then /layer.rename?layer=s1&name=Sky&drag=1 then /property.drag_end | /solid.set?layer=s1&drag=1&width=640&height=360&color=0,0,1 then /layer.rename?layer=s1&name=Sky&drag=1 then /property.drag_end | pass |
| Wiggle position: the expression reads the two new sliders | wiggle(effect("Slider Control")("Slider"), effect("Slider Control 2")("Slider")) | wiggle(effect("Slider Control")("Slider"), effect("Slider Control 2")("Slider")) | pass |
| on a layer that already has a slider, the new ones are numbered after it | wiggle(effect("Slider Control 2")("Slider"), effect("Slider Control 3")("Slider")) | wiggle(effect("Slider Control 2")("Slider"), effect("Slider Control 3")("Slider")) | pass |
| Speed and Amount start at 2 wiggles a second and 30 pixels | 2, 30 | 2, 30 | pass |
| the window: Wiggle position on the front drawn layer | Set the position expression and 2 more, sliders 2, 30, 1 entry | Set the position expression and 2 more, sliders 2, 30, 1 entry | pass |
| over 48 frames Position wanders, never more than 30 pixels from where it was | true, true | true, true | pass |
| Amount set to 0: Position stays where it was | true | true | pass |
| two Ctrl+Z (the Amount, then the preset) put the layer back | true | true | pass |
| every degree number shows turns and degrees | true | true | pass |
| F4 on the keyboard | true | true | pass |
| Ctrl+Shift+Y on the keyboard | true | true | pass |
| the columns F4 hides | true | true | pass |
| Wiggle position in the Animation menu | true | true | pass |
| the command finder's lines | true | true | pass |
