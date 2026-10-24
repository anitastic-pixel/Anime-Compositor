# B-644 playtest: Curl Noise with Source This Layer is faster, and looks the same

## What changed for you

A Curl Noise set to Source This Layer (or Other Layer) plays faster: on the reference shot about
136 ms a frame instead of 215 (Softness 50: 148 instead of 226). Nothing you see changes: every
picture is the same as before, and the Input Noise source is untouched.

It is not yet back to the 84 ms it took before the look change of D-749; the table says why.

## What to look at

`verification/B-644_curl_speed_table.md`: before and after times, the machine, and the checks that
show no pixel moved.

## Try it

1. Open a project with a Curl Noise on a picture layer, set Source to This Layer.
2. Play it. It looks exactly as it did yesterday, and plays more smoothly.
