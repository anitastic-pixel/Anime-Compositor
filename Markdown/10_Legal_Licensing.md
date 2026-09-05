# Legal, licensing and provenance review plan

> **UPDATED for version 0.3.** Distribution is **open source** (D-03, ADR-010), decided early and deliberately to remove ongoing license review from a project with no legal reviewer.
>
> Qt is **not** being used (ADR-004 selects Tauri), so the Qt licensing analysis below and source S-05 are retained as historical context only and no longer gate any decision. The dependency review obligations in this document still apply in full to the crates actually adopted.


Version 0.2 | 2026-09-04 | Proposed baseline

## Status and jurisdiction

This is a U.S.-oriented issue-spotting and engineering-control plan, not a legal clearance or legal opinion. Distribution jurisdictions, business structure and the eventual source license remain OPEN. Recheck applicable licenses and obtain qualified review for unresolved high-impact issues before distribution.

The U.S. Copyright Office distinguishes protected expression from ideas, methods and functional program aspects [S-04]. That distinction is not blanket permission to copy software, assets or interfaces, and it does not settle patents, trademarks, contracts, trade secrets or circumvention questions.

## Project policy

Implement general compositing operations from lawful references, independently written specifications or appropriately licensed source. Do not copy proprietary code, decompile restricted binaries, reuse branded assets, extract paid presets or bypass access controls. Preserve provenance for externally sourced algorithms, snippets, fixtures, fonts and icons.

Use original product naming and branding. Treat After Effects and plugin names as factual references in planning. Review public compatibility language and product naming for confusion before launch; no endorsement is implied by these documents.

## Dependency record

For every dependency, retain name, exact version/commit, upstream URL, license text, copyright/NOTICE files, modifications, build flags, linked components, distribution form and reviewer/date. Generate a software bill of materials from the final build inputs rather than a guessed list.

Qt offers commercial and open-source license paths with obligations that depend on the selected modules and use [S-05]. FFmpeg's license outcome depends in part on enabled components [S-06]. Both are candidates only; this pack does not approve a particular configuration or require their adoption.

---

## Effects and compatibility review

OLM states that some OpenTools are distributed under Apache 2.0 [S-01]. Verify the individual artifact, source availability and notices before reuse; do not extend this statement to every download, SDK or linked library. Publicly accessible source with unclear licensing stays blocked for copying until resolved.

A new implementation of familiar behavior needs its own specification and tests. Native AE project import, expression translation and binary plugin hosting each need separate technical and legal review. Do not assume a clean-room label or public API description resolves every legal issue.

Codec support requires a separate review of implementation licenses and potential patent obligations in intended distribution territories. A library license alone does not answer every codec-distribution question. PNG sequence delivery reduces early scope but is not a blanket legal clearance.

## Review triggers and evidence

Before dependency merge: archive the exact license and review configuration. Before reusing source: record origin and obligations. Before distributing sample artwork: record creator and permission. Before marketing compatibility: review the tested claim and limitations. Before public release: verify notices and source/relinking obligations applicable to the actual artifacts.

Proposed release packet: dependency inventory, licenses/notices, source provenance, asset permissions, compatibility wording, unresolved-issue list and reviewer sign-off. A developer may gather evidence; legal conclusions requiring professional judgment should be recorded by the appropriate reviewer.

## AI-assisted code provenance

AI-generated code receives the same dependency, security and similarity review as human-written code. Reject unexplained copied headers or code that appears tied to proprietary sources. Never instruct an agent to reproduce a commercial implementation from leaked material. Record added third-party packages explicitly; generation does not remove license obligations.

## Limitations and bias

Vendor license summaries may encourage commercial purchases; use them to identify obligations and consult the actual selected license text. Primary tool documentation establishes described behavior, not competitive superiority or legal clearance. This pack has not audited source code, trademarks, patents or distribution contracts.

Related documents: 06, 09, 13, 14 and 17.

Build/release dependency enforcement is specified in 29. Competitive and anime-tool research in 30/31 is observational; it does not authorize copying code, assets, branding or proprietary file formats.
