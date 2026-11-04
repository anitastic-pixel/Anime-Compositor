# B-949 playtest: Text Animator, the Expression Selector (D-1069)

The tutorial (iOknL8pPZGg, 4:25 to 5:40) drops each letter in one after another and lets it
bounce, with a bounce expression on an Expression Selector's Amount.

## Steps

1. Make a text layer that says `Bounce` and add Text Animator to it. Set Position to 0, -200.
   Leave Start 0 and End 100, so the Range Selector picks every letter.
2. Set Expression Selector to **On**. Every letter jumps 200 pixels up (Expression Amount 100).
3. Set Expression Amount to 0. Every letter is back in its place.
4. Alt-click the diamond of Expression Amount and type `textIndex*10`. The letters climb in a
   staircase, the first 20 pixels up, the last 120.
5. Replace the expression with the tutorial's bounce:

   ```
   freq = 1; decay = 5; duration = 0.10; retard = textIndex*thisComp.frameDuration*1;
   t = time - (inPoint + retard); startVal = [100,100,100]; endVal = [0,0,0];
   if (t < duration) { linear(t, 0, duration, startVal, endVal) }
   else { amp = (endVal - startVal)/duration; w = freq*Math.PI*2;
   endVal + amp*(Math.sin((t - duration)*w)/Math.exp(decay*(t - duration))/w) }
   ```

6. Play from frame 0. The letters fall in one after another from the left, overshoot a little
   below their place, and settle within about a second and a half.
7. Replace the expression with `Math.sqrt(textIndex - 7)`, which gives not-a-number. The app
   reports the expression error, and the letters stand at the Expression Amount.
8. Save, close and open the project. The bounce plays the same.

## What you should see

`verification/B-949 pictures/bounce_frames.png` shows frames 0, 2, 4, 6, 9 and 24 from the top:
all letters up; the B starting down; a slope of letters; the last letters still up while the first
sit low; most letters dipped below their place with the e last; and the word at rest.

Wrong if: in step 4 the letters are all at the same height; in step 6 they fall together or never
settle; in step 7 the window freezes or the letters vanish; or a title saved before this change
looks different.

Not the same as After Effects: the selector has no Based On of its own (it uses the Range
Selector's); its Amount is one number, not three; and an animator holds at most one Expression
Selector.
