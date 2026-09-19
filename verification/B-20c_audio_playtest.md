# B-20c playtest: reference audio, and the new layer bars

For a person, by ear and by eye. The sentences the window says and the refusals are already checked in `verification/B-20c_panel_table.md`. This sheet covers what a table cannot: what is heard, whether it stays with the picture, and what the bars look like.

Have any project open with a few frames of drawings, and one sound file ready. A WAV is best. An MP3 is a good second try. A clap, a count or a line of dialogue makes sync easy to judge.

Write **yes** or **no** beside each step, and what you saw or heard when it is no.

## Import and the layer

| # | Do this | You should find | Yes or no |
|---|---|---|---|
| 1 | Press Import. Look at the file type list in the dialog. | It says "Drawings and sound", and your sound file can be chosen. | |
| 2 | Choose the sound file. | The status line says it is a sound and tells you to drag it to the layer list. The bin shows it with the word "sound". | |
| 2b | Drag a sound file from Explorer onto the window. | It is imported the same way as step 2. It is not treated as a project to open. | |
| 3 | Drag it to the layer list, or select it and press Make a layer. | A new layer at the front. Its row says "sound", has a speaker where the eye would be, and has no blend mode list and no arrow to open properties. | |
| 4 | Look at the layer's bar. | It is green, and the shape of the sound is drawn inside it a moment after the layer appears. Loud parts are tall and silence is flat. | |
| 5 | Select the sound layer and look at the panels on the right. | One Level box in decibels and a line saying sound is heard in the window only. No position, scale, blend, mask, matte or parent. The exposures and effects panels each say a sound has none. | |

## Hearing it

| # | Do this | You should find | Yes or no |
|---|---|---|---|
| 6 | Put the playhead at the start and press Play. | The sound plays with the picture. | |
| 7 | Press Pause. | The sound stops at once. | |
| 8 | Press Play from the middle of the shot. | The sound starts from that point in the file, not from its beginning. | |
| 9 | Watch a clap or a hard consonant against the waveform and the playhead for a whole play through. | The sound and the playhead stay together. They are never more than about three frames apart. | |
| 10 | Step one frame at a time with the arrow keys, or drag the playhead slowly. | Each new frame gives a short blip of the sound at that frame. Staying on a frame is silent. | |
| 11 | Drag the green bar later by ten frames and play from the start. | Ten frames of silence, then the sound. The waveform moved with the bar. | |
| 12 | Trim the end of the bar shorter and play through it. | The sound stops where the bar ends. | |

## Level and speaker

| # | Do this | You should find | Yes or no |
|---|---|---|---|
| 13 | Type -12 into Level, press Enter, and play. | Clearly quieter. | |
| 14 | Type 13 into Level. | The status line refuses it, and the level stays where it was. | |
| 15 | Press the speaker on the row and play. | Silence. The row says "muted". | |
| 16 | Press Ctrl+Z twice, then play. | The speaker is back on and then the level is back. The sound is at full level again. | |
| 17 | Solo a picture layer and play. | Silence, because the sound layer is not the soloed one. Unsolo and it is heard again. | |

## The other bars

| # | Do this | You should find | Yes or no |
|---|---|---|---|
| 18 | Look at the Camera row. | A thin amber strip across the whole timeline. It cannot be dragged or trimmed. | |
| 19 | Add an adjustment layer. | Its bar is hatched, as before. | |
| 20 | Precompose a layer. | The composition layer's bar is violet with the inner composition's name in it. | |
| 21 | Give the sound layer a label colour from the right-click menu. | The label colour wins over the green. The row still says "sound". | |

## Saving and exporting

| # | Do this | You should find | Yes or no |
|---|---|---|---|
| 22 | Save, close the app, open the project again, and play. | The sound layer is there with its level, and it plays. | |
| 23 | Export the shot. | The frames are the same as they would be without the sound layer. No sound file is written. | |
| 24 | Move or rename the sound file on disk, open the project, then select the sound in the bin, press Relink and choose the file. | Before relinking the layer is silent and the project still opens. After relinking it plays. One Ctrl+Z takes the relink back. | |

## Known ceilings, not faults

- A level changed while the shot is playing is heard from the next time Play is pressed.
- The level cannot be keyed.
- Sound inside a composition layer is not heard from the outer composition.
- No sound goes into any export.
- The waveform is drawn from the first channel only.
