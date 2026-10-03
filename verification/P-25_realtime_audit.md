# P-25: Realtime audit — how fast a change shows while you drag

You asked:

> "now let's do a full performance/optimization audit focused goal towards realtime latency for any value/data manipulation, whether effects, transform, mask, etc. go through multiple planned passes with specified focuses, a systematic deep analysis and execution. do a sprawl, wiring, security audit as well."

and then left the rest to my judgement. This is what was measured, what changed, and what is left for you to decide.

## The short version

| What you drag | Delay before (73d9662) | Delay now | Pictures a second, before → now |
|---|---|---|---|
| Position | 25 ms | 16 ms | 22.5 → 23.7 |
| Scale | 137 ms | 18 ms | 12.8 → 22.6 |
| Rotation | 110 ms | 19 ms | 12.8 → 22.7 |
| Opacity | 128 ms | 10 ms | 12.7 → 22.8 |
| Gaussian Blur radius | 153 ms | 22 ms | 10.9 → 21.2 |
| Mask feather | 119 ms | 43 ms | 17.0 → 20.3 |
| A mask point | 121 ms | 52 ms | 14.4 → 19.7 |
| Text size | 41 ms | 32 ms | 19.0 → 20.7 |
| Blur radius at Full quality | 492 ms | 23 ms | 3.2 → 20.1 |
| Rotation of a blurred layer at Full quality | 217 ms | 20 ms | 6.5 → 22.2 |

The **delay** is the time from moving the hand to seeing the result, the middle of all the moves in one drag. The test drag moves about 22 to 23 times a second, so about 22 pictures a second is the most it can show. Every drag now shows almost all of them.

The pictures themselves are unchanged. The mask change is checked bit for bit (below), and the rest of the changes are about when and how often pictures are asked for, not how they are drawn.

## How it was measured

- **Machine:** AMD Ryzen 9 9900X (24 threads), RTX 4070 Ti SUPER on Vulkan, Windows 11.
- **Builds:** release builds of 73d9662 (before) and of this change (after), each started as a separate copy beside yours with its own browser profile. Your copy was not touched.
- **Project:** the reference shot, Draft quality and the picture drawn in the window, unless the row says Full. The Full rows switch refine-on-stop off, set Full, and put a Gaussian Blur on layer2.
- **The hand:** real mouse drags of 61 points sent to the window, about 2.7 seconds each. Each row is one drag.
- The steps are in the scratch folder as `p25/base.js`. Every row's numbers (requests, frame and outline times, settle time) were read from the page's own request log.

Full table, delay ms / pictures a second / frame ms / outline ms:

| Drag | Before (73d9662) | After step 1 (page) | Now (page + mask + frame route) |
|---|---|---|---|
| position | 25 / 22.5 / 16 / 4 | 21 / 22.1 / 14 / 2 | 16 / 23.7 / 13 / 2 |
| scale | 137 / 12.8 / 78 / 68 | 24 / 21.8 / 16 / 2 | 18 / 22.6 / 15 / 2 |
| rotation | 110 / 12.8 / 76 / 38 | 25 / 21.4 / 17 / 2 | 19 / 22.7 / 15 / 2 |
| opacity | 128 / 12.7 / 78 / 24 | 24 / 21.3 / 17 / 2 | 10 / 22.8 / 2 / 2 |
| Gaussian radius | 153 / 10.9 / 89 / 56 | 31 / 20.7 / 18 / 17 | 22 / 21.2 / 16 / 16 |
| mask feather | 119 / 17.0 / 53 / 55 | 102 / 16.9 / 54 / 55 | 43 / 20.3 / 35 / 13 |
| mask point | 121 / 14.4 / 63 / 65 | 132 / 15.0 / 60 / 62 | 52 / 19.7 / 39 / 2 |
| text size | 41 / 19.0 / 29 / 17 | 39 / 20.8 / 30 / 15 | 32 / 20.7 / 20 / 15 |
| Full: blur radius | 492 / 3.2 / 281 / 274 | 52 / 18.2 / 36 / 23 | 23 / 20.1 / 16 / 21 |
| Full: rotation (blurred) | 217 / 6.5 / 131 / 98 | 36 / 21.2 / 29 / 15 | 20 / 22.2 / 15 / 2 |

A second run on the final build, which also has the section 5 and 6 changes, gave these delays in the same order: 15, 7, 16, 18, 24, 43, 55, 31, 21 and 18 ms, at 20 to 25 pictures a second. That shows how much one run differs from the next: a few milliseconds either way. There were no errors on the page in either run.

## What was slow, and what changed

