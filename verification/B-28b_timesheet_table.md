# B-28b: a cut imported from its XDTS timesheet

D-84, accepted by the owner on 2026-09-24. Every expected value is `Fixtures/xdts/expected_xdts.json`, written by `tools/xdts_reference.py` before this code existed and printed in document 25 as FX-XDTS-001 to 040. For each cut the build's composition, its layers bottom first (name, track, timing, drawings and timesheet record) and its notes are compared with the reference's; the notes in any order. The timing is shown as document 25 prints it: a column's drawing on each frame, x for none.

## FX-XDTS-001 to 028 and 040, read whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-XDTS-001: One column on ones: a new drawing every frame. | 6 frames at 160 by 90, named c001<br>A: `1 2 3 4 5 6` | yes |
| FX-XDTS-002: On twos: each drawing written once and held for two frames. The sheet says nothing on the frames between, and a drawing holds until the next entry. | 8 frames at 160 by 90, named c002<br>A: `1 1 2 2 3 3 4 4` | yes |
| FX-XDTS-003: FX-XDTS-002 with SYMBOL_HYPHEN written on every held frame, as the specification writes a held line of dialogue. The same timing. | 8 frames at 160 by 90, named c003<br>A: `1 1 2 2 3 3 4 4` | yes |
| FX-XDTS-004: A long hold: the last drawing written holds to the end of the sheet. | 12 frames at 160 by 90, named c004<br>A: `1 1 1 2 2 2 2 2 2 2 2 2` | yes |
| FX-XDTS-005: Blank cells (SYMBOL_NULL_CELL, the X on a paper sheet): the column shows nothing from there until the next drawing. | 10 frames at 160 by 90, named c005<br>A: `1 1 1 x x 2 2 x x x` | yes |
| FX-XDTS-006: A column that starts late: nothing is shown before its first entry. | 8 frames at 160 by 90, named c006<br>A: `x x x x 1 1 1 1` | yes |
| FX-XDTS-007: Drawings reused out of order, as a cycle goes 1, 2, 3, 2, 1. | 10 frames at 160 by 90, named c007<br>A: `1 1 2 2 3 3 2 2 1 1` | yes |
| FX-XDTS-008: The same drawing written twice in a row is one exposure, not two. | 6 frames at 160 by 90, named c008<br>A: `1 1 1 1 2 2` | yes |
| FX-XDTS-009: Three columns, written in the file in the order 2, 0, 1: they stack by track number, 0 at the bottom, whatever the file order. A is on threes, B on twos, C on ones. | 6 frames at 160 by 90, named c009<br>C: `1 2 3 4 5 6`<br>B: `1 1 2 2 3 3`<br>A: `1 1 1 2 2 2` | yes |
| FX-XDTS-010: No folders: loose files beside the sheet named for their column, with each of the four separators D-84 accepts. | 2 frames at 160 by 90, named c010<br>D: `1 1`<br>C: `1 1`<br>B: `1 1`<br>A: `1 2` | yes |
| FX-XDTS-011: Both a folder and loose files for column A: the folder is used, and the loose files are named as not used. | 2 frames at 160 by 90, named c011<br>A: `1 2`<br>note `{"id":"TIMESHEET_NOT_USED","names":["A_0001.png","A_0002.png"]}` | yes |
| FX-XDTS-012: A column named in lower case finds a folder named in upper case. | 2 frames at 160 by 90, named c012<br>a: `1 1` | yes |
| FX-XDTS-013: The two tick marks change nothing that is shown: the drawing before each holds through it, and each is reported as a mark on its frames. | 4 frames at 160 by 90, named c013<br>A: `1 1 2 2`<br>note `{"column":"A","frames":[1],"id":"TIMESHEET_MARK","mark":"inbetween"}`<br>note `{"column":"A","frames":[3],"id":"TIMESHEET_MARK","mark":"reverse sheet"}` | yes |
| FX-XDTS-014: A sheet with a dialogue column and a camerawork column: all three are read, the two text columns named Dialogue and Camera, since the headers name only the cells. | 4 frames at 160 by 90, named c014<br>A: `1 1 1 1`<br>dialogue column Dialogue: `[[0,3,["MIKA","Wait!"]]]`<br>camera column Camera: `[[0,4,["PAN"]]]` | yes |
| FX-XDTS-015: Two timetables in one file: the first is read, the second is named as not read. | 2 frames at 160 by 90, named c015<br>A: `1 1`<br>note `{"id":"TIMESHEET_TABLE_NOT_READ","tables":["c015 retake"]}`<br>note `{"column":"A","drawings":[2],"id":"TIMESHEET_DRAWING_UNUSED"}` | yes |
| FX-XDTS-016: A version other than 5 is read anyway, and reported. | 2 frames at 160 by 90, named c016<br>A: `1 1`<br>note `{"id":"TIMESHEET_VERSION","version":4}` | yes |
| FX-XDTS-017: A sheet saved with a byte-order mark and Windows line endings reads as any other. | 2 frames at 160 by 90, named c017<br>A: `1 1` | yes |
| FX-XDTS-018: A timetable with no name: the composition is named after the sheet's file. | 2 frames at 160 by 90, named fx_xdts_018<br>A: `1 1` | yes |
| FX-XDTS-019: Text columns as a sheet can write them: a line held with hyphens, a line on one frame, a cross after it, a hyphen with nothing before it, a number where text belongs, an entry past the end, and a field this program does not know. | 8 frames at 160 by 90, named c019<br>A: `1 1 1 1 1 1 1 1`<br>dialogue column S1: `[[0,3,["MIKA","Wait!"]],[4,5,["KAI","No."]]]`<br>note `{"field":"7","id":"TIMESHEET_FIELD_NOT_READ","tracks":1}`<br>note `{"column":"S1","frames":[9],"id":"TIMESHEET_ENTRY_IGNORED","reason":"outside the sheet"}`<br>note `{"column":"S1","frames":[6],"id":"TIMESHEET_ENTRY_IGNORED","reason":"a continuation with nothing before it"}`<br>note `{"column":"S1","frames":[7],"id":"TIMESHEET_ENTRY_IGNORED","reason":"not text"}` | yes |
| FX-XDTS-020: The sheet calls for drawing 2, which is not in A's folder. The timing is kept, so those frames will say MEDIA_SEQUENCE_GAP when drawn, and the import names the drawing. | 6 frames at 160 by 90, named c020<br>A: `1 1 2 2 3 3`<br>note `{"column":"A","drawings":[2],"id":"TIMESHEET_DRAWING_MISSING"}` | yes |
| FX-XDTS-021: A's folder holds drawings 2 and 4, which the sheet never shows. | 4 frames at 160 by 90, named c021<br>A: `1 1 3 3`<br>note `{"column":"A","drawings":[2,4],"id":"TIMESHEET_DRAWING_UNUSED"}` | yes |
| FX-XDTS-022: Column B has no folder and no loose files: A becomes a layer and B does not, and the import says so. | 2 frames at 160 by 90, named c022<br>A: `1 1`<br>note `{"column":"B","id":"TIMESHEET_COLUMN_NO_DRAWINGS"}` | yes |
| FX-XDTS-023: A background folder and a background still beside the sheet match no column, and are named as not used. A text file is not a drawing and is not mentioned. | 2 frames at 160 by 90, named c023<br>A: `1 1`<br>note `{"id":"TIMESHEET_NOT_USED","names":["BG","BG.png"]}` | yes |
| FX-XDTS-024: A cell that is not a whole number (`3a`): the column is blank from there to its next entry, and the value and frame are reported. | 6 frames at 160 by 90, named c024<br>A: `1 1 x x 2 2`<br>note `{"column":"A","frame":2,"id":"TIMESHEET_CELL_UNREADABLE","value":"3a"}`<br>note `{"column":"A","drawings":[3],"id":"TIMESHEET_DRAWING_UNUSED"}` | yes |
| FX-XDTS-025: Entries the sheet cannot use: a second entry on frame 2, and one on frame 6 of a 4-frame sheet. Both are left out and reported. | 4 frames at 160 by 90, named c025<br>A: `1 1 2 2`<br>note `{"column":"A","frames":[6],"id":"TIMESHEET_ENTRY_IGNORED","reason":"outside the sheet"}`<br>note `{"column":"A","frames":[2],"id":"TIMESHEET_ENTRY_IGNORED","reason":"a second entry on the same frame"}`<br>note `{"column":"A","drawings":[3,4],"id":"TIMESHEET_DRAWING_UNUSED"}` | yes |
| FX-XDTS-026: Column B holds only blank cells: it makes no layer, and its folder is named as not used. | 2 frames at 160 by 90, named c026<br>A: `1 1`<br>note `{"column":"B","id":"TIMESHEET_COLUMN_EMPTY"}`<br>note `{"id":"TIMESHEET_NOT_USED","names":["B"]}` | yes |
| FX-XDTS-027: Track 1 has no name in the sheet, so no drawings can be found for it and it makes no layer. | 2 frames at 160 by 90, named c027<br>A: `1 1`<br>note `{"id":"TIMESHEET_COLUMN_UNNAMED","track":1}` | yes |
| FX-XDTS-028: Clip Studio Paint can write a column's first drawing before frame 0, which the specification does not allow. As OpenToonz reads it, the last entry before frame 0 is shown from frame 0 until the column's next entry, and is reported; A's earlier one is left out. B has its own entry on frame 0, so its entry before it is left out. | 4 frames at 160 by 90, named c028<br>B: `2 2 2 2`<br>A: `1 1 2 2`<br>note `{"column":"A","frame":-1,"id":"TIMESHEET_ENTRY_CARRIED_IN"}`<br>note `{"column":"A","frames":[-2],"id":"TIMESHEET_ENTRY_IGNORED","reason":"outside the sheet"}`<br>note `{"column":"B","frames":[-1],"id":"TIMESHEET_ENTRY_IGNORED","reason":"outside the sheet"}` | yes |
| FX-XDTS-040: The sample cut: two seconds, three columns, a line of dialogue, a camera instruction and a background. | 48 frames at 160 by 90, named s01 c012<br>C: `x x x x x x x x x x x x x x x x x x x x 1 2 3 4 5 5 6 6 x x x x x x x x x x x x 1 1 2 2 2 2 2 2`<br>B: `1 1 1 2 2 2 3 2 1 1 1 2 2 2 3 2 x x x x x x x x x x x x x x 1 1 1 2 2 2 3 3 3 1 1 1 1 1 1 1 1 1`<br>A: `1 1 2 2 3 3 4 4 5 5 6 6 7 7 8 8 8 8 8 8 8 8 8 8 7 7 6 6 5 5 4 4 3 3 2 2 1 1 1 1 1 1 1 1 1 1 1 1`<br>dialogue column Dialogue: `[[0,16,["MIKA","Over here!"]]]`<br>camera column Camera: `[[20,48,["FOLLOW"]]]`<br>note `{"column":"C","frames":[41],"id":"TIMESHEET_MARK","mark":"inbetween"}`<br>note `{"id":"TIMESHEET_NOT_USED","names":["BG.png"]}` | yes |

