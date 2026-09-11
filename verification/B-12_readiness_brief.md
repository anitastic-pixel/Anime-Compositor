# Are we ready for the MVP, and ready to redesign?

Written 2026-09-11 after W-07 landed, at the owner's request: "see if we are ready for
mvp/to redesign since we have the foundational features/engine." This is the agent's reading of
the evidence against the project's own definitions. It opens no gate. Document 00 says an agent
must not, and that rule stands.

## The short answer

**The engine is ready. The MVP gate is yours to run, not mine to call. The redesign can start
now, and it should start from one thing this build does not yet have: your account of a shot
finished end to end.**

## What "MVP" means in this project

Document 13 names the milestones. **M1, internal alpha**, is B-02 through B-11 complete with
fixtures passing and limitations written down. **M2, artist alpha**, is B-12 complete: "the owner
finishes W-01 and W-02 unaided, blocking usability defects are fixed, and performance is reported
on the declared machine." M2 is the closest thing this project has to an MVP, and its exit is a
thing you do, not a thing the build does.

## Where the build stands against M1

Every G1-core requirement has a passing artifact you have already been shown, and the two G1-rest
features whose triggers fired are in as well. Read down the left column of document 23's G1-core
table:

| Feature | Requirement | Evidence |
| --- | --- | --- |
| PNG sequence import | R-01 | `B-03_import_table.md`, 60 of 60 |
| Cel exposure holds | R-02 | `B-04_exposure_table.md`, 46 of 46 |
| Layer stack, transforms, keys | R-03 | `B-05_model_table.md`, 75 of 75; `H-02_transformed_table.md` |
| Four blend modes | R-03, R-10 | `H-03_blended_table.md`, 10 of 10 |
| Viewer and stepping | R-06a | `B-08_*`, `T-06_declared_fixture.md` |
| Undo and redo | R-07 | `B-05_model_table.md`; `B-09_persistence_table.md` |
| Save, reopen, relink, recovery | R-08 | `B-09_persistence_table.md`, 97 of 97 |
| PNG export | R-09 | `T-08_export_table.md`, 47 of 47; `T-07e_roundtrip_table.md`, 23 of 23 |
| Colour and alpha | R-10 | `B-02_fixture_table.md`; `H-01_whole_picture_table.md` |
| Offline | R-11 | `B-11_offline_table.md` |
| Masks and mattes | R-04 (G1-rest) | `B-06_mask_table.md`, 41 checks |
| Three effects | R-05 (G1-rest) | `B-07_effects_table.md` |

The performance batch P-01 through P-11 is in, with the numbers in `P-11_effect_cache.md` and
`T-06_performance_envelope.md`. The window exists, with 42 commands the page can send and the
contract tests that hold the page to document 24, all regenerated on every build.

**Two debts document 11 still names, and neither is a blocker to starting M2.** T-10 (offline)
is "read off the build" rather than run with the network unplugged. Q-01, "no known reproducible
project corruption in the release candidate", waits for a release candidate to exist. Both are
statements to make at the end of M2, not before it.

## Where the build stands against M2

M2 has not started. What has happened instead is two sittings with the window, which produced 14
findings in `B-12_run_notes.md` and, from the first sitting, the decisions in
`B-12c_owner_brief.md`. Every defect from those sittings is fixed (W-03 through W-07). Neither
sitting was W-01 walked from the first step to the last on the reference shot, which is what B-12
asks for and what its exit artifact is: "the finished shot, and the owner's written account of
what was awkward."

The run sheet for it is `B-12_acceptance_run.md`. It is written step by step against the
controls the window has now. One sitting, with the shot in `Fixtures/reference_shot/`, is what
it takes.

## Ready to redesign?

**Yes**, on the specification's own terms. Document 05 says the design work "should follow a
spike proving the application launches and displays a composited frame." That condition was met
long ago. What the two sittings say is the more useful signal: your findings have moved from
"this does not work" to "this should feel like Premiere / After Effects / Resolve" - the
timeline's scrub zones, zoom about the pointer, a scale bar, a colour wheel, dragging values.
Those are design questions, and the current page is a verification harness that grew a face. It
was never designed; it was assembled so the commands could be checked.

**What the redesign is, per document 05.** A styled design system and the five G1-core screens,
produced in Claude Design and delivered as HTML and CSS into `design/`. Dark theme, type scale,
spacing scale, state colours, icon direction.

**What it costs, said plainly.**

- The new page must send the same 42 commands document 24 names, or document 24 changes first.
  This is the good news: the engine, the routes, the undo model and every fixture stay where they
  are. The redesign is a new face on the same body.
- The contract tests in `app/src/main.rs` pin about fifty short snippets of the current page's
  text (which handler sends which command, what fires per keystroke, what is reachable without a
  mouse). A new page means re-pinning every one of those, and the tables in `B-12b_*` and
  `B-12c_keyboard_table.md` are regenerated from them. That is a day of agent work per screen, not
  a risk, but it is the reason the redesign should land as one unit per screen rather than one
  big swap.
- The page is baked into the binary. Every design iteration is a rebuild before you can see it.
  That is already true today and it is the reason the first build you tried on 2026-09-11 showed
  nothing new.

## The recommendation

1. **Run W-01 once, on this build, before the redesign starts.** One sitting with
   `B-12_acceptance_run.md` open beside the window. Not because the gate needs it first - it does
   not, the two can overlap - but because the redesign would otherwise be designed against
   gestures rather than against a workflow you have completed. B-12's exit artifact, "what was
   awkward", is the design brief document 05 asks for, and nothing else produces it. If W-01
   cannot be finished on this build, that finding is worth more than any screen.
2. **Then redesign, one screen at a time, each screen a unit with its own rebuilt exe.** The
   timeline first, since it is where four of the fourteen findings landed and where the "like
   Premiere" ask is most specific.
3. **Decide the small things that block a clean run** before or during that sitting. From
   `B-12_run_notes.md`: whether `composition.create` stays, and whether the three size limits
   (16384 a side, 67,108,864 pixels, 10,000 frames) become the specification's. From
   `B-12c_owner_brief.md`: closing D-41, the reopened D-40, and whether 24 fps on a ten-layer
   shot is a target this project keeps. D-48 through D-51 are engine questions and block
   nothing on this page.

## What an agent will do next unless told otherwise

Nothing that changes the window. The exe at `target/release/anime_compositor_app.exe` holds
everything through W-07. The next unit is yours: the sitting.
