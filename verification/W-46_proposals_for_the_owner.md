# What is left of the Sandbox: eleven proposals and five questions

Screens W-39 to W-45 built every part of the Sandbox that the page could build alone. What is left needs the engine to change, so each part is its own proposed decision. **Nothing below is built.** For each one you accept, the check is written first, run against today's build to show it fails, and only then is the change made. Each will end with the same pictures, click-through and playtest as the screens did.

Every proposal keeps the exported pictures byte for byte the same. None of them changes an existing command or shortcut.

**How to answer.** Something like "D-251 yes, D-252 yes, … D-261 (a)", plus your answers to the five questions at the end. "All as recommended" is also fine.

---

## D-251: a render queue of several items

**What you'd see.**
- Ctrl+M and Composition › Add to Render queue put the open composition in the queue as a new row. You choose its folder when you add it.
- Each row has a tick box and its own format.
- ▷ Render writes the ticked rows one after another.
- Rows can be dragged into another order, and a row can be removed with its ×.

**What changes underneath.**
- The window keeps a list of jobs, each a copy of the project taken when the row was added. This is the same copy one export takes today.
- The list lives while the window is open. It is **not saved in the project.**

**The check, first.**
1. Three rows of the reference shot: PNG, GIF and MP4, into three folders.
2. Each folder must hold exactly what one Export of that format writes today, byte for byte.
3. One more row is unticked, and must write nothing.
4. Stop during row 2 must leave row 1 whole and row 3 not started.

**Limits.**
- An edit after a row is added does not change that row. You see the warning "added at 14:32, before your last edits" and a **Refresh** button.
- Closing the window empties the queue.

**Recommended: yes.**

## D-252: watch it write, and Pause

**What you'd see.**
- While a render runs, the panel shows the frame just written, large.
- A strip of the frames around it.
- A bar with the frames whose drawing was missing marked in orange.
- **Pause**, which waits after the current frame. Press it again to go on.

**What changes underneath.**
- The export keeps a small copy of the last frame it wrote, and the page can ask for it.
- Pause is a second switch beside the existing Cancel.

**The check, first.**
1. Paused at frame 40, the folder must stop growing for five seconds.
2. After going on, the folder must be byte for byte what an unpaused run writes.
3. The picture shown must match the written frame 40, shrunk.

**Limits.**
- Frames are drawn several at a time, so Pause can take a few frames to settle.
- "Keep animating while it renders" and the popup or own-window watcher are **not** part of this. They are question 4.

**Recommended: yes, after D-251.**

## D-253: a cut status

**What you'd see.** A chip on each composition reading **Not started**, **In progress**, **Check**, **Retake** or **Done**. Click it to change it. The Project panel's By cut view shows it.

**What changes underneath.** One new optional line in the saved project, beside episode, scene, cut and animator. A project without it reads as Not started.

**The check, first.**
1. A project with a status saves it and reads it back.
2. Today's projects open and save unchanged, byte for byte.
3. A project with a status keeps it when opened and saved again by today's build, which keeps lines it does not know.

**Recommended: yes.**

## D-254: label colours on compositions and footage

**What you'd see.** The eight layer label colours, also on compositions and drawings in the Project panel. Right-click › Label.

**What changes underneath.** One new optional number on each composition and each footage item, saved the same way as a layer's label.

**The check, first.** The same three round-trip rows as D-253.

**Recommended: yes.**

## D-255: the region box

**What you'd see.** A button on the viewer's bar. Drag a box on the picture, and only that box is drawn. Everything outside is grey. Click the button again to draw the whole picture.

**What changes underneath.**
- **Nothing for still frames.** The engine can already draw part of a picture, which is how zooming in got faster (B-158). The box just tells it which part.
- During play the whole picture is drawn, as today.

**The check, first.** For three boxes on the reference shot, the drawn part must match the same pixels of the whole frame exactly.

**Limits.**
- It doesn't speed up play. Making it do so would be its own change to the frame memory.
- It is off while the view is turned.

**Recommended: yes.** This one is page-only.

## D-256: recovery copies of a project never saved

**What you'd see.** A project you never saved gets a recovery copy every two minutes, like a saved one. If the window closes without saving, the next start offers it on the recovery card.

**What changes underneath.** The copies go into the app's own folder, `%LOCALAPPDATA%`, because there is no project folder yet. Once you save, they go beside the project as today, and the old ones are removed.

**The check, first.**
1. A new project with two layers, never saved, writes a copy after two minutes.
2. A restart offers it.
3. Opening the copy gives the same project, byte for byte.

**Limits.** Five copies at most, the same as saved projects.

**Recommended: yes.** It protects work.