## FX-XDTS-030 to 036, nothing imported (D-84)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-XDTS-030: The folder holds no timesheet. | refused: `{"refused":"TIMESHEET_NOT_FOUND","sheets":[]}`<br>note `{"id":"TIMESHEET_NOT_FOUND","sheets":[]}` | yes |
| FX-XDTS-031: The folder holds two timesheets, and which one is meant is the person's to say. | refused: `{"refused":"TIMESHEET_NOT_FOUND","sheets":["fx_xdts_031a.xdts","fx_xdts_031b.xdts"]}`<br>note `{"id":"TIMESHEET_NOT_FOUND","sheets":["fx_xdts_031a.xdts","fx_xdts_031b.xdts"]}` | yes |
| FX-XDTS-032: The first line is not the XDTS line: this is bare JSON. | refused: `{"reason":"the first line is not exchangeDigitalTimeSheet Save Data","refused":"TIMESHEET_UNREADABLE"}`<br>note `{"id":"TIMESHEET_UNREADABLE","reason":"the first line is not exchangeDigitalTimeSheet Save Data"}` | yes |
| FX-XDTS-033: The first line is right and what follows is not JSON. | refused: `{"reason":"what follows the first line is not JSON","refused":"TIMESHEET_UNREADABLE"}`<br>note `{"id":"TIMESHEET_UNREADABLE","reason":"what follows the first line is not JSON"}` | yes |
| FX-XDTS-034: The sheet has a dialogue column and no cell column. | refused: `{"refused":"TIMESHEET_NO_CELLS"}`<br>note `{"id":"TIMESHEET_NO_CELLS"}` | yes |
| FX-XDTS-035: The sheet's length is 0 frames. | refused: `{"reason":"its length is not a number of frames","refused":"TIMESHEET_UNREADABLE"}`<br>note `{"id":"TIMESHEET_UNREADABLE","reason":"its length is not a number of frames"}` | yes |
| FX-XDTS-036: No column has any drawings beside the sheet, so no layer can be made. | refused: `{"refused":"TIMESHEET_NO_CELLS"}`<br>note `{"column":"A","id":"TIMESHEET_COLUMN_NO_DRAWINGS"}`<br>note `{"id":"TIMESHEET_NO_CELLS"}` | yes |

