"""Collect and package, worked a second way.

D-61 proposes how R-14 collects a project and its media into one folder that opens from
anywhere, with a manifest of every file's size and SHA-256. This file is a whole
implementation of those rules in Python, written from D-61 and sharing nothing with the build.
The build hashes with its own SHA-256; this one uses Python's hashlib, so the two agree only if
both are right.

It writes, under `Fixtures/packaging/`:

- `source/`: a small shot, `shot.json`, and its drawings, laid out to catch every rule: a
  drawing used for two frame numbers, two drawings with the same file name, a file name that
  is not ASCII, two asset IDs that become the same folder name, an ID that is a Windows
  device name, media the artist may not pass on, a missing drawing and an unused asset.
- `expected_manifest.json`: the exact bytes the build must write as `package-manifest.json`.
- `expected_package.json`: where every drawing must be in the package, what the packaged
  project must say, and what checking the package must answer in six situations.

Fixtures are read-only to implementation work: this file is run when the specification
changes, and never to make a build pass.

    python tools/package_reference.py
"""

import copy
import hashlib
import json
import shutil
import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "packaging"
SOURCE = OUT / "source"

FORMAT = "anime-compositor-package"
MANIFEST = "package-manifest.json"
MEDIA = "media"
BAD = set('<>:"/\\|?*')
DEVICES = {"con", "prn", "aux", "nul"} | {f"com{i}" for i in range(1, 10)} | {
    f"lpt{i}" for i in range(1, 10)
}


# --- The naming rule (D-61) --------------------------------------------------------------


def safe(name):
    """One path segment that every supported file system accepts."""
    name = "".join("_" if c in BAD or ord(c) < 32 else c for c in name)
    name = name.rstrip(". ")
    if not name:
        name = "_"
    if name.split(".")[0].lower() in DEVICES:
        name = "_" + name
    return name


def with_suffix(name, n):
    """`x.png` with 2 is `x-2.png`; a name with no extension, or only a leading dot, gets it
    at the end."""
    dot = name.rfind(".")
    if dot <= 0:
        return f"{name}-{n}"
    return f"{name[:dot]}-{n}{name[dot:]}"


def unique(name, taken):
    """The first of name, name-2, name-3 ... not already taken, ignoring case."""
    n = 1
    candidate = name
    while candidate.lower() in taken:
        n += 1
        candidate = with_suffix(name, n)
    taken.add(candidate.lower())
    return candidate


# --- The shot ------------------------------------------------------------------------------


def png(rgba):
    """A 4 by 4 PNG of one colour."""
    raw = b"".join(b"\x00" + bytes(rgba) * 4 for _ in range(4))

    def chunk(kind, data):
        body = kind + data
        return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", 4, 4, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw, 9))
        + chunk(b"IEND", b"")
    )


FILES = {
    "media/cel_0001.png": (255, 0, 0, 255),
    "media/cel_0002.png": (0, 255, 0, 255),
    "bg/背景.png": (40, 40, 90, 255),
    "a/x.png": (255, 255, 0, 255),
    "b/x.png": (0, 255, 255, 255),
    "c/X.png": (255, 0, 255, 128),
    "licensed/sheet.png": (200, 120, 30, 255),
}

INTERPRETATION = {"color_space": "srgb", "alpha": "straight"}


def still(id, name, path, **extra):
    return {"id": id, "kind": "still", "name": name, "path": path,
            "interpretation": dict(INTERPRETATION), **extra}


def sequence(id, name, pattern, frames):
    return {"id": id, "kind": "image_sequence", "name": name, "pattern": pattern,
            "frames": {str(k): v for k, v in frames.items()},
            "interpretation": dict(INTERPRETATION)}


