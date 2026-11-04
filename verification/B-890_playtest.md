# B-890 playtest: Color Grade's HSL Secondary Sharpen (D-1010)

What it is: in the Lumetri tutorial (6:20) the HSL Secondary's correction has its own Sharpen
slider, so only the picked colours get sharper or softer. Color Grade's secondary had no sharpen.
Now **Secondary Sharpen** (-100 to 100) sits under Secondary Saturation.

## Steps

1. Open a project with a picture that has one clear colour, such as red brick or a blue sky,
   and select its layer.
2. Effect > Color Grade. In the HSL Secondary part, click **Set** and click the coloured area.
3. Set Show Key to Color/Gray to see what the key takes, then back to Off.
4. Set **Secondary Sharpen** to 100. Zoom in to 200% on an edge inside the picked colour, and on
   an edge outside it.
5. Set Secondary Sharpen to -100.
6. Set **HSL Secondary** (the section switch) to Off, then On.
7. Save the project, close it and open it again.

## What you should see

- Step 4: edges inside the picked colour get crisper; everything outside the key looks exactly
  as before.
- Step 5: the picked colour goes soft and slightly blurry; the rest stays sharp.
- Step 6: Off brings back the picture as it was; On brings the sharpen back.
- Step 7: Secondary Sharpen is still -100.

## Pictures

- `verification/D-1010 pictures/`: the street (`town.png`), the key on its warm buildings
  (`1_key.png`, white is taken), the buildings sharpened at 100 (`2_sharper.png`) and softened
  at -100 (`3_softer.png`). Outside the key both pictures match the street.

## Fixtures

- `verification/D-1010_color_grade_7_table.md`: every row "yes". FX-GRADE7-001 to 010, the file
  read and saved, the commands, the card against the processor and the pictures.
- FX-GRADE- to FX-GRADE6- still pass (D-397, D-398, D-571, D-637, D-1007 and D-1009 tables).
