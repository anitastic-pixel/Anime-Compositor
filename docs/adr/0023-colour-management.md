# ADR-023: Colour management: our own named spaces, OpenColorIO as the outside judge

Status: ACCEPTED on 2026-10-26 with D-905, under the owner's answers ("go with your recommended
answers for colour management")
Date: 2026-10-26
Deciders: Andrew (owner), through wave 11's pick 7 and the answers of 2026-10-26
Relates to: ADR-007 (OpenColorIO not adopted, changed here), D-07 (the working space), D-17 (the
sRGB curve), D-62 (EXR interpretation), D-73 (exports, no colour choice), D-74 / D-182 / D-822
(stored and display colours), D-319 / D-330 / D-333 / D-763 (the After Effects depths), D-902
(step C1), D-906 (step C2), documents 08, 19, 20, 21, 27 and 28

## Context

The build has one fixed pipeline: sRGB in, linear light with Rec.709 primaries inside, sRGB out.
The primaries are only implied, and the sRGB curve is used in about 40 rules as "the display
encoding". After Effects (26.5 ships OpenColorIO 2.5.1 and the ACES 2.0 configs), Nuke, Resolve
and Blender all offer named working spaces, file tags and viewer transforms. On 2026-10-26 the
owner took pick 7 of the professional-programs research ("Do all of your picks at your
recommended order. Defer or tackle until solved if consequential to your discretion."), and then
answered the plan's nine questions: "go with your recommended answers for colour management".

Three ways were weighed: link OpenColorIO itself; build a small named set ourselves; or a hybrid,
our own set with OpenColorIO as the offline judge and the one thing too large to port (the ACES
output transform) baked into a table by OpenColorIO.

## Decision

The hybrid.

1. **Off is today.** Colour management is a project switch, off by default. While it is off every
   pixel, file, cache key and stored value is byte for byte what it was before this ADR. A new
   project key is written only when it is on, as `float_depth` is. Every step proves this with the
   existing suites.
2. **Our own named spaces.** `src/color.rs` holds the spaces (linear Rec.709/sRGB, ACEScg (AP1),
   ACES2065-1 (AP0), linear Rec.2020, linear P3-D65) and the curves (sRGB, BT.1886 gamma 2.4,
   gamma 2.2, ACEScct, ACEScc), written from their published specifications. Matrices are worked
   out from the chromaticities in double precision. Where the whites differ (the ACES white
   against D65) the adaptation is Bradford, which is what OpenColorIO's ACES configs were made
   with (measured in D-906: their matrices equal the Bradford ones to 1e-15). D-17's curve stays
   as the "sRGB" curve.
3. **OpenColorIO is the outside judge, offline.** PyOpenColorIO, pinned (2.5.1, the version After
   Effects 26.5 ships), BSD-3-Clause, installed from PyPI in its own environment, writes expected
   values for fixtures (`tools/ocio_reference.py`), from its built-in ACES 2.0 config read by its
   URI. It is never linked into the build and never shipped.
4. **The working space** when on is ACEScg by default; linear Rec.709 is also offered.
5. **ACES 2.0 only.** ACES 1.3 is added only if a real project needs it.
6. **The ACES view is a table baked by OpenColorIO**: a shaper and a 3-D table, applied on the
   processor and on the card by the code Color Lookup already uses, so the card matches the
   processor by construction. The table is judged against OpenColorIO's own maths: within 1 level
   of 255 on 99.9% of pixels for SDR. If it misses, the table grows, or a hand port becomes its own
   later unit.
7. **Depths.** With colour management on, only Display and Float are allowed. 8 bpc (AE) and
   32 bpc (AE) copy After Effects' non-colour-managed project and are refused with a reason.
8. **Stored colours keep their look.** Stored colours stay linear Rec.709 numbers in the file and
   are converted to the working space when drawn, so a picked orange stays that orange. The
   display-value rules (blend modes, luma, Levels, Curves, gradients, Color Lookup and others) take
   the colour to linear Rec.709 and then the sRGB curve.
9. **Exports never take the viewer's look** unless the export's own Output chooses it.
10. **The monitor is sRGB.** No wide-gamut or HDR display for now.
11. **No studio `.ocio` configs now.** That is step C8, asked first, when a real config arrives.
12. **Input files are tagged by the user, never guessed** (D-62's rule kept). Colour information a
    file carries but the build does not read is named, never silently dropped (D-902).

## The steps

| Step | What | Decision |
|---|---|---|
| C0 | This record; document 21's "Colour spaces"; document 28's reserved identifiers | D-905 |
| C1 | A PNG's own colour chunks named when not read | D-902 / B-782 |
| C2 | The named spaces and curves, checked against OpenColorIO; the app does not use them yet | D-906 / B-785 |
| C3 | The switch, the working space, "Interpret as" per file; conversion at decode | D-909 / B-788; in the window: D-916 / B-795 |
| C4 | Stored colours, display-value rules and the depths with colour management on | D-910 / B-789 |
| C5 | The viewer's view: Standard, Un-tone-mapped, ACES 2.0 SDR | D-915 / B-794 |
| C6 | An Output colour choice per export, files tagged honestly | D-917 / B-796 |
| C7 | Pickers and Info show working and display numbers | later |
| C8 | Studio configs, camera logs, wide-gamut and HDR monitors: ask first | waits |

## Consequences

- No C++ enters the build; ADR-005's dependency policy and document 29's reproducibility stand.
- Every colour number has an outside judge with a pinned version and optimisation level
  (`OPTIMIZATION_LOSSLESS`; OpenColorIO's default allows an approximate power off by 2.5e-5).
- Where OpenColorIO's config does not follow the published formula (its sRGB curve's smooth knee,
  its ACEScc table and clamps), the build follows the formula and the table says so per row.
- Only the viewer's ACES view, and an export that chooses it, touch a shader.
- A studio's own config cannot be loaded until C8.

## Not taken

- **Linking OpenColorIO (`ocio-sys` 0.3.0).** MSVC C++ and CMake in every build, about seven
  vendored C++ libraries, a crate three days old with one maintainer, a "stub mode" that does no
  colour management without saying so, and results that depend on OpenColorIO's optimisation flags
  and SSE path. ADR-007 rejected exactly this weight.
- **The pure-Rust `ocio` crate.** Too young to rely on (no published evidence); a timed spike for C8.
- **A hand port of the ACES 2.0 output transform.** About 1,500 lines of reference code, only an
  approximation until measured; the baked table is measured from the start.
- **ACES 1.3.** Not needed by any project yet.

## Reopened, all only while colour management is on

ADR-007 (here), D-07 (C3), D-17 (C2: kept, one curve of several), D-62 (C3), document 21's PNG
interpretation (C1, C3), D-74 / D-182 / D-822 and document 21's display-value rules (C4), D-319 /
D-330 / D-333 / D-763 (C4), document 08 and document 20 step 9 (C5), D-73 (C6). Unchanged: D-23
(trace images stay Standard), ADR-006 and D-100a (the card stays viewer-only).
