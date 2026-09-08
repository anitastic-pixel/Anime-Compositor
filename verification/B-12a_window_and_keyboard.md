# B-12a, the editing window: what it looks like, and using it without a mouse

The eight tables under `B-12a_*_table.md` say that every command the editing panels invoke does
what it claims. None of them can say that the panels are on the screen, laid out, and operable.
That is what these four photographs are for, and Q-03 — display scaling and keyboard
reachability — is the requirement they answer for this window. One keyboard defect was found
here and is fixed.

## 1. The five panels

![the editing window](B-12a_window.png)

The reference shot, no project opened, nothing selected. Document 24's five areas are all
present at once, which is the claim:

| Area | Where | What it shows here |
| --- | --- | --- |
| the media bin | left | layer1 to layer4 with their frame counts, **Import drawings…**, **Relink drawings…** |
| the viewer | centre | frame 0 of the reference shot, composited |
| the inspectors | right | LAYER, EXPOSURES, EFFECTS, each saying "No layer is selected." |
| the transport | below the viewer | Play, the two step arrows, the frame number, the draft warning, Full resolution, Alpha only, Hide grid |
| the layer list and the document bar | bottom | layer4 down to layer1 with their eye and lock buttons, then Add layer through Export… and the keyboard hint |

Two things in it are worth reading rather than glancing at. **Relink drawings… is greyed out**,
because nothing in the bin is selected and W-02 relinks a chosen sequence, not whatever was last
touched. And the inspectors say *No layer is selected* rather than showing empty fields, so an
empty box is never mistaken for a value of zero.

## 2. The keyboard alone

![layer4 chosen by keyboard](B-12a_keyboard.png)

Thirteen presses of Tab and then a space, no mouse at any point. The thirteenth stop is layer4
in the layer list, the space chooses it, and the LAYER panel fills in on the right: Name,
Identifier, Drawing, Frames, Anchor, Position, Scale, Rotation, Opacity, Blend, Mask, Matte. The
transform inspector being populated is the evidence — it is only ever populated for a selected
layer.

```
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name B-12a_keyboard -Keys "<13 tabs> "
```

**The defect this found, and the fix.** The rows in the layer list and the media bin were
reachable by Tab and showed a focus ring, so they looked keyboard-operable. They were not: a row
is chosen by clicking it, and nothing answered a key, so a person navigating by keyboard could
walk onto a layer and then have no way to select it. Reachable is not the same as usable. Both
lists are built by one `row()` function in `app/ui/index.html`, so one handler there fixes both:
Enter and Space now do what the mouse does. B-11's rule that a focused control keeps the keys it
is given is what makes the space bar select the row instead of starting playback.

## 3. Display scaling

| Photograph | Scale | What holds |
| --- | --- | --- |
| ![100%](B-12a_scale_100.png) | 100% | the five panels side by side, the document bar on one row |
| ![200%](B-12a_scale_200.png) | 200% | the same five panels; the inspector column and the lower bar scroll, and the picture shrinks rather than being cut |

Same shot, same frame, in both. What to check going from one to the other: every panel is still
there, nothing overlaps anything, and the composited picture is whole. At 200% the right-hand
column is taller than the room it has and grows a scroll bar — EFFECTS is half visible at the
bottom edge of it — and the document bar wraps to two rows. That is the intended behaviour and
not a defect: the fix made for B-11 was that the bars keep at most half the window and scroll
the rest, so the thing being composited is never what gets pushed off.

```
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name B-12a_scale_100 -Scale 1.0
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name B-12a_scale_200 -Scale 2.0
```

As B-11 records, `-Scale` is not the same as changing the Windows display setting — it renders
the interface at that factor inside a window the operating system still sizes at this machine's
150%, so 200% here is a harsher test than a real 200% display, and the title bar is unaffected
because nothing in this repository draws it.

## What these four do not cover

- **The Open, Save As and Import dialogs.** Windows draws them and the capture script has no
  hands to answer one. The whole relink and import flow is therefore photographed only up to the
  point the dialog opens; what happens after it is in `B-12a_media_table.md` and
  `B-12a_relink_table.md`, driven by the same command the dialog would have invoked.
- **`app/ui/index.html` itself.** No test exercises the page — the tables reach the commands
  directly. These photographs are the only check that the page invokes them, and they cover one
  path through it. B-12, the owner's acceptance run of W-01 and W-02, is what covers the rest.
- **Screen readers and high-contrast mode.** Nothing here has been checked against either, and no
  requirement asks for it yet.
- **A real change of the Windows display setting**, which needs a sign-out and a person.

Captured at 1522×1016 physical pixels, the size a 1000×640 window has on this display running at
150%. Like every photograph under `verification/`, these are made by `tools/capture_window.ps1`
and not by `cargo test`, so no test regenerates them and nothing can disagree with them; window
placement, the desktop behind the window and the title bar theme belong to the machine and the
moment.