### 1. The panel numbers were asking for far more than the picture (page)

Dragging a number in Effect controls or the timeline (Scale, Rotation, Opacity, an effect setting) did not use the drag path the picture drag uses. Every move was sent as a separate change, and every change made the page ask again for:

- the whole project, about 60 times per drag;
- the composition's thumbnail, about 30 times;
- the timeline sheet, about 57 times.

All of those queue behind the picture on one line, so each picture waited for them. Now the numbers go through the same drag path as dragging on the picture:

- only the newest value is sent, one at a time;
- one drag is still one step of undo;
- the project, the thumbnail and the sheet are asked for once, after you let go.

Three other page tasks that checked things during a drag now wait until it ends:

- whether a picture is ready;
- the expression layer list;
- whether a file has gone missing.

### 2. Feathered masks are worked out about twice as fast (core)

A feathered mask is a blur of the mask's shape. The blur now skips the parts of each row and column where every pixel is the same, such as fully inside or fully outside the mask, and fills them in directly. The numbers it fills in are exactly the ones it used to add up, in the same order, so the result is the same to the last bit.

| Mask | Before | After |
|---|---|---|
| 1080p, feather 60 | 38.9 ms | 17.7 ms |
| 1080p, feather 200 | 157 ms | 77 ms |
| 4K, feather 60 | 142 ms | 58 ms |
| 4K, feather 200 | 515 ms | 183 ms |

Feather 0 has no blur and is unchanged: about 7 ms at 1080p and 25 to 31 ms at 4K.

**Same pixels:** 288 masks at 800 × 450 were fingerprinted before and after: four shapes (one reaching off the edge, one touching it), feathers 0, 0.5, 3, 17.3, 60 and 250, expansion 0, −12.5 and 20, plain and inverted at 60% opacity, alone and with a second feathered mask subtracting. All 288 fingerprints are identical.

### 3. The layer outlines no longer draw masks (command layer)

The outlines on the picture (the boxes round each layer) worked out each masked layer's mask on every move, only to throw it away: a mask never changes the size of a layer's box. They now leave plain masks out. This is why the outline time for a mask point drag fell from 62 ms to 2 ms.

### 4. A drag's pictures are not kept (command layer)

While you drag, each picture is of a state that lasts one move of the hand. Before, each one was:

- looked for in the frame memory;
- copied back from the graphics card;
- stored there.

None of that can be used again. Now a drag's pictures are only painted. The one after you let go is kept as before. This is what took Opacity from 17 ms a picture to 2 ms.

### 5. More controls change the picture during the drag (page)

Found by the wiring pass: some controls still changed only when you let go. These now change the picture while you drag:

- **Solids:** colour, width and height.
- **Shapes:** fill and stroke colour, mitre limit, and each gradient stop's colour, position and opacity.

Each drag or colour pick is still one step of undo.

**In the window:**
- A 60-step drag of a new solid's width sent 60 changes and drew 60 pictures while the hand was down. It went from 1920 to 1740, and it was one step of undo.
- Ten moves of the solid's colour picker drew ten pictures and added one more step.

Exposure spans are the one control left changing on release. While a span is dragged its own start frame moves, and the command names the span by that start, so making it live needs a change to how the command finds the span. That is proposed as D-269 below.

### 6. Effect settings held inside their ranges (page)

D-266 held each effect setting inside the range its description gives. Four descriptions gave their range in other words, so a drag could still pass the end and be refused:

- Brightness & Contrast's Contrast, −100 to 100;
- Vibrance's Saturation, −100 to 100;
- Vignette's Roundness, 0 to 100;
- Ripple's Fade, 0 to 100000.

Their descriptions now state the range, so those drags stop at the end too.

## Security pass

The window's command routes, the files it reads and writes, and the expression language were read for anything a crafted project file or web page could misuse. Fixed in this change:

| | What could happen | Now |
|---|---|---|
| M1 | A project file with an enormous composition size (say 100000 × 100000) asked for a picture buffer too big for memory. That ended the program instead of saying why. | A file is held to the same limits a new composition is: no side past 16384, at most 67,108,864 pixels, at most 10000 frames. It is refused with a message that names the size. |
| L2 | A font name in a project could name a path, and the text layer would read that file as a font. | A font name has to be a font name. The existing rule for typed names now applies to saved ones too. |
| L3 | The window's Open route took any path from the page. | It only opens a file on the recent list. Anything else goes through the Open button. |
| L4 | An expression of a few thousand brackets or minus signs went deeper than the program's stack and ended it. | Nesting is limited to 64 levels, with a message saying so. |
| L5 | Check Package split a manifest's paths only on `/`, so a path written with `\` could reach a network share or another drive. | It splits on both, and refuses drive letters and `:`. |
| L6 | A PNG whose header claimed, say, 20000 × 20000 asked for its whole buffer before reading a pixel. | A drawing past 16384 × 16384 is refused with a message naming its size. That is four times what a composition may be, so a long pan still opens. |

