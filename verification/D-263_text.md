# D-263: Text

Text layers, at your request ("implement the text tool as well"). The **Text tool** is the **T** button in the viewer's tools, after the shape tool. Click the picture where the words should start. That makes a layer called "Text 1", reading "Text", in white, 120 pixels high on a 1080-high cut. The new layer is selected, the tool goes back to Select, and the cursor is in the words box in Effect controls, ready to type.

You can also make one from **Layer › New › Text**, the timeline's **+ New layer › Text**, or the command search (Ctrl+Shift+P, "Text tool" or "New text layer"). Those put it at the middle of the picture, centred. The **T key** stays the opacity reveal, so the tool has no key of its own. **Ctrl+T** stays the free-transform box.

What Effect controls has for a text layer:

- **Text**: the words. Enter starts a new line below. They are sent when you leave the box.
- **Font**: a font file's name. Choose the one that comes with the program (M PLUS Rounded 1c, which has Japanese) or one from the list, or type any font file name in the Windows fonts folder.
- **Size**, in pixels, 1 to 2000.
- **Colour**.
- **Starts at X / Y**: where the first line begins, in the cut's pixels.
- **Align**: Left, Centre or Right, about that point.

Each change is one step for Ctrl+Z. The layer moves, scales, turns and fades with its Transform like any other layer, and takes effects.

If a project names a font this computer does not have, the words are kept but nothing is drawn: no other font is put in its place. The warning count in the top bar goes up by one, the line "Layer … 's font … is not on this computer, so its words are not drawn" shows under the timeline, and **Error details** names it `TEXT_FONT_MISSING`. An export of that frame is marked incomplete. Open the project on a computer with the font, or choose another, and the words come back.

## Pictures (`D-263 pictures/`)

These are photographs of the running window, on the reference shot.

- `1_placed_where_clicked.png`: the Text tool clicked at 12% across and 30% down (230, 324 in the cut's pixels).
  - The words were changed to "Cut 012" and "あいう" on a second line, and the colour to yellow.
  - The words begin where the click was. Effect controls shows the Text section, and the timeline has "Text 1 · text" on top.
- `2_centred_with_japanese.png`: the same layer with Align set to Centre. Both lines are now centred on the same point. "あいう" is drawn in the font that comes with the program.
- `3_missing_font_said.png`: the font changed to `NoSuchFont.ttf`.
  - The words are gone from the picture, and the top bar's warning count reads 2 instead of 1.
  - The line under the timeline and Error details say `TEXT_FONT_MISSING`.

## The check, written first

`tests/d263_text.rs`, committed in c464052. **On the build before the change**, it did not build, because there were no text layers. **All 39 checks now pass**, in `D-263_text_table.md`. In short:

| Check | Result |
|---|---|
| A text layer made and changed through the commands the page sends; sizes, colours, places and font names outside the ranges refused; undo puts the words back | pass |
| Saved and opened again, the layer is the same; saved again, byte for byte the same; lines no build writes yet are kept | pass |
| A text record on a drawn layer, a text layer without one, or one holding exposures, is refused when the file opens | pass |
| The letters land where they were placed, the same every time; left, right and centred land where they should; a second line is below the first; overlapping outlines are filled and an O keeps its hole | pass |
| Frame 10 changes with the layer on, and is byte for byte the frame without it with the layer off | pass |
| A missing font is said as `TEXT_FONT_MISSING`, the frame is marked incomplete, and nothing is drawn in its place | pass |
| All 2297 fixture projects save byte for byte as before (one SHA-256 over all of them) | pass |

## In the running app

| Step | What happened |
|---|---|
| The T button | The tool is Text, the pointer is a text cursor, and the status line says "The text tool: click the picture where the words should start." |
| A click at 12% across, 30% down | 4 layers became 5. "Text 1" was selected, the tool went back to Select, the cursor was in the words box, and the text starts at 230, 324 |
| Words "Cut 012", a new line, then "あいう"; colour yellow | Both lines drawn in yellow from the click |
| Align: Centre | Both lines centred on 230 |
| Font `NoSuchFont.ttf` | Nothing drawn. The warning count went from 1 to 2, and `TEXT_FONT_MISSING` was in Error details |
| Font `nothing` | Refused: "it needs a font's file name ending .ttf, .otf or .ttc, not "nothing"" |
| Ctrl+Z five times | Font, then align, then colour, then words were undone, then the layer itself; back to 4 layers. The warning count was back to 1 |
| Errors on the page | none |

The click was real mouse input sent to the window. The window's own settings were put back afterwards. No project file was saved.

## Faults found and fixed on the way

- **A missing font was said only in the session log** at first, because the viewer had no way to show a frame's warnings. A frame now carries its missing fonts beside the window's other notes, so the count and Error details show them. Missing drawings still use their own Relink line and are not repeated.
- **A click with the Text tool at Draft** would have put the words a quarter of the way to where you clicked, because a Draft picture is a quarter size. The click is now converted to the cut's own pixels, as a drag already is.
- **The check's own spelling of whole numbers** was changed to match how the project file writes them (`1`, not `1.0`) before the build. Only how the check writes its expected text changed. What it compares did not.

## Checks

- All 88 of the app's tests pass. The page, route, keyboard and command-map lists (B-12b, B-12c) gain the Text tool, the two commands, and `layer.source_text` in the window's answer.
- The whole core suite passes. The offline record (B-11) and the dependency record gain `ttf-parser` 0.25.1, which reads font files and has no dependencies of its own. Its licences are in `Licenses/ttf-parser-0.25.1/`. The diagnostic catalogue (B-12b) gains `TEXT_FONT_MISSING`.
- The font that comes with the program is M PLUS Rounded 1c, under the SIL Open Font License, in `assets/fonts/` with the licence in `docs/third_party/`.
- No fixture, existing saved project byte, or exported picture changes.

## Limits, stated

- **One style per layer.** The size, colour and font apply to the whole layer; there is no bold or colour on single words.
- **The words are not animated.** Size, colour, place and words have no keys. Animate the layer's Transform, or use effects.
- **No shaping for joined scripts.** Each letter is drawn on its own, which is right for Japanese and English. Arabic or Hindi would come out with their letters unjoined. Kerning pairs are not applied.
- **No outline, shadow or box behind the words.** Use effects, such as Drop Shadow or Outline, on the layer.
- **No typing directly on the picture.** The words are typed in Effect controls. There is no text box on the picture.
- **Fonts are named by file**, such as `arial.ttf`, from the Windows fonts folder or the one that comes with the program. There is no font picker that shows the fonts by name.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click the **T** button in the viewer's tools, then click on the picture | A layer "Text 1" reading "Text" appears, starting where you clicked; the cursor is in the words box on the right | |
| 2 | Type a few words, Enter, more words, then click elsewhere | Two lines on the picture, the second below the first | |
| 3 | Type Japanese, e.g. あいう | It is drawn | |
| 4 | Change Size, Colour, Starts at X/Y, and Align | The words change to match each one | |
| 5 | Press **Ctrl+Z** a few times | Each change is undone one at a time | |
| 6 | Type `arial.ttf` in Font | The words change to Arial | |
| 7 | Type `NoSuchFont.ttf` in Font | The words disappear; the warning count in the top bar goes up; Error details says TEXT_FONT_MISSING | |
| 8 | Move, scale or turn the layer, or add an effect | The words follow like any layer | |
| 9 | **Layer › New › Text** | A "Text 2" centred in the middle of the picture | |
| 10 | Save, close, open the project again | The text layers are back as they were | |
| 11 | Export a few frames | The words are in the exported frames | |

Anything marked ✗, tell me the row number.
