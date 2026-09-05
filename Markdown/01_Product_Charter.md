# Product charter and success criteria

Version 0.3 | 2026-09-04 | Accepted for baseline

## Problem and audience

The goal is a compositing tool for finishing digital and traditional 2D animation, including anime, that the owner owns outright and can run offline without a subscription.

Version 0.3 narrows the framing. Version 0.2 described an After Effects alternative. That description is retired, not because the ambition changed but because it set the wrong measuring stick: it invited comparison across a feature surface this project will never match and does not need to.

The accurate description is narrower and more defensible. This is a cel exposure and finishing compositor. Its subject is the timing of drawings and the assembly of layers into a finished frame sequence. Document 30 identified exposure-and-layer-first finishing as the one structural gap against existing free tools, and everything else on the version 0.2 feature list was a gap those tools have already closed. The product is the gap.

The audience is one animator finishing short shots from painted backgrounds and transparent cel sequences. Small-team handoff is a possible later concern. Studio pipeline replacement, general video editing and drawing tools are not targets and will not become targets.

## Product promise

Take existing artwork from import through timing, compositing, preview and reliable export, offline, with no subscription and no account.

The core demonstration is a 10-second, 1920 by 1080, 24 fps shot with one painted background and three transparent cel sequences on mixed exposure. The artist adjusts exposures, transforms layers, saves, reopens and exports all 240 frames identically. That is the milestone. Masks and effects are not part of it, and their absence is a deliberate statement about what this tool is for rather than an admission of incompleteness.

## Principles

Preserve drawings and intentional held frames; a compositor that silently alters timing has failed at its one job. Keep destructive actions reversible. Show how media, color and timing are being interpreted rather than assuming. Make unsupported behavior explicit and diagnosable. Favor a small coherent toolset over a catalog. Keep saved projects readable and usable without this application.

Treat familiar compositing concepts as inspiration only. Original branding, documentation and interface assets. Pixel-perfect duplication and format compatibility are not objectives.

---

## The constraint that shapes everything

The owner has no programming background and cannot read the code that will be written. This is stated in the charter, not buried in process documentation, because it determines the product as much as it determines the process.

It is why scope is narrow: unverifiable surface area is worse than absent features. It is why the project format is inspectable JSON. It is why correctness is defined by independent fixtures with expected values written in advance. It is why the renderer can dump its intermediate steps as images. And it is why the first milestone was cut roughly in half in version 0.3.

Document 12 specifies how verification works in practice. Nothing else in this pack should be read without it.

## Success measures

Milestone completion: the reference shot exports with the intended frame count, timing, alpha and color interpretation, and reopening the saved project reproduces that result exactly. Evidence is T-01 through T-10 with attached artifacts.

Usability: the owner completes W-01 unaided after a short introduction. Record completion time, errors and workarounds. Set improvement targets only after a baseline exists; do not invent a percentage improvement over any other tool.

Reliability: the release candidate passes recovery and low-resource tests with no known reproducible corruption defect. A release gate, not a claim of zero defects.

Performance: measured and reported on the declared reference machine, cold and warm separately, with no extrapolation to other hardware.

Verification integrity: every closed requirement has an artifact the owner actually reviewed. This is a success measure in its own right, because a project that cannot be checked has no other measures worth reporting.

## Release boundaries

The first useful release is G1-core: image-sequence import, exposure timing, layer transforms, deterministic export. The next adds masks and effects, if real shots demand them. Camera-driven parallax follows after that.

Permanently outside scope: drawing and inking, automatic in-betweening, character rigging, particles, general 3D rendering, multi-user editing, plugin marketplaces and AE project import. Public claims of studio readiness are not to be made at all.

## Ownership and remaining unknowns

Andrew is the product decision owner, artistic reviewer and sole verifier. There is no engineer, no legal reviewer and no release manager, and this pack does not assume that any will appear.

Remaining open: weekly capacity, which is deliberately uncommitted, and the public name, which is deferred until there is something to name. Distribution, platform, stack, renderer and reference artwork are closed in document 14.

A narrow reliable workflow that gets finished is worth more than a broad one that does not. If capacity proves tighter than the current scope, narrow the scope again rather than extending the schedule silently.

Related documents: 02, 03, 04, 12, 13, 14 and 23.
