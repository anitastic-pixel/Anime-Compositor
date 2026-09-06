# -*- coding: utf-8 -*-
"""Writes docs/DEPENDENCIES.md from the build inputs.

Document 10: "Generate a software bill of materials from the final build inputs rather than a
guessed list." So the table below is not typed by hand. Names, versions, declared licences and
upstreams come from `cargo metadata`; the checksums come from `Cargo.lock`. The prose is written
by hand and lives here so that regenerating the table cannot silently drop it.

Two things this used to decide by hand and now derives, because the shell brought the graph from
twenty-eight crates to two hundred and sixty-four and a hand-kept set of names silently stops
being true at that size:

  which crates are direct   - read off the dependency lists of this workspace's own manifests
  which are build-time only - anything the linked walk cannot reach, see `linked()`
"""
import json, os, subprocess, io

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# The only platform this project builds for (ADR-001, and the CI runner is windows-latest). Asking
# cargo about every platform at once answers a question nobody has: it returns 435 crates, 171 of
# which are the macOS and Linux windowing stacks that never compile here. A record naming crates
# this build does not contain is a guessed list with extra steps.
PLATFORM = 'x86_64-pc-windows-msvc'


def metadata():
    out = subprocess.run(['cargo', 'metadata', '--format-version', '1',
                          '--filter-platform', PLATFORM],
                         cwd=ROOT, capture_output=True, text=True, encoding='utf-8')
    if out.returncode != 0:
        raise SystemExit('cargo metadata failed:' + chr(10) + (out.stderr or '').strip())
    return json.loads(out.stdout)


def direct(meta):
    """Crate names this workspace's own manifests ask for by name.

    Everything else in the table arrived underneath one of these. Read off the manifests rather
    than listed here, so that adding a dependency cannot leave the record describing the old
    set."""
    names = set()
    for p in meta['packages']:
        if p.get('source'):
            continue
        for dep in p['dependencies']:
            names.add(dep['name'])
    return names


def linked(meta):
    """Ids of the crates that end up inside what this project would ship.

    Walk from the workspace's own packages along normal dependency edges only. Build dependencies
    are not followed, and a proc macro is added but not walked through: it runs inside the compiler
    and its own dependencies run there with it. Whatever the walk does not reach is compiled during
    the build and then left behind.

    That distinction is the one a licence review cares about most, which is why it is computed from
    the resolved graph rather than remembered."""
    pkg = {p['id']: p for p in meta['packages']}
    nodes = {n['id']: n for n in meta['resolve']['nodes']}
    members = set(meta['workspace_members'])
    reached, stack = set(), list(members)
    while stack:
        for dep in nodes[stack.pop()]['deps']:
            if not any(k['kind'] is None for k in dep['dep_kinds']):
                continue
            if dep['pkg'] in reached or dep['pkg'] in members:
                continue
            reached.add(dep['pkg'])
            if not any('proc-macro' in t['kind'] for t in pkg[dep['pkg']]['targets']):
                stack.append(dep['pkg'])
    return reached


def checksums():
    lock = io.open(os.path.join(ROOT, 'Cargo.lock'), encoding='utf-8').read()
    out = {}
    for block in lock.split('[[package]]')[1:]:
        f = {}
        for line in block.split(chr(10)):
            line = line.strip()
            for key in ('name', 'version', 'checksum'):
                head = key + ' = "'
                if line.startswith(head) and line.endswith('"'):
                    f[key] = line[len(head):-1]
        if 'name' in f and 'version' in f:
            out[(f['name'], f['version'])] = f.get('checksum', '')
    return out


def table(meta):
    sums = checksums()
    is_direct = direct(meta)
    is_linked = linked(meta)
    rows = ['| Crate | Version | Declared licence | Role | Form | Upstream | crates.io SHA-256 |',
            '|---|---|---|---|---|---|---|']
    external = sorted((p for p in meta['packages'] if p.get('source')),
                      key=lambda p: (p['name'], p['version']))
    for p in external:
        digest = sums.get((p['name'], p['version']), '')
        digest = '`%s…`' % digest[:16] if digest else '(not from crates.io)'
        rows.append('| `%s` | %s | %s | %s | %s | %s | %s |'
                    % (p['name'], p['version'], p.get('license') or '(none declared)',
                       'direct' if p['name'] in is_direct else 'transitive',
                       'linked' if p['id'] in is_linked else 'build-time only',
                       p.get('repository') or '', digest))
    return chr(10).join(rows), len(external), len(external) - len(is_linked)