## D-257: deleted files in the health chip

**What you'd see.** The top bar's health chip also counts drawings whose file has gone from the disk since the project opened, for example "2 files gone". Clicking it lists them, each with Relink.

**What changes underneath.** A new request that changes nothing. It lists the project's files that are not on the disk. The page asks every few seconds, and after the window gets focus back.

**The check, first.**
1. Delete one drawing of a copied reference shot. The request must name it.
2. Put it back, and it must not.

**Limits.** A file replaced by a different picture under the same name is not "gone". The viewer already redraws it.

**Recommended: yes.**

## D-258: onion skin of one layer

**What you'd see.** An **Onion** button on the viewer. The chosen layer's drawing a few frames before is shown in red and a few after in green, faintly, over the picture. It never exports.

**What changes underneath.** The frame request gains "this layer alone", the same idea as solo but for one request, without changing what is on screen.

**The check, first.**
1. Layer2 alone at frame 10 must match, byte for byte, the picture with layer2 soloed.
2. The ordinary frame must be unchanged.

**Limits.** One layer at a time, one or two frames each side.

**Recommended: yes.**

## D-259: a middle picture quality, Half

**What you'd see.** Full, **Half** and Draft under the viewer's quality. Half draws at half size, and Draft stays at a quarter.

**What changes underneath.** A third level beside the two that exist. Full and Draft are untouched.

**The check, first.**
1. Full and Draft must stay byte for byte as today.
2. Half's frame 10 is recorded as a new expected picture.
3. Half must be at least as quick as Full on this machine. Measured, never assumed.

**Recommended: yes.**

## D-260: real pictures in the Project panel

**What you'd see.** The Pictures view shows each drawing's first picture and each composition's frame at its playhead, small, instead of the symbols.

**What changes underneath.** A new request for a small picture of any footage item or composition. It is drawn on the side and kept until that item changes.

**The check, first.**
1. A drawing's small picture must match its file, shrunk.
2. A composition's must match its full frame, shrunk.
3. Asking must not change what the viewer shows.

**Limits.** Sound shows the symbol ♪ as today.

**Recommended: yes.**

## D-261: Sketch (W-46)

D-248 let Sketch in narrowly, as notes drawn over the cut that are **never exported and never part of the rendered picture.** It comes last and hidden.

**What you'd see.** The Sandbox's Sketch board:
- a fourth workspace;
- brush, pencil and eraser, with three sizes and five colours, and Ctrl+Z for a stroke;
- sketch layers you can show or hide, each drawing on one frame or on the whole cut;
- the cut underneath at Off / 30 / 60 / 100%;
- a frame strip with a dot on frames that have their own sketch;
- onion skin of the sketches;
- **Clear**;
- "Also over the Compose viewer".

**What changes underneath.**
- The strokes are saved in the project, beside the composition, as lists of points.
- **The engine never reads them for a picture.** The page draws them over the viewer.

**The check, first.**
1. A project with two sketch layers saves and reads back exactly.
2. Exports of it are byte for byte the same as without the sketches.
3. Today's build keeps the sketches when it opens and saves the project.
4. A project without sketches is unchanged.

**Limits, stated.**
- Strokes are lines, not paint. There is no fill, no blending and no pressure.
- They are drawn at the screen's size, so a sketch looks thinner when zoomed out.

**Choices:**
- (a) build it as above;
- (b) build it without "Also over the Compose viewer";
- (c) leave it out.

**Recommended: (a).**

---

## Also waiting for you

- **D-250**, from W-41b: when the view is turned or mirrored, or a snapshot or compare is used, the window sends the page the picture's pixels. Built and checked. Recommended: accept.
- **Question 1: do workspaces keep their own panel changes?** For example, moving a panel in Animate wouldn't change Compose. Today D-85 stands, and Reset puts a workspace back. Recommended: yes, each workspace remembers its own.
- **Question 2: the Sheet's column order.** The Sandbox puts the layers first, then Action and Dialogue. The window keeps paper order, Frame and Action first, as D-84c and D-84g decided. Recommended: keep paper order.
- **Question 3: curves behind the front one.** They share its scale (B-19d), so a Rotation curve draws nearly flat under a Position curve. The Sandbox draws each at its own height. Recommended: own height for the curves behind, with the front curve's numbers on the side. This changes B-19d's page check only, not a picture.
- **Question 4: watching a render from Compose.** Should a running render's watcher open as a popup, or stay in the Render workspace? Recommended: stay in Render, with the top bar's task as the way there, as built in W-45.
- **Question 5: the text tool.** D-248 kept Ctrl+T as the free-transform box with no Text tool. That stands unless you ask for text layers, which would be a large new piece of work.
