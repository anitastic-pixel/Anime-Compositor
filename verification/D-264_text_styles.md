# D-264: Text styles

You asked for this: "let's build the text tool/text properties and modify section to be similar to Premiere/Davinci/AE's text tools." Effect controls' Text section is now laid out the way After Effects' Character and Paragraph panels are, and the way Premiere's and Resolve's text controls are. The Text tool works the way their type tools do.

**The Text tool** (the T button in the viewer's tools):

- **Click** the picture to start one line of words there (point text), as D-263 did.
- **Drag** a box to make words that wrap inside it (paragraph text). The box's top-left corner is where you started the drag.
- **Click on existing words** to type into them. With the Select tool, a **double-click** on the words does the same.

**You type on the picture itself.** A dashed box sits over the words, and you see the caret and the selection. The letters you see are the real picture, redrawn as you type. Escape, Ctrl+Enter or a click away finishes. One typing is one step for Ctrl+Z.

A text layer's outline, and what a click on the picture picks, is now the box round its words, not the whole frame.

**Effect controls, for a text layer:**

- **Source text**: the words. They change in the picture as you type.
- **Character**:
  - **Font**: the family and the style, from every font on this computer and the one that comes with the program. A font this computer does not have shows "(not on this computer)".
  - **Size**.
  - **Tracking**: room added between every two letters, in thousandths of the size.
  - **Leading**: from one line to the next, in pixels. 0 is Auto, the font's own spacing.
  - **Kerning**: Metrics moves pairs like AV closer, as the font asks. Off does not.
  - **Style**: **B** faux bold, **I** faux italic, **TT** all caps. The words themselves stay as you typed them.
- **Paragraph**:
  - **Left**, **Centre**, **Right** and **Justify**. Justify spreads every line of a box but the last to both edges.
  - **Box width**: 0 is point text. Above 0, the words wrap.
  - **Starts at X / Y**.
- **Appearance**:
  - **Fill**: the colour of the letters.
  - **Stroke**: a line round the letters, with its colour and width.
  - **Background**: a box behind the words, with its colour, opacity, padding and corner roundness.
  - **Shadow**: with its colour, opacity, angle, distance and softness.
  - Each of these three has a tick box to switch it on, and its settings appear below it while it is on.

Every change is one step for Ctrl+Z, and all of it saves with the project.

## Pictures (`D-264 pictures/`)

These are photographs of the running window, on the reference shot, at Draft.

- `typing.png`: a box dragged with the Text tool from 10% to 60% across, then words typed into it.
  - The words wrap inside the box as you type, in the program's own font at 120 pixels.
  - The dashed typing box sits over them.
- `styled.png`: the same layer set as follows:
  - size 64, tracking 40 and leading 80;
  - Stroke, Background and Shadow ticked, giving a black 4-pixel stroke, a dark-blue rounded box at 80%, and a soft shadow down and to the right;
  - yellow fill, Justify and B.
  - The lines of the box run edge to edge, except the last. Effect controls shows all four groups, with B and Justify lit.
- `font.png`: the Font family changed to Arial.
  - Its style list shows Arial's nine styles (Black, Bold, … Regular), and Regular was chosen.
  - 351 font files were found, in 190 families.
- `editing.png`: one click with the Text tool on the words.
  - No new layer was made. The typing box opened over the words in Arial, with all the words selected, and the blue selection lies on the drawn letters.

## The check, written first

`tests/d264_text_styles.rs` was committed first, in f6546de. **On the build before the change it did not build**, because there were no text styles. **All 57 checks now pass**, in `D-264_text_styles_table.md`.

| Check | Result |
|---|---|
| Tracking, the font's own kerning, leading, all caps, faux bold and faux italic each move or change the letters by what they say | pass |
| A box the words wrap in: Japanese breaks between any two characters, a long word is broken; left, centre, right and justified lines | pass |
| A stroke, a rounded background box and a soft shadow, drawn in that order under the fill and inside the words' box | pass |
| The new settings save and read back exactly, are written only when used, and keep lines no build writes yet; values out of range are refused | pass |
| Three D-263 pictures are byte for byte as on 2edc62d (an unstyled layer looks as it did) | pass |
| Fonts are listed by family and style | pass |
| A styled layer reaches exported frame 10 | pass |
| Every fixture project saves byte for byte as on e395d8b | pass |

Two of the check's own expected wordings were wrong, and I corrected them after it was committed. In both cases the mistake was in the check, not in the build:

- The justify sample sentence was changed to "the quick brown fox jumps over the lazy dog", because the first sample's first line already filled the box, so justify had nothing to spread.
- The bundled font's family name is now the one the font file itself gives, "Rounded Mplus 1c".

## In the running app

| Step | What happened |
|---|---|
| The T button | The tool is Text. The status line says "click where the words should start, drag a box for words that wrap, or click words to type in them." |
| A real mouse drag from 10% to 60% across | 4 layers became 5. "Text 1" was selected with a box 960 wide starting at 192, 216, and the tool went back to Select. The typing box was open, focused and 960 wide |
| Words typed into the typing box | They wrapped in the box in the picture as they were typed |
| Escape | The typing box closed. The words were saved as one step ("Set the text") |
| Size, tracking, leading; Stroke, Background, Shadow ticked; their settings; Justify; B | All saved on the layer, as listed in `styled.png` above. Groups shown: Source text, Character, Paragraph, Appearance, Transform, Blend |
| Font family → Arial | The layer's font became `arial.ttf`. The style list showed Arial's nine styles |
| A real click with the Text tool on the words | Still 5 layers. The typing box opened with the words, set in Arial at 64 pixels |
| Ctrl+Z ten times | The settings were taken back one at a time, then the layer itself; back to 4 layers |
| Errors on the page | none |

The drag and click were real mouse input sent to the window. The window's own settings were put back afterwards. No project file was saved.

## Faults found and fixed on the way

- **The words' outline broke masks and free transform at first.** Both had assumed a layer's outline is its whole picture. The outline now also carries where the words' box sits in the layer, and the code that converts between screen and layer uses it.
- **Empty words have no width**, so their outline would have been a line nobody could click. A text layer with no words is still outlined and picked by its whole frame.
- **At Draft, the words' outline fell back to the whole frame.** I had shrunk the box to a quarter for Draft, but a text layer's picture stays full size at Draft, and only its placement shrinks. This was found in the window photographs and fixed. The click on the words in `editing.png` was taken after the fix.

## Checks

- All 88 of the app's tests pass. The page and route lists (B-12b) gain the two new window routes: `/fonts` lists the fonts, and `/font` gives one font to the typing box. Neither changes the project. The keyboard list (B-12c) was rewritten to match.
- The whole core suite passes: 203 of 203 test groups ok, none failed.
- No fixture, existing saved project byte, or exported picture changes. A text layer that uses none of the new settings saves and draws exactly as before.

## Limits, stated

- **One style per layer.** No bold or colour on single words.
- **Text settings are not animated.** They have no keys. Animate the layer's Transform, or use effects.
- **No Japanese line-breaking rules (kinsoku).** A box breaks Japanese between any two characters, so a 。 can start a line.
- **No shaping or ligatures.** Joined scripts such as Arabic come out unjoined. Kerning pairs are applied.
- **No text on a path**, and no vertical text.
- **The typing box is placed when it opens and as it grows.** If you zoom or scroll while typing, it stays where it was until you type again.
- **The caret's place is close, not exact.** The page lays out its own copy of the words in the same font to place the caret. Faux bold, faux italic and justify can put it a little off the drawn letters.
- **While typing on the picture, the Source text box in Effect controls shows the old words until you finish.**

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click the T button and drag a box across the picture | A text layer "Text" in a box as wide as your drag, with the typing box open over it | |
| 2 | Type a long sentence | It wraps inside the box as you type, in the picture | |
| 3 | Press Escape, then Ctrl+Z | Typing finishes; one Ctrl+Z takes back the whole typing | |
| 4 | With the T tool, click on the words | No new layer; the typing box opens over the words | |
| 5 | With Select, double-click the words | Same as 4 | |
| 6 | In Effect controls, change Font to another family, then another style | The words redraw in it | |
| 7 | Drag Tracking up, then Leading up | Letters spread apart; lines move apart | |
| 8 | Press B, I and TT | Thicker, slanted, all capitals | |
| 9 | Press Justify on a box of several lines | Every line but the last reaches both edges | |
| 10 | Tick Stroke, Background and Shadow and change their numbers | A line round the letters, a box behind, a shadow, each following its settings | |
| 11 | Save, close and open the project | Everything is as you left it | |
| 12 | Export a frame | The styled words are in it | |