PROSE_HEAD = """# Dependency record

B-11's dependency and licence record. This file supersedes the short generated table that stood
here before; it is the single dependency record ADR-005 asks for, and `.gitignore` names it as the
reason `Cargo.lock` is committed.

`tests/b11_dependency_record.rs` checks this file against `Cargo.lock` in both directions and
writes `verification/B-11_record_table.md`. The table is produced by `tools/gen_dependencies.py` from
`cargo metadata` and `Cargo.lock` rather than typed, because document 10 asks for a bill of
materials "generated from the final build inputs rather than a guessed list". The prose sections
are written by hand.

Distribution form: statically linked, open source (ADR-010, D-03), under `MIT OR Apache-2.0`
(D-31, decided 2026-09-05). No dependency is modified. No non-default build flags are set. Reviewer: none. Date reviewed: none. Both are blank on purpose;
see the last section.

## Bill of materials

Every crate the build resolves for `x86_64-pc-windows-msvc`, at the version it resolved.

The platform matters to what this list means. Asked about every platform at once, cargo answers
with 435 crates, and 171 of those are the macOS and Linux windowing stacks that this build never
compiles. Naming them here would describe a program nobody has. ADR-001 makes Windows the only
supported platform and CI runs `windows-latest`, so the record is filtered to match.

`miniz_oxide` appears twice because two majors of it are in the graph at once, reached by different
dependants; that is not an error, and the check compares whole version sets per crate so that it
stays visible. Several other crates now do the same for the same reason.

"""

