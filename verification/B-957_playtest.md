# B-957 playtest: Output templates (D-1077)

The tutorial (nTpT2PELCbA, about 0:40) sets up a lossless output, saves it with Make Template as
"test", and picks "test" again for the next render. This is a change to the window only, so there
is no picture: the check is the steps below and the table `verification/B-957_output_templates_table.md`.

## Steps

1. Open the Render workspace (or File > Export: the same choices show under it). Above the format
   list are **Template: none** and **Make template...**.
2. Choose **EXR, full float, lossless** and tick **Sound as a WAV beside it**.
3. Press **Make template...**, type `test`, press Enter. The list now says **Template: test**.
4. Change the format to **MP4** and set it to **High**. Make a template named `film`.
5. Pick **Template: test**. The format goes back to EXR full float, and the WAV tick is on.
   Pick **Template: film**: MP4, High.
6. With **test** chosen, press **Add to queue**. The new queue row writes EXR full float.
7. Close the window and open it again. Both templates are still in the list.
8. Make a template named `test` again on PNG. There is still one `test`, now PNG.

Wrong if:
- picking a template does not change the format list or its ticks;
- **Write frames whose drawing is missing** changes when you pick a template (it never should);
- the templates are gone after reopening the window.

Not the same as After Effects:
- A template cannot be deleted or renamed yet; make one of the same name to replace it.
- Only the Output's choices are kept, not render settings such as the frame range.
