# Production-tool references and research agenda

Version 0.2 | 2026-09-04 | Proposed baseline

## Evidence boundaries

The sources below establish that named tools and documented capabilities exist. They do not establish universal adoption, comparative quality or that users want an identical implementation here. Industry-wide usage and unmet AE needs remain research questions.

OLM states that the tools on its page were developed with artists and used in its production work, and that some OpenTools are distributed under Apache 2.0 [S-01]. This is first-party production evidence for OLM, not a neutral survey of all studios.

## Candidate capability mapping

OLM Smoother / Smoother v2: investigate edge smoothing that protects line character. Compare thin diagonals and flat-color boundaries over changing drawings. OLM Blur / Directional Blur: investigate cel-oriented softness and directional treatment. Compare transparent borders and held-frame behavior.

OLM Color Key and Toon Dilate: investigate targeted color separation and extension near edges. Test line contamination and halos. OLM KiraKira and Distance Gradation: later research into highlight treatment and distance-based falloff. Do not treat these names as planned native product names [S-01].

PSOFT anti-aliasing for AE: vendor documentation confirms an AE effect and render-engine support [S-07]. Use as a capability reference for comparison; source reuse and interoperability are not approved. Assess whether a native smoothing operation solves the same artist problem without copying implementation.

F's Plugins: the author's public repository identifies an AE plugin collection [S-08]. Review exact effect needs and per-artifact licensing before reuse. Source visibility alone does not resolve licensing, SDK dependencies or portability.

---

## Adjacent workflows

OpenToonz documents both Xsheet/timeline workflows and an effects system [S-09]. Study exposure-oriented interaction and animation handoffs. Do not assume its native projects can be imported; that requires a separate format and licensing investigation.

Blender documents node-based compositing [S-10]. Evaluate it as an adjacent finishing option and a possible reference workflow when testing whether a new application is justified. Do not confuse node-based capabilities with the proposed layer-first interaction model.

The user's original concern is affordability and workflow fit. This pack does not rank all competing products or conclude that an independent application is the cheapest path. An extend-versus-build review remains a useful G0 decision if implementation cost proves excessive.

## Quality-of-life hypotheses

Hypotheses to validate: artists may value explicit exposure editing, predictable cache invalidation, quick media revision, strong relink/recovery, searchable commands, readable render errors and fewer repetitive setup steps. These are proposed design priorities, not findings from a user survey.

For each hypothesis, observe the task, count repetitions and errors, prototype the smallest improvement and compare against the baseline. Avoid claims such as “AE users universally want this” or “no other app has this” without targeted evidence.

## Research worksheet

For each candidate record: task, source/date, stated capability, firsthand usage evidence, license status, representative input, desired result, undesirable artifacts, estimated integration burden and decision. Status is RESEARCH until a tested requirement is accepted into 03.

Initial recommended order: held-frame workflow and revision handling; alpha/edge correctness; smoothing and line recolor; then glows, blur variations and packaging. This order is an inference from the project's focus, not a verified studio standard.

## Bias and limitations

Vendor and developer sources are useful for behavior and licensing but have adoption/promotional incentives. No comparative benchmark or artist interview has been conducted. The catalog is deliberately selective and is not an exhaustive professional anime plugin inventory.

Related documents: 02, 04, 09, 10 and 17.

Expanded research: document 30 compares build-versus-extend candidates; document 31 turns anime-specific production references into explicit research questions and promotion criteria.