PROSE_TAIL = """

## Purpose — why each direct dependency is here

The workspace names four dependencies by hand. Everything else in the table above arrived
underneath one of them.

- **`png`** decodes the cel images the compositor reads and encodes the frames it exports. PNG is
  the format the reference shot is drawn in and the format document 21 names for export. Writing a
  PNG encoder that is correct about bit depth, alpha and interlacing is not work this project has
  any reason to do.
- **`rayon`** renders frames in parallel. A 240-frame export is 240 independent compositions, and
  the export path is the only place it is used.
- **`serde_json`** reads and writes the project file. The format is JSON by ADR-008; the
  alternative is a hand-written parser, which is a source of silent data loss and the one failure
  this project is least able to tolerate.
- **`tauri`** is the window, chosen in ADR-003 and ADR-006. It is a dependency of `app/` only, not
  of the rendering core: document 06 requires that "the rendering core stays independent of the
  interface", and keeping the shell in its own crate is what makes cargo enforce that rather than
  leaving it to good intentions. `tauri-build` is its build-time half.

## What the shell cost, in crates

This is the honest number and it is worth stating plainly. Before the window, this record held
**28** crates. With it, **{total}**. One dependency brought in roughly two hundred and thirty
others, which is what a browser engine, an async runtime, a CSS selector engine and a bundler
amount to once they are counted rather than assumed.

That was known when ADR-003 chose Tauri, and the alternative it was weighed against was writing a
window and a rendering surface by hand. The number is recorded here so the trade is visible rather
than implied. **{build_only}** of them are build-time only — compiled, run during the build, and
absent from anything this project would ship.

An archive this size is not maintainable by hand, which is why it is not maintained by hand any
more. `tools/archive_licenses.py` copies every licence text out of the crate sources cargo
unpacked, `verification/B-11_license_archive.md` is the artifact it writes, and `--check` runs in
CI.

## What a reviewer has to look at

Named, not decided. Document 10 reserves that judgement: "legal conclusions requiring professional
judgment should be recorded by the appropriate reviewer." The first entry is the one that changed
with the shell and it should be read first.

- **`MPL-2.0` — five crates.** `cssparser`, `cssparser-macros`, `dtoa-short`, `option-ext` and
  `selectors`, all linked, all arriving under `tauri`. **This graph is no longer entirely
  permissive.** MPL-2.0 is file-scope copyleft: it attaches to the MPL-licensed files themselves
  rather than to a larger work that links them, so it does not reach this project's own code the
  way a GPL would. It does carry an obligation to make the source of those files available on
  distribution. This project is open source and modifies no dependency, which is the easiest
  possible position to be in, but easy is not the same as reviewed — and the previous version of
  this record said "nothing in the graph is copyleft". That sentence was true of twenty-eight
  crates and is not true of two hundred and sixty-four.
- **Eleven crates ship no licence text at all.** `alloc-stdlib`, `defmt-parser`, `selectors`, the
  five `unic-*` crates and the three `webview2-com*` crates declare terms in their manifests and
  publish no file carrying them. `selectors` is the one to look at twice: it is the MPL-2.0 crate,
  and MPL-2.0 is the licence in this graph with the most to say about notices. Their archive
  directories hold a generated `NO-LICENCE-TEXT-SHIPPED.md` recording the absence, because an empty
  directory cannot state a fact and git does not keep one anyway.
- **`Unicode-3.0` alone — eighteen crates.** The ICU family: `icu_collections`, `icu_locale_core`,
  `icu_normalizer`, `icu_normalizer_data`, `icu_properties`, `icu_properties_data`, `icu_provider`,
  `litemap`, `potential_utf`, `tinystr`, `writeable`, `yoke`, `yoke-derive`, `zerofrom`,
  `zerofrom-derive`, `zerotrie`, `zerovec`, `zerovec-derive`. Not a choice of MIT or Apache; the
  Unicode licence is the only terms offered. It is permissive and carries an attribution
  requirement, and it is in the graph because URL and text handling need Unicode tables.
- **`unicode-ident` 1.0.24** — `(MIT OR Apache-2.0) AND Unicode-3.0`. The `AND` is the point: this
  is not a choice of one licence, both sets of terms apply. It is build-time only, which likely
  changes the answer, but "likely" is not a review.
- **Conjunctive terms elsewhere.** `brotli` is `BSD-3-Clause AND MIT` and `ryu` is
  `Apache-2.0 AND MIT`. Both mean both, not either.
- **Crates offering no MIT or Apache option at all.** `tao` 0.35.3 is `Apache-2.0` only and is the
  windowing layer, so it is unavoidable for as long as there is a window. `zlib-rs` 0.6.7 and
  `foldhash` 0.2.0 are `Zlib`. `alloc-no-stdlib` and `alloc-stdlib` are `BSD-3-Clause`. All
  permissive, none a choice.
- **`memchr` 2.8.3** — `Unlicense OR MIT`, and `same-file` and `walkdir` alongside it. The
  Unlicense is a public-domain dedication whose standing differs by jurisdiction, and document 10
  records that distribution jurisdictions are still open. `memchr`'s `UNLICENSE` file was absent
  from `Licenses/` until 2026-09-06, when `tools/archive_licenses.py` was written and found it: the
  record discussed a licence whose text the archive did not hold. That was the one disagreement in
  twenty-eight hand-made directories, and it is the reason the archive is no longer made by hand.

## Licence compatibility, as an engineering read

Not a legal opinion, and a less confident one than this section used to give. Every crate above
offers permissive terms. Five are weak copyleft at file scope (MPL-2.0) and carry a source
availability obligation on distribution rather than a licence on this project's own code; nothing
in the graph is copyleft in the sense that would constrain what licence this project may carry.
Roughly a dozen crates offer no MIT or Apache option, so a distribution has to satisfy their terms
as written rather than choosing the familiar ones.

The archived texts under `Licenses/` are what a distribution would have to carry with it. Two
checks keep them honest and neither reads a word of the terms: `tests/b11_dependency_record.rs`
confirms a directory exists for every crate at its resolved version, and
`tools/archive_licenses.py --check`, which runs in CI, confirms that what is inside each directory
is the set of files that crate actually ships. The second is the one a hand-maintained archive
fails quietly, because a missing file looks exactly like a crate that never had one.

## What this record does not yet contain

- **A reviewer and a date.** There has been no legal reviewer. Inventing a sign-off would be worse
  than leaving it blank. The MPL-2.0 entry above is the first thing in this project that genuinely
  wants one.
- **NOTICE files.** Document 10 lists them separately from licence texts. That is read off the
  crate sources themselves rather than off the archive — `tools/archive_licenses.py` looks for
  `NOTICE` alongside the licence names and reports every crate shipping no text at all — but it is
  still not verified by a reviewer.
- **A distribution.** T-16 stays NOT RUN because there is no distributable build to check. Nothing
  here has been shipped to anyone, so no obligation in it has come due. `bundle.active` is `false`
  in `app/tauri.conf.json` for that reason.
- **A signed-off review.** D-31 is now closed — the project is `MIT OR Apache-2.0`, `Cargo.toml`
  declares it and `LICENSE-MIT` and `LICENSE-APACHE` are in the repository root — but that is the
  owner choosing a licence, not a reviewer confirming that this graph may be redistributed under
  it. The copyright line in `LICENSE-MIT` names the GitHub identity that owns the repository; the
  owner should replace it with whatever name belongs on the notice.
"""

if __name__ == '__main__':
    meta = metadata()
    body, total, build_only = table(meta)
    io.open(os.path.join(ROOT, 'docs', 'DEPENDENCIES.md'), 'w', encoding='utf-8',
            newline=chr(10)).write(
        PROSE_HEAD + body + PROSE_TAIL.format(total=total, build_only=build_only))
    print('wrote docs/DEPENDENCIES.md: %d crates, %d build-time only' % (total, build_only))