ASSETS = [
    # Frame 3 holds frame 2's drawing: one file, copied once, listed once with both numbers.
    sequence("asset-cel", "Cel", "cel_####.png",
             {1: "media/cel_0001.png", 2: "media/cel_0002.png", 3: "media/cel_0002.png"}),
    # Used by two layers in two compositions; a file name that is not ASCII.
    still("asset-bg", "Background", "bg/背景.png"),
    # Three drawings called x.png, one of them X.png: the second and third are renamed.
    sequence("asset-mix", "Mix", "x.png", {1: "a/x.png", 2: "b/x.png", 3: "c/X.png"}),
    # The artist may not pass this on: listed, hashed, not copied.
    still("asset-licensed", "Licensed sheet", "licensed/sheet.png", redistribute=False),
    # One drawing is missing and one is there.
    sequence("asset-gone", "Gone", "gone_####.png",
             {1: "media/gone_0001.png", 2: "media/cel_0001.png"}),
    # Two IDs that make the same folder name; the second in project order is renamed.
    still("shot 1/é", "Odd one", "bg/背景.png"),
    still("shot 1?é", "Odd two, unused", "media/cel_0001.png"),
    # A Windows device name.
    still("con", "Device name", "media/cel_0002.png"),
]


def layer(template, id, asset, frames):
    out = copy.deepcopy(template)
    out.update(id=id, name=id, asset_id=asset, in_frame=0, out_frame=frames)
    out["exposure_spans"] = [
        {"start_frame": 0, "end_frame_exclusive": frames, "drawing_number": 1}]
    return out


def shot():
    project = json.loads((ROOT / "Fixtures/projects/minimal_project.json").read_text("utf-8"))
    held = json.loads((ROOT / "Fixtures/projects/cel_holds_project.json").read_text("utf-8"))
    template = held["compositions"][0]["layers"][0]
    project["project_id"] = "proj-package"
    project["assets"] = ASSETS
    main = project["compositions"][0]
    main["duration_frames"] = main["work_area"]["end_frame_exclusive"] = 3
    main["width"], main["height"] = 64, 36
    main["layers"] = [
        layer(template, "layer-bg", "asset-bg", 3),
        layer(template, "layer-cel", "asset-cel", 3),
        layer(template, "layer-mix", "asset-mix", 3),
        layer(template, "layer-licensed", "asset-licensed", 3),
        layer(template, "layer-gone", "asset-gone", 3),
    ]
    for l, spans in zip(main["layers"][1:3], ([1, 2, 3], [1, 2, 3])):
        l["exposure_spans"] = [
            {"start_frame": i, "end_frame_exclusive": i + 1, "drawing_number": n}
            for i, n in enumerate(spans)]
    main["layers"][4]["exposure_spans"] = [
        {"start_frame": 0, "end_frame_exclusive": 3, "drawing_number": 2}]
    alt = copy.deepcopy(main)
    alt.update(id="comp-alt", name="Alt")
    alt["layers"] = [
        layer(template, "layer-bg-alt", "asset-bg", 3),
        layer(template, "layer-odd", "shot 1/é", 3),
        layer(template, "layer-con", "con", 3),
    ]
    for c in (main, alt):
        c["layer_order"] = [l["id"] for l in c["layers"]]
    project["compositions"] = [main, alt]
    return project


# --- Collecting (D-61) ---------------------------------------------------------------------


def stored_files(asset):
    """(stored path, [frame numbers]) in the order a package lists them."""
    if asset["kind"] == "still":
        return [(asset["path"], [])]
    order = []
    for number in sorted(asset["frames"], key=int):
        path = asset["frames"][number]
        for entry in order:
            if entry[0] == path:
                entry[1].append(int(number))
                break
        else:
            order.append((path, [int(number)]))
    return order


def collect(project, source):
    folders = set()
    manifest_assets = []
    packaged = copy.deepcopy(project)
    places = {}
    for asset, out_asset in zip(project["assets"], packaged["assets"]):
        folder = unique(safe(asset["id"]), folders)
        names = set()
        redistribute = asset.get("redistribute", True)
        files = []
        moved = {}
        for stored, numbers in stored_files(asset):
            name = unique(safe(stored.replace("\\", "/").split("/")[-1]), names)
            where = f"{MEDIA}/{folder}/{name}"
            moved[stored] = where
            data = (source / stored).read_bytes() if (source / stored).is_file() else None
            if data is None:
                status = "missing"
            elif redistribute:
                status = "copied"
            else:
                status = "excluded"
            files.append({
                "frames": numbers,
                "path": where,
                "status": status,
                "bytes": None if data is None else len(data),
                "sha256": None if data is None else hashlib.sha256(data).hexdigest(),
            })
        if "path" in out_asset:
            out_asset["path"] = moved[out_asset["path"]]
        for k in out_asset.get("frames", {}):
            out_asset["frames"][k] = moved[out_asset["frames"][k]]
        places[asset["id"]] = {k: out_asset[k] for k in ("path", "frames") if k in out_asset}
        used_by = [{"composition": c["id"], "layer": l["id"]}
                   for c in project["compositions"] for l in c["layers"]
                   if l["asset_id"] == asset["id"]]
        manifest_assets.append({
            "id": asset["id"],
            "name": asset["name"],
            "kind": asset["kind"],
            "redistribute": redistribute,
            "used_by": used_by,
            "files": files,
        })
    manifest = {"format": FORMAT, "version": 1, "project": "shot.json",
                "assets": manifest_assets}
    return manifest, places


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


