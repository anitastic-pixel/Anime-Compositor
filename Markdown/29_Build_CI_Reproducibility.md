# Build, CI and reproducibility specification

> **UPDATED for version 0.3.** The stack is now decided (D-06, ADR-003 to ADR-007), so this document no longer waits on B-01 to name a toolchain.
>
> Build description is **cargo**, with `Cargo.lock` committed. There is no CMake and no separate build system; the `cmake/` entry in the repository layout below does not apply. Repository layout is otherwise as described, except that planning documents live in `Markdown/` and decision records in `docs/adr/`.
>
> Toolchain pinning means: record OS edition and build, Rust toolchain version, Tauri and WebView2 versions, and exact crate versions from the lockfile. Everything else in this document stands.


Version 0.2 | 2026-09-04 | Proposed baseline

## Scope

This document defines the minimum engineering hygiene for a reproducible local/CI build after ADR-003 through ADR-007 are resolved.

Status as of 2026-09-04: ADR-003 through ADR-007 are accepted and B-01 has recorded the stack. The toolchain is pinned in `rust-toolchain.toml` at rustc 1.89.0, and `.github/workflows/ci.yml` runs the CI gates below that currently have something to run: build, tests, formatting and static analysis, plus a check that the committed verification artifacts still match what the build produces. Schema validation and persistence fixtures arrive with B-09 and the packaging smoke test with B-11.

## Repository layout target

`src/` production source; `tests/` automated tests; `Schemas/` project schemas; `Fixtures/` committed small fixtures; `docs/` canonical Markdown planning/specification files if migrated into the repository; `third_party/` only for explicitly vendored approved material; `cmake/` or equivalent build helpers; root `AGENTS.md` and `CLAUDE.md` for agent instructions.

## Toolchain pinning

Record OS edition/build, compiler name/version, SDK, build generator, dependency manager and exact dependency versions/revisions. Production releases must be rebuildable from a clean checkout plus documented prerequisites without relying on mutable `latest` tags.

Local developer builds may fetch dependencies during configuration if the chosen policy allows it; application runtime must not require network access for G1.

## Build configurations

Minimum configurations: Debug with assertions/sanitizer options where supported, and Release/RelWithDebInfo for performance/release tests. Warnings introduced by project code should be treated as build failures once the baseline is clean.

Formatting/lint rules are automated and checked in CI. Generated files are either reproducible from source or clearly marked; do not hand-edit generated schema bindings.

## CI gates

Every pull request/change set should run, as applicable:

1. configure/build on the primary supported platform;
2. unit tests for model/time/math;
3. schema validation and persistence fixtures;
4. render numeric/golden fixture tests;
5. dependency/license manifest check;
6. formatting/static analysis;
7. packaging smoke test on release branches or milestone gates.

GPU tests may run on a dedicated compatible runner rather than every CI job, but CPU reference tests are mandatory.

## Artifact provenance

Release artifacts record commit/revision, toolchain versions, dependency manifest hash, schema version and test summary. Do not publish performance numbers without the machine/driver/build configuration that produced them.

## Dependency updates

Update one consequential dependency at a time where feasible. Record reason, version delta, license delta, migration/behavior risk and affected tests. Automated dependency PRs do not bypass review for codecs, UI frameworks, GPU layers, expression runtimes or security-sensitive parsers.

## Security and untrusted inputs

Use compiler hardening and sanitizers where supported during development. Fuzzing is recommended for project/media parsers once parsers stabilize. CI fixtures must not contain secrets or copyrighted production footage without redistribution rights.

## Release reproducibility gate

Before M1/M2 distribution, build from a clean environment, run the applicable verification set, inspect packaged dependency notices and launch/save/export with network disabled. Record the result in a release manifest.

Related documents: 10, 11, 13 and 18.
