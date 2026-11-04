"""D-1008: Color Grade's colour wheels you can drag, the reference for the page's arithmetic.

Writes `Fixtures/color_wheels/expected_wheel_drags.json`: where a press or drag on a wheel puts
its hue and amount, and where a wheel's hue and amount put its dot. Run before the page code;
B-888's page check runs every case through the page's own `wheelAt` and `wheelDot`.

A wheel is a disc of radius 1, x to the right, y up. The hue h sits at the angle h + 103 degrees,
counter-clockwise from the right, so red (0) is up and to the left, near where a vectorscope and
Lumetri's wheels put it. A point (x, y): the amount is 100 times its distance from the middle,
rounded and held to 100; the hue is its angle less 103, rounded, 0 to 359. An amount of 0 keeps
the hue as it was. The dot: at the hue's angle, the amount over 100 from the middle.

  python -I tools/color_wheels_reference.py
"""
import json
import math
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_wheels"
RED = 103


def rnd(x):
    return math.floor(x + 0.5)


def wheel_at(x, y, hue):
    amount = min(100, rnd(100 * math.hypot(x, y)))
    if amount == 0:
        return {"hue": hue, "amount": 0}
    return {"hue": rnd(math.degrees(math.atan2(y, x)) - RED) % 360, "amount": amount}


def wheel_dot(hue, amount):
    a = math.radians(hue + RED)
    return {"x": round(amount / 100 * math.cos(a), 6), "y": round(amount / 100 * math.sin(a), 6)}


AT = [
    ("The middle: no push, the hue kept.", 0, 0, 40),
    ("A touch off the middle, under half a step: still no push, the hue kept.", 0.004, 0, 40),
    ("Up and to the left at the edge: red, full.", math.cos(math.radians(103)), math.sin(math.radians(103)), 0),
    ("Straight up, half way: hue 347, amount 50.", 0, 0.5, 0),
    ("Straight down, a quarter: hue 167, amount 25.", 0, -0.25, 0),
    ("To the right, 0.3: hue 257, amount 30.", 0.3, 0, 0),
    ("To the left, 0.7: hue 77, amount 70.", -0.7, 0, 0),
    ("Past the edge: held at 100.", 1.5, 1.5, 0),
    ("A blue-ish point.", 0.42, -0.31, 0),
    ("A hue that rounds across 0 to 359.5, so 0.", math.cos(math.radians(102.5)), math.sin(math.radians(102.5)), 0),
]
DOTS = [
    ("Hue 0, amount 100: up and to the left.", 0, 100),
    ("Hue 120, amount 50.", 120, 50),
    ("Hue 257, amount 30: to the right.", 257, 30),
    ("Amount 0: the middle.", 200, 0),
]


def main():
    cases = {
        "rule": "D-1008",
        "at": [{"says": s, "x": x, "y": y, "hue_before": h, "to": wheel_at(x, y, h)} for s, x, y, h in AT],
        "dot": [{"says": s, "hue": h, "amount": a, "to": wheel_dot(h, a)} for s, h, a in DOTS],
    }
    # The claims the cases make.
    t = [c["to"] for c in cases["at"]]
    assert t[0] == {"hue": 40, "amount": 0} and t[1] == {"hue": 40, "amount": 0}
    assert t[2] == {"hue": 0, "amount": 100} and t[3] == {"hue": 347, "amount": 50}
    assert t[4] == {"hue": 167, "amount": 25} and t[5] == {"hue": 257, "amount": 30}
    assert t[6] == {"hue": 77, "amount": 70} and t[7]["amount"] == 100 and t[9]["hue"] == 0
    for c in cases["dot"]:   # a dot read back gives its own hue and amount
        if c["amount"]:
            assert wheel_at(c["to"]["x"], c["to"]["y"], -1) == {"hue": c["hue"], "amount": c["amount"]}
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "expected_wheel_drags.json").write_bytes((json.dumps(cases, indent=1) + "\n").encode("utf-8"))
    print("checked")


if __name__ == "__main__":
    main()
