# Two decisions an agent made for you, and what it costs to take them back

D-45 and D-46 are marked PROVISIONAL in `Markdown/14_Decisions_Risks.md`, which means an agent
answered a question that was not an agent's to answer, wrote down what it did and why, and left
the answer standing so that B-07 could be finished. They have been standing since 2026-09-06.
This page is here so they can be closed or reversed without reading the register: what the
question was, what was built, what you would look at to disagree, and what taking it back costs
in this build today.

Nothing here is a defect and nothing here is urgent. Both entries are marked, both are reversible
for less than a day's work, and the reason to settle them is that a PROVISIONAL entry is a debt
that grows: every week more code is written on top of an answer nobody chose.

---

## D-45 — changing an effect's settings is a fourth command

**The question.** Document 24 gives the effect stack three commands: add an effect, delete an
effect, bypass an effect. There is no command that changes a setting on an effect that already
exists. R-05 is choosing a blur radius, a number of stops and a colour, so the three commands as
listed do not add up to using an effect at all.

**What was built.** A fourth command, `effect.set_parameters`, carrying which effect instance and
a complete replacement of that instance's settings. It refuses settings that belong to a
different kind of effect, and refuses values outside contract, without changing the stack.

**Why not just edit the value.** Undo in document 26 is defined over commands. A change made
outside a command is a change that cannot be undone, and the undo register would quietly stop
matching the file — the failure would show up later, as an undo that puts back the wrong thing.

**What you would look at to disagree.** `verification/B-07_effects_table.md`: the command rows
cover both refusals, that a refusal leaves the stack exactly as it was, and that a settings
change is undoable like anything else. In the window it is the Radius, Stops, Colour and Amount
fields in the EFFECTS panel; typing in one of them is this command.

**What is left open even if you close it.** Document 24 is a map of interactions, and this entry
only settles that the command exists. It does not settle what document 24 should say about the
gesture that produces it, which is a documentation decision rather than a build one.

**Cost of reversing it.** One command variant, one row of document 24, the four fields in the
EFFECTS panel, and the question comes straight back: an effect that cannot be adjusted has to be
deleted and re-added to change a number, which loses that instance's place in the stack every
time.

---

## D-46 — a bad effect setting is refused in a command and only reported in a file

**The question.** Document 28 gives `EFFECT_PARAMETER_INVALID` one severity, ERROR, and one
remediation, "reject edit/load as appropriate". That decides nothing for the case that matters:
a project already on disk holding, say, a negative blur radius. Refusing the load throws away the
whole project over one number. Repairing the number silently changes what you saved without
telling you, which is the silent fidelity fallback this project's own instructions forbid.

**What was built.** The same answer D-43 gave a mask outline this build refuses to draw, one day
earlier, for the same reason:

| Where the bad value appears | What happens |
| --- | --- |
| in a command, while you are working | refused outright; the effect stack is left exactly as it was |
| in a project file being opened | opened, and the bad value **kept unrepaired**; that one effect is bypassed and the rest of the stack still runs; the warning is recorded on **every frame it affects**, not once at load; an export made from it is marked as incomplete fidelity |

**The part that is not bookkeeping.** Recording it per frame rather than once at load is what
stops a picture going out with an effect quietly missing from it and nothing in the render saying
so. Without that line, an export would look finished.

**What you would look at to disagree.** `verification/B-07_effects_table.md`, the command and
persistence rows: that a refused command changes nothing, that a saved bad value is still in the
file after a reopen, and that the export is marked.

**Cost of reversing it.** The two-severity row in document 28 and one branch in the loader. What
you would be choosing instead is one of the two answers this rejected: refuse to open the
project, or repair the number without saying so.

---

## What "closing" one of these looks like

Either entry can be closed as it stands — the answer becomes yours rather than an agent's and
nothing in the code changes — or reversed, which is the work above. Both entries in
`Markdown/14_Decisions_Risks.md` already carry their evidence basis and their cost of reversal in
their own words; this page is the same information without the register around it.

There is a third possibility worth naming for D-46, because it is not a reversal: keep the two
severities and change what a warned effect does to the picture. Today the one bad effect is
bypassed and the rest of the stack runs. Skipping the whole stack, or refusing to render the
layer, are both defensible and neither was decided — the build does the least destructive of the
three.
