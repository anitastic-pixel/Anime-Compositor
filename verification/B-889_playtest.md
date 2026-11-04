# B-889 playtest: Color Grade's section switches and white balance eyedropper (D-1009)

What it is: in the Lumetri tutorial (1:05) each section is checked by switching it off and on,
and the white balance is set by clicking something white with the WB Selector eyedropper. Color
Grade had neither. Now each of its six sections has an On/Off switch, and a WB Selector button
sits just above Temperature.

## Steps

1. Open a project with a picture, such as a street or a face, and select its layer.
2. Effect > Color Grade. Set Temperature to 40, Contrast to 30 and Vignette Amount to -2.5.
3. In Effect Controls, set **Vignette** (near the bottom, with the other choices) to Off, then On.
4. Set **Basic Correction** to Off, then On.
5. Set all six switches (Basic Correction, Creative, Curves, Color Wheels, HSL Secondary,
   Vignette) to Off.
6. Turn them all back On. Click **WB Selector**, then click something in the picture that should
   be white or grey (a wall, a shirt, a cloud).
7. Click WB Selector again and click the same spot.
8. Save the project, close it and open it again.

## What you should see

- Step 3: the dark corners go away, then come back. The Vignette Amount number stays -2.5.
- Step 4: the warmth and the extra contrast go away, then come back. The numbers stay as typed.
- Step 5: the picture looks exactly as it does with no effect.
- Step 6: Temperature and Tint change by themselves, and the spot you clicked turns white or grey.
  A warm (orange) spot lowers Temperature; a blue spot raises it.
- Step 7: the numbers change only a little the second time, because the spot is already grey.
- Step 8: each switch is as you left it.

## Pictures

- `verification/D-1009 pictures/`: the street with a warm grade and dark corners
  (`1_all_on.png`), the same with Vignette off (`2_vignette_off.png`) and with Basic Correction
  off (`3_basic_off.png`), and every section off (`4_all_off.png`, the street as it is,
  `town.png`).
- `verification/B-889 pictures/wb_panel.png`: the panel in a private copy of the app after the
  WB Selector picked a warm white (#f5dec4); Temperature went from 20 to -53.

## Fixtures

- `verification/D-1009_color_grade_6_table.md`: every row "yes". FX-GRADE6-001 to 011, the file
  read and saved, the commands, the card against the processor and the pictures.
- `verification/B-889_white_balance_page_table.md`: 11 of 11. WBPICK-001 to 008 run through the
  page's own code, the six switches present, a real pick, and Vignette switched off.
- FX-GRADE- to FX-GRADE5- still pass (D-397, D-398, D-571, D-637 and D-1007 tables).
