# B-608 playtest: animation presets (D-739)

## What to try

1. Select a layer and put the playhead where the animation should start.
2. In the Effects panel, open the **Animation presets** folder (12 built in) and double-click a preset, for example **Pop in** or **Slide in from the left**. Its keys start at the playhead frame.
3. Press Ctrl+Z once. The whole preset comes off in one step.
4. To save your own: animate a layer (Scale, Rotation, a mask, effects...), right-click the layer and choose **Save as animation preset...** (it is also in the menu bar and the Effect Controls menu). Tick the parts to keep, name it and press Enter. It shows up in the Animation presets folder and can be applied to any layer at any frame.

## What the check found (private copy, `Fixtures/animation_preset/host.json`): 14 of 14 pass

- All 12 built-in presets are in the Animation presets folder ("Animation presets (12)").
- Pop in at frame 10 keys Scale at frames 10, 18 and 22, and one undo removes it.
- Slide in from the left keys Position at frames 0 and 12, from [-1620, 200] to [300, 200].
- Typewriter adds a Text Animator to the text layer.
- Write-on on a solid is refused in words: "Write-on" trims the lines of a shape layer, and small is not a shape layer. Nothing was changed.
- Write-on on the shape layer sets its trims.
- The save box offers Scale and Rotation ticks. Saving "My spin" says: Saved the preset "My spin": Scale, Rotation.
- "My spin" applied to the other card at frame 30 keys Rotation at frames 30 and 42.
- No page errors.

Fixture check `tests/b608_animation_preset.rs`: 110 of 110 (see `B-608_animation_preset_table.md`).

## Pictures (`B-608 pictures/`)

- `B-608_sheet.png`: rows show Pop in (f10/14/18/22), Slide in from the left (f0/9/10/12) and Typewriter (f0/6/12/24). The slide card stays off screen until about f9 because it travels a whole screen width.
- `04_save_box.png`: the name box with a tick for each part.
- `05_my_spin_f36.png`: the saved preset partway through, on the other card.

## How this differs from After Effects

- Wipe reveal uses the Linear Wipe effect, not a mask.
- When saving, you tick the parts to keep instead of selecting properties first.
- Shape colours and stroke widths are not saved yet. They come with D-738.
- Numbers aren't scaled to the layer's size: a built-in slide travels a full comp width.
- A separated position (X and Y apart) is skipped, with a note to join them first.