## What a person reads for each note (documents 25 and 28)

| Check | The build's answer | Matches |
| --- | --- | --- |
| every one of document 25's sixteen TIMESHEET IDs is said by some fixture | 16 of 16 said | yes |
| TIMESHEET_CELL_UNREADABLE, which document 25 makes WARNING | WARNING TIMESHEET_CELL_UNREADABLE: Column A, frame 2: the cell "3a" is not a drawing number, so the column is blank until its next entry. | yes |
| TIMESHEET_COLUMN_EMPTY, which document 25 makes INFO | INFO TIMESHEET_COLUMN_EMPTY: Column B never shows a drawing, so it made no layer. | yes |
| TIMESHEET_COLUMN_NO_DRAWINGS, which document 25 makes WARNING | WARNING TIMESHEET_COLUMN_NO_DRAWINGS: Column B has no folder and no loose drawings, so it made no layer. Put its drawings in a folder named for the column, beside the sheet. | yes |
| TIMESHEET_COLUMN_UNNAMED, which document 25 makes WARNING | WARNING TIMESHEET_COLUMN_UNNAMED: Column 1 has no name, so its drawings cannot be found; it made no layer. Name the column in the program that made the sheet. | yes |
| TIMESHEET_DRAWING_MISSING, which document 25 makes WARNING | WARNING TIMESHEET_DRAWING_MISSING: Column A: the sheet calls for drawings 2 that its folder does not have. The timing is kept; add the drawings and relink. | yes |
| TIMESHEET_DRAWING_UNUSED, which document 25 makes INFO | INFO TIMESHEET_DRAWING_UNUSED: Column A: drawings 2 are never shown by the sheet. | yes |
| TIMESHEET_ENTRY_CARRIED_IN, which document 25 makes INFO | INFO TIMESHEET_ENTRY_CARRIED_IN: Column A: the entry written on frame -1 is shown from frame 0. | yes |
| TIMESHEET_ENTRY_IGNORED, which document 25 makes WARNING | WARNING TIMESHEET_ENTRY_IGNORED: Column S1: the entries on frames 9 were left out (outside the sheet). | yes |
| TIMESHEET_FIELD_NOT_READ, which document 25 makes INFO | INFO TIMESHEET_FIELD_NOT_READ: The 7 field (1 columns) was not read; only drawing, dialogue and camera columns are. | yes |
| TIMESHEET_MARK, which document 25 makes INFO | INFO TIMESHEET_MARK: Column A: inbetween marks on frames 1; they change nothing shown. | yes |
| TIMESHEET_NOT_FOUND, which document 25 makes ERROR | ERROR TIMESHEET_NOT_FOUND: The chosen folder holds no .xdts timesheet. Choose a cut's folder holding exactly one .xdts file. | yes |
| TIMESHEET_NOT_USED, which document 25 makes INFO | INFO TIMESHEET_NOT_USED: Not used by any column: A_0001.png, A_0002.png. | yes |
| TIMESHEET_NO_CELLS, which document 25 makes ERROR | ERROR TIMESHEET_NO_CELLS: No drawing column of the timesheet could become a layer. The other notes say why for each column. | yes |
| TIMESHEET_TABLE_NOT_READ, which document 25 makes INFO | INFO TIMESHEET_TABLE_NOT_READ: Only the first timetable was read, not: c015 retake. | yes |
| TIMESHEET_UNREADABLE, which document 25 makes ERROR | ERROR TIMESHEET_UNREADABLE: The timesheet could not be read: the first line is not exchangeDigitalTimeSheet Save Data. Save it again as XDTS from the program that made it. | yes |
| TIMESHEET_VERSION, which document 25 makes WARNING | WARNING TIMESHEET_VERSION: The timesheet says version 4, not 5; it was read anyway. | yes |