# --- Checking (D-61) -----------------------------------------------------------------------


def check(manifest, present):
    """What checking answers for each listed file, given what is in the package now.

    `present` maps a package path to its bytes, for the files there."""
    rows = []
    for asset in manifest["assets"]:
        for f in asset["files"]:
            data = present.get(f["path"])
            if data is None:
                answer = "excluded" if f["status"] == "excluded" else "missing"
            elif f["sha256"] is None:
                answer = "unverified"
            elif len(data) == f["bytes"] and hashlib.sha256(data).hexdigest() == f["sha256"]:
                answer = "ok"
            else:
                answer = "changed"
            rows.append({"path": f["path"], "answer": answer})
    return rows


def main():
    if SOURCE.exists():
        shutil.rmtree(SOURCE)
    for path, colour in FILES.items():
        (SOURCE / path).parent.mkdir(parents=True, exist_ok=True)
        (SOURCE / path).write_bytes(png(colour))
    project = shot()
    (SOURCE / "shot.json").write_bytes(dump(project).encode("utf-8"))

    manifest, places = collect(project, SOURCE)
    (OUT / "expected_manifest.json").write_bytes(dump(manifest).encode("utf-8"))

    packaged = {f["path"]: (SOURCE / src).read_bytes()
                for a, asset in zip(manifest["assets"], project["assets"])
                for f, (src, _) in zip(a["files"], stored_files(asset))
                if f["status"] == "copied"}
    licensed = (SOURCE / "licensed/sheet.png").read_bytes()
    first = "media/asset-cel/cel_0001.png"
    tampered = dict(packaged)
    tampered[first] = bytes([packaged[first][0] ^ 1]) + packaged[first][1:]
    removed = {k: v for k, v in packaged.items() if k != first}
    supplied = dict(packaged, **{"media/asset-licensed/sheet.png": licensed})
    wrong = dict(packaged, **{"media/asset-licensed/sheet.png": packaged[first]})
    gone = dict(packaged, **{"media/asset-gone/gone_0001.png": packaged[first]})
    situations = [
        ("as collected", packaged),
        ("one byte of cel_0001.png changed", tampered),
        ("cel_0001.png deleted", removed),
        ("the licensed sheet supplied by the recipient", supplied),
        ("a different file put where the licensed sheet goes", wrong),
        ("a file put where the missing drawing goes", gone),
    ]
    expected = {
        "folders": [a["files"][0]["path"].split("/")[1] for a in manifest["assets"]],
        "copied": sorted(packaged),
        "places": places,
        "checks": [{"situation": s, "rows": check(manifest, p)} for s, p in situations],
    }
    (OUT / "expected_package.json").write_bytes(dump(expected).encode("utf-8"))

    # The tables document 25 pins.
    print("| Asset | Folder | Files | Status |")
    print("|---|---|---|---|")
    for a in manifest["assets"]:
        print(f"| `{a['id']}` | `{a['files'][0]['path'].split('/')[1]}` | "
              + ", ".join(f"`{f['path'].split('/')[-1]}` {f['frames'] or ''}".strip()
                          for f in a["files"])
              + " | " + ", ".join(f["status"] for f in a["files"]) + " |")
    print()
    for s in expected["checks"]:
        odd = [f"{r['path'].split('/', 1)[1]}: {r['answer']}" for r in s["rows"]
               if r["answer"] != "ok"]
        print(f"| {s['situation']} | {'; '.join(odd)} |")


if __name__ == "__main__":
    main()
