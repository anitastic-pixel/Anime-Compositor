# Competitive landscape and build-versus-extend analysis


> **AMENDED in version 0.3 under D-15.** The G0 comparison task specified below (running a mini-shot through Natron, Fusion, Blender and OpenToonz) **will not be run.** The owner has decided to build rather than extend.
>
> This is recorded as a **preference-based decision, not an evidence-based one.** The analysis in this document stands as competitive context and its reasoning about the structural gap directly shaped the G1-core narrowing in D-12. But the measured comparison it calls for does not exist, and nothing in this pack should be read as claiming it does.
>
> The off-ramp is therefore closed by choice, not by evidence. If the project stalls, reopening it remains legitimate.


Version 0.2 | 2026-09-04 | Proposed baseline

## Decision question

The project should exist only if a focused layer-first anime compositor solves a workflow/ownership gap more effectively than configuring or extending an existing tool. This is not a market-share report. The comparison below uses vendor/project documentation to establish capabilities and licensing constraints, then separates inference from fact.

## Reference products

After Effects remains the interaction/reference target for layer-based compositing concepts: Adobe documents a composition as an ordered layer system, including footage layers, cameras, adjustment layers, precompositions and 3D layers [S-14]. Adobe also documents a JavaScript-based expression language extended with application-specific objects [S-02]. These facts justify compatibility research; they do not authorize copying proprietary code/assets or promise `.aep` fidelity.

Natron presents itself as an open-source, node-based compositor and documents OpenFX support, tracking, roto, keying, curve/dope-sheet editing and headless rendering [S-11]. It is therefore a serious extend/reference candidate for conventional 2D compositing, but its node-first interaction model differs materially from this project's proposed exposure/layer-first workflow.

Blackmagic documents Fusion inside DaVinci Resolve as a node-based compositor with 2D and 3D tools, masks, animation editors and a true 3D workspace; a free Resolve download is available alongside the paid Studio edition [S-12]. This makes Fusion a strong baseline for "can an existing free tool already finish the shot?" tests, even though the proposed product is intentionally not node-first.

Blender's compositor is node-based and processes image/movie inputs through compositing nodes; current manuals also expose timeline-based animation around compositing workflows [S-10]. Blender is broad and extensible, but an anime-specific compositor would need to justify its existence through lower interaction overhead rather than raw feature breadth.

Foundry offers Nuke Non-commercial for personal/educational non-commercial work, with documented restrictions including 1920x1080 output and disabled/limited interoperability features [S-13]. Nuke is valuable as a high-end compositing reference, but the non-commercial license does not satisfy a general-purpose free commercial production requirement.

OpenToonz provides Xsheet/timeline exposure workflows and effects capabilities [S-09, S-16]. It is the closest reference here for animation timing/exposure concepts, but the project hypothesis is a dedicated finishing/compositing environment rather than a complete drawing/animation suite.

## Build-versus-extend matrix

| Candidate | Main strength | Main mismatch with project | Extension path to investigate | Decision status |
|---|---|---|---|---|
| Configure After Effects | known layer workflow/ecosystem | subscription/ownership goal; proprietary host | presets/scripts only; does not remove dependency | reference only |
| Extend Natron | open-source compositor, OFX | node-first UX; project seeks exposure/layer-first | fork/plugin/custom front-end feasibility | investigate G0 |
| Use/extend Fusion | mature free node compositor + 3D | node-first; integration tied to Resolve product | workflow templates/scripts, not independent ownership | benchmark/reference |
| Extend Blender | open-source, broad 3D/compositor | broad app complexity; node-first compositor | add-on/custom workspace/node tools | investigate if build cost rises |
| Use Nuke NC | high-end reference | non-commercial restrictions, node-first | reference/learning only | not product base |
| Extend OpenToonz | exposure-centric animation workflow | broader animation suite; compositor architecture differs | plugin/fork/reuse concepts under license review | investigate selectively |
| Build focused app | exact layer/exposure UX and ownership | largest engineering/maintenance burden | staged G1/G2 scope | current hypothesis |

## Required G0 comparison task

Before committing to years of custom implementation, run one representative mini-shot through at least Natron, Fusion, Blender/OpenToonz as relevant, and the G1 prototype when available. Measure setup steps, exposure-edit friction, revision/relink behavior, edge quality, save reliability and export repeatability. Record what required scripts/add-ons/workarounds.

A custom build is justified only if the gap is structural and repeated - for example, cel exposure/layer workflow and predictable finishing behavior - rather than a preference that an existing tool can solve with a small configuration.

## What not to conclude

Do not claim "no competitor exists" or "industry standard" from vendor pages. Vendor documentation has promotional incentives and does not measure artist preference. Free/non-commercial editions can also change terms/features, so distribution decisions require date/version checks.

## Provisional conclusion

Continue G0 custom-prototype work, but keep an explicit off-ramp: if an existing open-source base can satisfy W-01/W-02 with modest, maintainable customization, extending it may be economically superior. The current pack has not demonstrated that outcome either way.

Related documents: 01, 04, 16, 18 and 31. Sources: S-02, S-09 through S-16.