## Into a project (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-XDTS-040 added to an empty project: three drawing sequences, and a composition of 48 frames at 24 a second with layers A, B and C, bottom first, each timed as read and carrying its timesheet record | applied; 3 assets; comp-1 "s01 c012" 48 frames at FrameRate { numerator: 24, denominator: 1 }; layers A on asset-1 (15 exposures, Some(("fx_xdts_040.xdts", "A", 0))), B on asset-2 (12 exposures, Some(("fx_xdts_040.xdts", "B", 1))), C on asset-3 (8 exposures, Some(("fx_xdts_040.xdts", "C", 2))) | yes |
| drawing paths are stored relative to the project's folder | A's drawing 1 at Some("fx_xdts_040/A/A_0001.png") | yes |
| and one undo takes the whole import away | the empty project | yes |
| importing it a second time counts new IDs past the first | comp-2 | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| an imported layer is written with its timesheet record | `"timesheet": {"column":"A","sheet":"fx_xdts_040.xdts","track":0}` | yes |
| and it opens again as the same project | equal | yes |
| the key is written once per imported layer, three times | 3 | yes |
| its dialogue and camera columns are written as `sheet_text` (D-84c) | `[{"entries":[{"end_frame_exclusive":16,"start_frame":0,"text":["MIKA","Over here!"]}],"kind":"dialogue","name":"Dialogue","track":0},{"entries":[{"end_frame_exclusive":48,"start_frame":20,"text":["FOLLOW"]}],"kind":"camera","name":"Camera","track":0}]` | yes |
| a column of a kind this build does not know is kept and saved back | kept | yes |
| a text entry that ends before it starts is refused: PROJECT_SCHEMA_INVALID | Some("PROJECT_SCHEMA_INVALID") | yes |
| a project with no text columns is written without the key | absent | yes |
| a record whose track is not a whole number is refused: PROJECT_SCHEMA_INVALID | Some("PROJECT_SCHEMA_INVALID") | yes |

## Result

65 of 65 checks pass.
