# Delivery, capacity and release plan

Version 0.3 | 2026-09-04 | Accepted for baseline

## Planning basis

There are no dates in this document and there will not be until D-04 closes. Weekly capacity is deliberately uncommitted, and inventing a schedule on top of an uncommitted capacity produces a number that is wrong in a way that feels authoritative. Progress is tracked by dependency gates instead.

What is known: development is solo, assisted by Claude Code on a subscription with no API spend, with the owner as sole verifier. The scarce resource is not tokens and not compute. It is owner attention: the hours available to specify expected behavior, review verification artifacts and judge whether output is actually right. Every planning decision in this pack should be read against that constraint.

Track implementation, verification, artistic review and maintenance as separate costs. Fast code generation does not reduce the verification cost, and verification is the bottleneck.

## Milestones and exit evidence

M0 / Feasibility. B-01 complete: five spike reports recorded, reference shot drawn, accepted ADRs either confirmed or explicitly reopened. Exit: measured spike results on the reference machine, not estimates.

M1 / Internal alpha. B-02 through B-11 complete, covering G1-core only. Exit: applicable fixtures pass with artifacts the owner has reviewed, save and recovery work, and known limitations are written down honestly. Use the reference shot and copies of the owner's own artwork.

M2 / Artist alpha. B-12 complete. Exit: the owner finishes W-01 and W-02 unaided, blocking usability defects are fixed, and performance is reported on the declared machine. This validates the supported workflow only; no studio-readiness claim follows from it.

M3 / G1-rest. Masks, effects and cache, each promoted individually when its trigger in document 23 fires. Not a scheduled milestone; a set of conditionally unlocked ones.

M4 / 2.5D. B-13 and B-14. Exit: G2 fixtures and W-04 pass with documented plane-order and expression limitations.

---

## Capacity worksheet

When D-04 closes, record available hours per week, review availability and any budget. For each remaining task record low and high effort, dependencies and confidence. Calendar duration then follows from available capacity and blocked time, and is never computed by dividing total estimated effort by assumed full-time hours.

Reserve explicit effort for regressions and support, sized from actual defect data rather than a guessed percentage. Re-estimate at every milestone and whenever a dependency changes the architecture.

## Cost and sustainability

Distribution is open source per D-03, so there is no revenue model and none is sought. What remains are real costs to plan for: build infrastructure if any is used, release hosting, documentation, and ongoing compatibility work as Windows and WebView2 change underneath the application.

Code signing is worth considering before public distribution, since an unsigned binary triggers SmartScreen warnings that make a legitimate tool look untrustworthy. This is a cost decision, not a technical one, and it can wait until there is something to distribute.

No subscription or cloud dependency may be added to core editing. That is a charter commitment, not a preference.

## Release procedure

Freeze the candidate revision and dependency versions. Run applicable fixtures and inspect the actual packaged build on a clean supported environment, not on the development machine, because the development machine hides missing dependencies.

Verify installation, first launch, fully offline operation, save locations, recovery, export and uninstall. Package license notices, known limitations, the supported hardware and OS envelope, and recovery guidance. Preserve reproducible build instructions and rollback artifacts.

Re-run the viewer color and transport fixtures against each release candidate, because WebView2 updates outside this project's control per K-08.

An updater is not planned. Manual download is sufficient and removes an entire class of authenticity and failure-handling work.

## Support and incident handling

A bug report carries build, OS, GPU and driver, reproduction steps, expected and actual result, and a minimal project. Remove private paths and artwork unless deliberately shared. Telemetry and crash uploads are opt-in only, if they exist at all.

For suspected corruption: preserve the original project first, work on a copy, document the recovery and issue a verified fix. Never overwrite the only copy during investigation.

## Go and no-go record

Release owner, product reviewer and verifier are all the same person. Record the build, gates passed, accepted limitations, unresolved risks and distribution scope. No gate is currently passed; this document plans future delivery and asserts nothing about the present.

If capacity proves insufficient for G1-core as scoped, narrow the scope again rather than extending silently. Document 04 describes how, and version 0.3 already did it once.

Related documents: 01, 04, 10, 11, 12, 14 and 15.
