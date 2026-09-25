# B-29: workspaces, by hand

Built on 2026-09-24 against D-85, the workspaces you chose the same day ("Slot workspaces",
"Workspaces first").

No table can check this. The page decides where every panel goes, and no test here can run the
page. The tables that read the page's source were run again and still pass:
`verification/B-12c_keyboard_table.md` lists the new Workspace controls, and
`verification/B-28d_sheet_table.md` still gets 57 of 57, with the Sheet now a panel of its own.
This sheet is the check.

## Before you start

Open any project with a few layers. Importing `Fixtures/xdts/fx_xdts_040` with **Import cut...**
gives the Sheet something to show.

## What to check

1. **It opens as before.** The command strip has **Workspace: Standard**, **Save workspace...**
   and **Reset workspace** after Preferences. The window shows Project on the left, Composition
   in the middle, Effect controls on the right and the timeline at the bottom.
2. **The bottom has two panels.** Above the timeline is a row with two names, **Timeline** and
   **Sheet**. Timeline is lit. The timeline's own **Timeline** and **Graph** buttons are still
   under it, and Shift+F3 still swaps them.
3. **The Sheet in front, and the fault from your screenshot.** Click **Sheet** in that row. The
   sheet shows in the same space and the bottom stays the same height. The viewer does not
   shrink, and its zoom percentage does not change. The table scrolls inside the panel, and
   the frame/A/B/C heading stays at the top while it scrolls. Click **Timeline** to go back.
4. **The Timing workspace.** Choose **Workspace: Timing**. The Sheet becomes a tall column on the
   left, with Project behind it (the names row at the top of that column reads Sheet, Project).
   The bottom has only the timeline, and its names row is gone. Play with Space: the highlighted
   row runs down the column with the picture.
5. **Moving a panel by right-click.** Right-click the title bar of **Effect controls** (where
   it says "Effect controls"). A menu offers to move it to the left, the centre or the bottom;
   the right is greyed because it is already there. Choose **the bottom**. It moves to the
   bottom, in front of the timeline. The right column folds away and the viewer widens.
6. **Moving a panel by dragging.** Drag the **Effect controls** name in the bottom row, or its
   title bar, up onto the viewer. The viewer gets a blue outline while you hold it over. Let go:
   Effect controls is in the middle with Composition behind it. Click **Composition** in that
   row to bring the picture back.
7. **A folded place comes back through the menu.** The right column is still folded from
   step 5, so there is nothing there to drag onto. Right-click Effect controls' title bar and
   choose **the right**. The column comes back.
8. **Reset workspace.** Click **Reset workspace**. Timing comes back exactly as in step 4, with
   the Sheet column, Project behind it, and Effect controls on the right.
9. **Saving one.** Pull the border beside the Sheet column wider, and move Project to the
   bottom. Click **Save workspace...**, type `Mine`, and press Enter. The status line says it was
   saved, and the list now reads **Workspace: Mine**. Choose **Standard**, then **Mine** again.
   Your arrangement and the column's width come back.
10. **A built-in name is refused.** Save workspace..., type `Standard`, Enter. The status line
    says it is built in and cannot be saved over. Press Escape to put the box away.
11. **It is remembered.** Close the window and open it again. It opens in the workspace and
    arrangement you left it in. Choose **Standard** to finish.
12. **Filling the window still works.** Hold the pointer over the Sheet (anywhere it is) and
    press the ` key (left of 1). Its place fills the window. Press ` again to put it back.
13. **Nothing else moved.** In Standard, walk through a quick edit: select a layer, change
    Position in Effect controls, drag a bar on the timeline, undo. It all behaves as before.

## Known limits

- No floating windows, and a place cannot be split in two. Four places, each with a stack of
  panels.
- A saved workspace cannot be deleted yet. Saving under the same name replaces it.
- Choosing another workspace forgets unsaved changes to the one you were in. Save it first if
  you want to keep them. After Effects remembers them; this does not yet.
- Panels move with the mouse only. Choosing a workspace works from the keyboard (Tab to the
  list).
- Workspaces live on this computer, like the panel sizes, and not in the project file.

## What to answer

"works", or which step number did something else and what it did.