I checked that no project file in the fixtures, verification or tests folders is past the M1 limits.

Proposed rather than changed, because each needs your decision (D-268):

- **M2, network paths in a project.** A project that names a media file as `\\server\share\...` makes Windows contact that server when the file is read, and Windows sends your sign-in name and a scrambled form of your password to prove who you are. A stranger's project could use this. The choice is between refusing such paths, asking before opening them, or leaving them, since a real network folder works the same way.
- **L1, Collect Files reads any path a project lists.** A crafted project could list a private file as a drawing, and Collect Files would copy it into the package folder you then share. The choice is between copying only image, video and audio files, or showing the list of files before copying.

## Wiring pass

Every route the page asks for exists, every command the window knows is reachable or deliberately internal, the drag paths are consistent, all 94 effect settings match between the page and the core, and a project saves and reads back unchanged. Fixed: the four range descriptions and the release-only controls above. Left as they are, and noted for later:

- `camera.set_property` and `layer.set_depth` are commands only the tests use. The camera and depth are changed through the property commands on the page.

## Sprawl pass

Removed, because nothing used them:

- an old page function for the layer commands (`onSelected`) and the window check that looked for it;
- a page style (`.rhint`);
- two functions in the core (`into_image`, `total_nanos`).

No leftover debugging, no to-do notes, no commented-out code and no unused libraries were found. Proposed rather than changed (D-270), because each is a tidy-up with no effect you could see:

- Twelve tracked files are rewritten every time the tests run (tables and pictures with times or dates in them). That is why the git status always shows changes you did not make. They could be written to a scratch folder, or their changing lines dropped.
- A few small helpers are written out several times in the page. For example, `el` is defined three times, and 58 hand-built `layer=` addresses could use the existing `whose()`.
- Each effect's code is spread across about ten places. A table per effect would put it in one.
- The repository is 323 MB, with 187 files stored more than once.

## Proposed for later speed (D-269)

None of these were changed. Each either changes what the program draws or is a larger piece of work:

- **Masks at Draft resolution.** Masks are worked out at the drawing's full size even in Draft. Working them out at Draft size would cover a quarter of the pixels, so it should be several times cheaper (not measured), but Draft pictures would no longer match the current ones pixel for pixel.
- **A second line for requests.** The window answers one request at a time, so an outline request waits behind a picture. Answering them side by side would cut the delay further, but it changes how the page and the window talk.
- **The outlines for card effects and text.** The outlines still run a layer's effects and draw its text to learn its size. A cheaper size rule for each would bring text and effect drags down to the transform drags' 2 ms outline time.
- **Each drag move copies the project.** Not measured here; the cost grows with the size of the project.
- **Exposure spans live**, as in section 5.

## Checks

- The app's 88 checks pass.
- The core suite: all 224 checks in its 203 test groups pass, and 47 measurements that are only run on purpose were skipped. The four new checks below were added after that run, and pass on their own along with the rest of the core's unit checks (14 of 14).
- New checks:
  - a project past the size or length limits is refused, and one just inside them opens;
  - an expression nested 2000 deep is refused, and 60 deep still reads;
  - a package path with `\\host`, `..`, a drive letter or a `:` is refused, and `media/a.png` is checked;
  - a 20000 × 20000 drawing is refused, with its size in the message.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Drag a layer's Scale or Rotation number in Effect controls | The picture follows your hand smoothly, with no stutter | |
| 2 | Put a Gaussian Blur on a layer and drag its radius, first at Draft, then at Full with refine-on-stop off | The blur follows your hand in both | |
| 3 | Draw a mask, set its feather to about 100, then drag the feather number and then a mask point | Both follow your hand much more closely than before | |
| 4 | Press Ctrl+Z after any of these | The whole drag is undone in one step | |
| 5 | Make a solid, then drag its Width and pick a new colour | The solid changes size and colour while you drag and pick | |
| 6 | Make a shape with a gradient fill, then drag a stop's position | The gradient moves while you drag | |
| 7 | Drag Brightness & Contrast's Contrast far to the right | It stops at 100 and does not jump back | |
| 8 | Open a project you used before from the recent list | It opens as always | |
