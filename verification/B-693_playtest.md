# B-693 playtest: Drop Shadow and Rim Light are faster, and look the same

## What changed for you

Drop Shadow and Rim Light take less than half the time they did on the processor: one Drop
Shadow on a full HD layer about 12 ms instead of 25, one Rim Light about 8 instead of 17. On the
title from tutorial 4 with seven Drop Shadows, a 32 bpc frame now takes about 0.3 s instead of
0.9 to 1.5 s, and an 8 bpc frame about 0.2 s instead of 0.3 to 0.5 s. Every picture is the same
as before, to the last bit.

## What to look at

`verification/B-693_covering_once_table.md`: before and after times, the machine, the shots with
no shadow (unchanged), and the check that 504 shadows and rims came out identical.

## Try it

1. Open a project with a Drop Shadow on a text or picture layer (or the tutorial 4 title with its
   seven shadows) and play it, or scrub it.
2. It looks exactly as it did, and plays more smoothly.
3. Do the same with a Rim Light.
