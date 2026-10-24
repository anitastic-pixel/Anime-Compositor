"""Bounded expressions, worked a second way.

D-59 proposes the answer to D-10: expressions are a small language this project specifies and
evaluates itself, not an embedded scripting runtime. Its words are After Effects' where After
Effects has one - `time`, `value`, `wiggle`, `loopOut`, `valueAtTime`, `posterizeTime`,
`linear`, `ease`, `random`, `seedRandom`, `thisComp.layer(...)` - and its rules are written in
document 09. This file is a whole implementation of those rules in Python: a reader, an
evaluator and the property lookups, and it prints the numbers document 25 pins.

**It shares nothing with the build.** The build's evaluator is Rust and is written from
document 09; this one is written from the same document. The only thing the two must agree on
bit for bit is the noise behind `wiggle` and `random`, and that is specified to the operation
in document 09 so that it can be: SplitMix64, FNV-1a and a quintic fade, all integer or
ordinary f64 arithmetic in a stated order.

It also writes `Fixtures/projects/expression_project.json`, the file B-14b must open and save
unchanged, and D-766's `Fixtures/projects/expression_controls_project.json`, whose layers carry
After Effects' Expression Controls for `effect(...)` to read.

Fixtures are read-only to implementation work: this file is run when the specification
changes, and never to make a build pass.

    python tools/expression_reference.py
"""

import copy
import json
import math
import struct
import sys
from pathlib import Path

from ease_reference import ease as bezier

MASK = (1 << 64) - 1
MAX_TEXT_BYTES = 4096
MAX_STEPS = 10_000
MAX_DEPTH = 16
MAX_OCTAVES = 16

# The names an expression may start from. Anything else is EXPRESSION_SYNTAX, which is how
# `require`, `eval`, `system.callSystem` and every other door out of the sandbox are closed:
# the language has no word for them.
ROOTS = {"time", "value", "thisComp", "thisLayer", "thisProperty", "Math", "wiggle", "loopOut",
         "linear", "ease", "clamp", "random", "seedRandom", "posterizeTime", "effect"}
IGNORED_DECLARATIONS = {"var", "let", "const"}
# D-766: words of the language that are not values to start from.
KEYWORDS = {"if", "else", "true", "false"}

# D-766: After Effects' six Expression Controls: the effect's name and match name, then its one
# setting's name and saved key, and the range an expression's number is held in.
CONTROLS = {
    "core.slider_control": ("Slider Control", "ADBE Slider Control", "Slider", "slider",
                            -1e6, 1e6),
    "core.checkbox_control": ("Checkbox Control", "ADBE Checkbox Control", "Checkbox",
                              "checkbox", 0.0, 1.0),
    "core.angle_control": ("Angle Control", "ADBE Angle Control", "Angle", "angle",
                           -1e9, 1e9),
    "core.point_control": ("Point Control", "ADBE Point Control", "Point", "point", None, None),
    "core.color_control": ("Color Control", "ADBE Color Control", "Color", "color", None, None),
    "core.layer_control": ("Layer Control", "ADBE Layer Control", "Layer", "layer", None, None),
}
# The display names of the other effects the fixture project holds.
OTHER_EFFECTS = {"core.gaussian_blur": "Gaussian Blur"}

# Which properties an expression may sit on or read, and how many numbers each holds.
LAYER_PROPS = {"anchor": 2, "position": 2, "scale": 2, "rotation": 1, "opacity": 1, "depth": 1}
LAYER_ALIASES = {"anchorPoint": "anchor"}
CAMERA_PROPS = {"position": 2, "depth": 1, "zoom": 1}


class ExprError(Exception):
    def __init__(self, code, message):
        super().__init__(message)
        self.code = code
        self.message = message


def syntax(message):
    return ExprError("EXPRESSION_SYNTAX", message)


def type_error(message):
    return ExprError("EXPRESSION_TYPE", message)


# --- noise -------------------------------------------------------------------------------------

def splitmix64(x):
    z = (x + 0x9E3779B97F4A7C15) & MASK
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
    return z ^ (z >> 31)


def mix(a, b):
    return splitmix64(a ^ splitmix64(b))


def unit(x):
    """[0, 1) from the top 53 bits."""
    return (x >> 11) * 2.0 ** -53


def fnv1a64(text):
    h = 0xCBF29CE484222325
    for byte in text.encode("utf-8"):
        h = ((h ^ byte) * 0x100000001B3) & MASK
    return h


def as_u64(n):
    """A signed integer as its 64-bit two's complement."""
    return n & MASK


def bits(f):
    return struct.unpack("<Q", struct.pack("<d", f))[0]


def lattice(seed, stream, i):
    return 2.0 * unit(mix(mix(seed, stream), as_u64(i))) - 1.0


def noise(seed, stream, x):
    i = math.floor(x)
    s = x - i
    a = lattice(seed, stream, i)
    b = lattice(seed, stream, i + 1)
    w = s * s * s * (s * (s * 6.0 - 15.0) + 10.0)
    return a + (b - a) * w


def truthy(v):
    """D-766: a condition holds when its number is neither 0 nor NaN, as JavaScript has it."""
    return v != 0 and not math.isnan(v)


def show_arg(v):
    return f"\"{v}\"" if isinstance(v, str) else f"{v:g}"


# --- reading -----------------------------------------------------------------------------------

def tokens(text):
    out, i = [], 0
    while i < len(text):
        c = text[i]
        if c in " \t\r":
            i += 1
        elif c == "\n":
            out.append(("nl", "\n"))
            i += 1
        elif text.startswith("//", i):
            while i < len(text) and text[i] != "\n":
                i += 1
        elif c.isdigit() or (c == "." and i + 1 < len(text) and text[i + 1].isdigit()):
            j = i
            while j < len(text) and (text[j].isdigit() or text[j] == "."):
                j += 1
            if j < len(text) and text[j] in "eE":
                k = j + 1
                if k < len(text) and text[k] in "+-":
                    k += 1
                if k < len(text) and text[k].isdigit():
                    j = k
                    while j < len(text) and text[j].isdigit():
                        j += 1
            try:
                out.append(("num", float(text[i:j])))
            except ValueError:
                raise syntax(f"\"{text[i:j]}\" is not a number")
            i = j
        elif c.isascii() and (c.isalpha() or c == "_"):
            j = i
            while j < len(text) and text[j].isascii() and (text[j].isalnum() or text[j] == "_"):
                j += 1
            out.append(("name", text[i:j]))
            i = j
        elif c in "\"'":
            j = text.find(c, i + 1)
            if j < 0 or "\n" in text[i:j]:
                raise syntax("a quoted text is not closed")
            out.append(("str", text[i + 1:j]))
            i = j + 1
        elif text.startswith(("===", "!=="), i):
            out.append(("op", text[i:i + 2]))
            i += 3
        elif text.startswith(("==", "!=", "<=", ">=", "&&", "||"), i):
            out.append(("op", text[i:i + 2]))
            i += 2
        elif c in "+-*/%()[],.;=<>!{}":
            out.append(("op", c))
            i += 1
        else:
            raise syntax(f"\"{c}\" is not part of the language")
    out.append(("end", None))
    return out


class Reader:
    """Statements, then expressions by precedence. Every node is a tuple."""

    def __init__(self, text):
        if len(text.encode("utf-8")) > MAX_TEXT_BYTES:
            raise syntax(f"an expression is at most {MAX_TEXT_BYTES} bytes long")
        self.toks = tokens(text)
        self.at = 0
        self.locals = set()

    def peek(self, kind=None, value=None):
        k, v = self.toks[self.at]
        return (kind is None or k == kind) and (value is None or v == value)

    def take(self, kind=None, value=None):
        if not self.peek(kind, value):
            k, v = self.toks[self.at]
            raise syntax(f"expected {value or kind}, found {v if v is not None else 'the end'}")
        tok = self.toks[self.at]
        self.at += 1
        return tok

    def ended(self, closer):
        """Whether the statement just read is ended: by a line, a ";", the closer, or its "}"."""
        return (self.peek("nl") or self.peek("op", ";") or self.peek(*closer)
                or self.toks[self.at - 1] == ("op", "}"))

    def program(self):
        stmts = []
        while True:
            while self.peek("nl") or self.peek("op", ";"):
                self.at += 1
            if self.peek("end"):
                break
            stmts.append(self.statement())
            if not self.ended(("end",)):
                raise syntax(f"expected the end of a line, found {self.toks[self.at][1]}")
        if not stmts:
            raise syntax("the expression is empty")
        return stmts

    # D-766: `{ statements }`, or one statement.
    def block(self):
        while self.peek("nl"):
            self.at += 1
        if not self.peek("op", "{"):
            return [self.statement()]
        self.take()
        stmts = []
        while True:
            while self.peek("nl") or self.peek("op", ";"):
                self.at += 1
            if self.peek("op", "}"):
                self.take()
                return stmts
            if self.peek("end"):
                raise syntax("a \"{\" is not closed")
            stmts.append(self.statement())
            if not self.ended(("op", "}")):
                raise syntax(f"expected the end of a line, found {self.toks[self.at][1]}")

    # D-766: `if (condition) block`, then `else block` if wanted, on the same line or a later one.
    def if_statement(self):
        self.take("name", "if")
        self.take("op", "(")
        cond = self.expr()
        self.take("op", ")")
        then = self.block()
        back = self.at
        while self.peek("nl") or self.peek("op", ";"):
            self.at += 1
        if self.peek("name", "else"):
            self.take()
            return ("if", cond, then, self.block())
        self.at = back
        return ("if", cond, then, [])

    def statement(self):
        if self.peek("name", "if"):
            return self.if_statement()
        if self.peek("name") and self.toks[self.at][1] in IGNORED_DECLARATIONS:
            self.at += 1
            if not (self.peek("name") and self.toks[self.at + 1] == ("op", "=")):
                raise syntax("a declaration names a value and gives it one")
        if self.peek("name") and self.toks[self.at + 1] == ("op", "="):
            name = self.take("name")[1]
            self.take("op", "=")
            if name in ROOTS or name in IGNORED_DECLARATIONS or name in KEYWORDS:
                raise syntax(f"\"{name}\" is a word of the language and cannot be given a value")
            node = ("let", name, self.expr())
            self.locals.add(name)
            return node
        return ("do", self.expr())

    # D-766: || then && then == != then < <= > >=, each looser than the arithmetic.
    def expr(self):
        node = self.both()
        while self.peek("op", "||"):
            self.take()
            node = ("or", node, self.both())
        return node

    def both(self):
        node = self.equality()
        while self.peek("op", "&&"):
            self.take()
            node = ("and", node, self.equality())
        return node

    def equality(self):
        node = self.relation()
        while self.peek("op", "==") or self.peek("op", "!="):
            op = self.take()[1]
            node = ("cmp", op, node, self.relation())
        return node

    def relation(self):
        node = self.additive()
        while any(self.peek("op", o) for o in ("<", "<=", ">", ">=")):
            op = self.take()[1]
            node = ("cmp", op, node, self.additive())
        return node

    def additive(self):
        node = self.term()
        while self.peek("op", "+") or self.peek("op", "-"):
            op = self.take()[1]
            node = ("bin", op, node, self.term())
        return node

    def term(self):
        node = self.unary()
        while self.peek("op", "*") or self.peek("op", "/") or self.peek("op", "%"):
            op = self.take()[1]
            node = ("bin", op, node, self.unary())
        return node

    def unary(self):
        if self.peek("op", "!"):
            self.take()
            return ("not", self.unary())
        if self.peek("op", "-") or self.peek("op", "+"):
            op = self.take()[1]
            return ("neg", self.unary()) if op == "-" else self.unary()
        return self.postfix()

    def postfix(self):
        node = self.primary()
        while True:
            if self.peek("op", "."):
                self.take()
                node = ("get", node, self.take("name")[1])
            elif self.peek("op", "("):
                self.take()
                args = []
                if not self.peek("op", ")"):
                    args.append(self.expr())
                    while self.peek("op", ","):
                        self.take()
                        args.append(self.expr())
                self.take("op", ")")
                node = ("call", node, args)
            elif self.peek("op", "["):
                self.take()
                index = self.expr()
                self.take("op", "]")
                node = ("index", node, index)
            else:
                return node

    def primary(self):
        kind, v = self.toks[self.at]
        if kind == "num":
            self.at += 1
            return ("num", v)
        if kind == "str":
            self.at += 1
            return ("str", v)
        if kind == "name" and v in ("true", "false"):
            self.at += 1
            return ("num", 1.0 if v == "true" else 0.0)
        if kind == "name":
            self.at += 1
            if v not in ROOTS and v not in self.locals:
                raise syntax(f"\"{v}\" is not a word this language knows")
            return ("name", v)
        if kind == "op" and v == "(":
            self.at += 1
            node = self.expr()
            self.take("op", ")")
            return node
        if kind == "op" and v == "[":
            self.at += 1
            items = [self.expr()]
            while self.peek("op", ","):
                self.take()
                items.append(self.expr())
            self.take("op", "]")
            return ("array", items)
        raise syntax(f"expected a value, found {v if v is not None else 'the end'}")


# --- the project -------------------------------------------------------------------------------

def key_value(prop, frame):
    """Document 20: base with no keys, held beyond the ends, hold, linear or eased between."""
    keys = prop["keyframes"]
    if not keys:
        return prop["base"]
    if frame <= keys[0]["frame"]:
        return keys[0]["value"]
    for a, b in zip(keys, keys[1:]):
        if "spatial" in a or "spatial" in b:
            raise SystemExit("this reference does not walk motion paths; no fixture uses one")
        if frame < b["frame"]:
            if a["interp"] == "hold":
                return a["value"]
            p = (frame - a["frame"]) / (b["frame"] - a["frame"])
            f = bezier(a["ease"], p) if a["interp"] == "ease" else p
            if isinstance(a["value"], list):
                return [x + (y - x) * f for x, y in zip(a["value"], b["value"])]
            return a["value"] + (b["value"] - a["value"]) * f
    return keys[-1]["value"]


class Project:
    def __init__(self, data, comp_index=0):
        self.comp = data["compositions"][comp_index]
        self.layers = {layer["id"]: layer for layer in self.comp["layers"]}
        rate = self.comp["frame_rate"]
        self.num, self.den = rate["numerator"], rate["denominator"]

    def property(self, target, prop):
        if target == "camera":
            camera = self.comp.get("camera")
            if camera is None:
                w, h = self.comp["width"], self.comp["height"]
                zoom = w * 50 / 36
                camera = {"position": {"base": [w / 2, h / 2], "keyframes": []},
                          "depth": {"base": -zoom, "keyframes": []},
                          "zoom": {"base": zoom, "keyframes": []}}
            return camera[prop]
        layer = self.layers.get(target)
        if layer is None:
            raise ExprError("EXPRESSION_REFERENCE_MISSING",
                            f"there is no layer with the id \"{target}\"")
        if isinstance(prop, tuple):
            # D-766: ("fx", effect index, saved key), one Expression Control's setting.
            record = layer["effects"][prop[1]]["parameters"][prop[2]]
            if isinstance(record, dict):
                return record
            return {"base": record, "keyframes": []}
        if prop == "depth":
            return layer.get("depth") or {"base": 0, "keyframes": []}
        return layer["transform"][prop]

    def seed_name(self, target, prop):
        owner = f"{self.comp['id']}/camera" if target == "camera" else target
        if isinstance(prop, tuple):
            # D-291's name for an effect setting: the effect's instance id, then the setting.
            fx = self.layers[target]["effects"][prop[1]]
            return f"{owner}/{fx['instance_id']}/{prop[2]}"
        return f"{owner}/{prop}"

    def dims(self, target, prop):
        if isinstance(prop, tuple):
            return 1
        return (CAMERA_PROPS if target == "camera" else LAYER_PROPS)[prop]

    # Opacity is 0 to 1 in the file and 0 to 100 in the window and in an expression. Every
    # other property is read in the file's own units: scale is already a percentage there.
    def to_expr(self, prop, v):
        return v * 100 if prop == "opacity" else v

    def to_file(self, prop, v):
        return v / 100 if prop == "opacity" else v

    def pre(self, target, prop, frame):
        return self.to_expr(prop, key_value(self.property(target, prop), frame))

    def frame_of(self, seconds):
        x = seconds * self.num / self.den
        return int(math.floor(abs(x) + 0.5)) * (1 if x >= 0 else -1)


class Run:
    """One top-level evaluation: its step budget and the properties being evaluated."""

    def __init__(self, project):
        self.project = project
        self.steps = 0
        self.stack = []

    def final(self, target, prop, frame):
        """A property's value after its expression, in expression units."""
        record = self.project.property(target, prop)
        expr = record.get("expression")
        if not expr or not expr["enabled"]:
            return self.project.pre(target, prop, frame)
        here = (target, prop, frame)
        if here in self.stack:
            raise ExprError("EXPRESSION_CYCLE",
                            f"{target} {prop} depends on itself at frame {frame}")
        if len(self.stack) >= MAX_DEPTH:
            raise ExprError("EXPRESSION_TIMEOUT",
                            f"more than {MAX_DEPTH} expressions deep")
        self.stack.append(here)
        try:
            return Scope(self, target, prop, frame).outcome(expr["text"])
        finally:
            self.stack.pop()

    def setting(self, ref, frame):
        """D-766: an Expression Control's setting at a frame, after its own expression."""
        fx = self.project.layers[ref.target]["effects"][ref.index]
        _, _, _, key_name, lo, hi = CONTROLS[fx["type_id"]]
        prop = ("fx", ref.index, key_name)
        record = self.project.property(ref.target, prop)
        expr = record.get("expression")
        if expr and expr["enabled"]:
            here = (ref.target, prop, frame)
            if here in self.stack:
                raise ExprError("EXPRESSION_CYCLE",
                                f"{key_name} depends on itself at frame {frame}")
            if len(self.stack) >= MAX_DEPTH:
                raise ExprError("EXPRESSION_TIMEOUT",
                                f"more than {MAX_DEPTH} expressions deep")
            self.stack.append(here)
            try:
                v = Scope(self, ref.target, prop, frame).outcome(expr["text"])
            finally:
                self.stack.pop()
            v = min(hi, max(lo, v))
        else:
            v = key_value(record, frame)
        if key_name == "checkbox":
            return 1.0 if v != 0 else 0.0
        if key_name == "color":
            return list(v) + [1.0]
        return v


class Effects:
    """D-766: a layer's effects, which `effect(name or number)` picks from."""

    def __init__(self, target):
        self.target = target


class FxRef:
    def __init__(self, target, index):
        self.target, self.index = target, index


class SettingRef:
    def __init__(self, target, index):
        self.target, self.index = target, index


class PropRef:
    def __init__(self, target, prop, own=False):
        self.target, self.prop, self.own = target, prop, own


class LayerRef:
    def __init__(self, target):
        self.target = target


class Namespace:
    def __init__(self, name):
        self.name = name


class Func:
    def __init__(self, name):
        self.name = name


class Scope:
    def __init__(self, run, target, prop, frame):
        self.run, self.p = run, run.project
        self.target, self.prop, self.frame = target, prop, frame
        self.dims = self.p.dims(target, prop)
        self.posterize = None
        self.user_seed = 0
        self.timeless = False
        self.wiggles = 0
        self.randoms = 0
        self.locals = {}

    # The value the expression ends on, checked against the property, in expression units.
    def statements(self, stmts):
        """The value of the last statement that gives one; D-766: an if gives its branch's."""
        result = None
        for stmt in stmts:
            if stmt[0] == "let":
                self.locals[stmt[1]] = self.number_or_array(self.eval(stmt[2]))
            elif stmt[0] == "if":
                self.step()
                cond = self.number(self.eval(stmt[1]), "an if's condition")
                result = self.statements(stmt[2] if truthy(cond) else stmt[3])
            else:
                result = self.eval(stmt[1])
        return result

    def outcome(self, text):
        result = self.plain(self.statements(Reader(text).program()))
        if result is None:
            raise type_error("the expression ends without a value")
        if isinstance(result, str):
            raise type_error("the expression ends on a text, not a number")
        if not isinstance(result, (float, int, list)):
            raise type_error("the expression ends on something that is not a number")
        if self.dims == 1 and isinstance(result, list):
            raise type_error(f"{self.prop} is one number and the expression gives {len(result)}")
        if self.dims > 1 and not (isinstance(result, list) and len(result) == self.dims):
            got = len(result) if isinstance(result, list) else 1
            raise type_error(f"{self.prop} is {self.dims} numbers and the expression gives {got}")
        for v in (result if isinstance(result, list) else [result]):
            if not math.isfinite(v):
                raise type_error("the expression gives a number that is not finite")
        if self.prop == "zoom" and result <= 0:
            raise type_error("a camera's zoom must be greater than zero")
        if self.prop == "opacity":
            return min(100.0, max(0.0, result))
        return result

    def step(self):
        self.run.steps += 1
        if self.run.steps > MAX_STEPS:
            raise ExprError("EXPRESSION_TIMEOUT", f"more than {MAX_STEPS} steps")

    def time(self):
        f, num, den = self.frame, self.p.num, self.p.den
        if self.posterize is None:
            return f * den / num
        return math.floor(f * den * self.posterize / num + 1e-9) / self.posterize

    def seed(self):
        return mix(fnv1a64(self.p.seed_name(self.target, self.prop)), as_u64(self.user_seed))

    def value_of(self, ref, frame):
        if isinstance(ref, SettingRef):
            return self.run.setting(ref, frame)
        if ref.own:
            return self.p.pre(ref.target, ref.prop, frame)
        return self.run.final(ref.target, ref.prop, frame)

    def plain(self, v):
        return self.value_of(v, self.frame) if isinstance(v, (PropRef, SettingRef)) else v

    def number_or_array(self, v):
        v = self.plain(v)
        if isinstance(v, (float, int, list)) and not isinstance(v, bool):
            return v
        raise type_error("only a number or a list of numbers can be used here")

    def number(self, v, what):
        v = self.plain(v)
        if isinstance(v, (float, int)) and not isinstance(v, bool):
            return v
        raise type_error(f"{what} must be one number")

    def eval(self, node):
        self.step()
        kind = node[0]
        if kind == "num":
            return node[1]
        if kind == "str":
            return node[1]
        if kind == "name":
            name = node[1]
            if name in self.locals:
                return self.locals[name]
            if name == "time":
                return self.time()
            if name == "value":
                return self.p.pre(self.target, self.prop, self.frame)
            if name == "thisProperty":
                if isinstance(self.prop, tuple):
                    raise syntax("on an effect's setting, use value for the setting's own number")
                return PropRef(self.target, self.prop, own=True)
            if name == "thisLayer":
                if self.target == "camera":
                    raise syntax("the camera is not a layer, so it has no thisLayer")
                return LayerRef(self.target)
            if name == "effect":
                if self.target == "camera":
                    raise syntax("the camera is not a layer, so it has no effects")
                return Effects(self.target)
            if name in ("thisComp", "Math"):
                return Namespace(name)
            if name in ROOTS:
                return Func(name)
            # D-766: a name given its value in a branch that was not taken.
            raise type_error(f"\"{name}\" has no value here")
        if kind == "not":
            return 0.0 if truthy(self.number(self.eval(node[1]), "what \"!\" turns")) else 1.0
        if kind == "cmp":
            a, b = self.plain(self.eval(node[2])), self.plain(self.eval(node[3]))
            if not all(isinstance(v, (float, int)) for v in (a, b)):
                raise type_error(f"\"{node[1]}\" compares two numbers")
            op = node[1]
            return 1.0 if (a == b if op == "==" else a != b if op == "!=" else
                           a < b if op == "<" else a <= b if op == "<=" else
                           a > b if op == ">" else a >= b) else 0.0
        if kind in ("and", "or"):
            a = self.number(self.eval(node[1]), f"the left of \"{'&&' if kind == 'and' else '||'}\"")
            if truthy(a) == (kind == "or"):
                return a
            return self.number_or_array(self.eval(node[2]))
        if kind == "array":
            items = [self.number(self.eval(n), "each item of a list") for n in node[1]]
            if len(items) > 4:
                raise type_error("a list holds at most four numbers")
            return items
        if kind == "neg":
            v = self.number_or_array(self.eval(node[1]))
            return [-x for x in v] if isinstance(v, list) else -v
        if kind == "bin":
            return self.arith(node[1], self.eval(node[2]), self.eval(node[3]))
        if kind == "index":
            v = self.plain(self.eval(node[1]))
            i = self.number(self.eval(node[2]), "a position in a list")
            if not isinstance(v, list):
                raise type_error("only a list can be indexed")
            if i != math.floor(i) or not 0 <= i < len(v):
                raise type_error(f"a list of {len(v)} has no item {i:g}")
            return v[int(i)]
        if kind == "get":
            return self.member(self.eval(node[1]), node[2])
        if kind == "call":
            return self.call(self.eval(node[1]), node[2])
        raise AssertionError(kind)

    def arith(self, op, a, b):
        a, b = self.number_or_array(a), self.number_or_array(b)
        la, lb = isinstance(a, list), isinstance(b, list)
        if not la and not lb:
            if op == "+":
                return a + b
            if op == "-":
                return a - b
            if op == "*":
                return a * b
            if b == 0:
                return math.copysign(math.inf, a) * math.copysign(1, b) if a != 0 and op == "/" \
                    else math.nan
            return a / b if op == "/" else math.fmod(a, b)
        if op in "+-" and la and lb:
            if len(a) != len(b):
                raise type_error(f"a list of {len(a)} and a list of {len(b)} cannot be added")
            return [x + y if op == "+" else x - y for x, y in zip(a, b)]
        if op == "*" and la != lb:
            n, v = (b, a) if la else (a, b)
            return [x * n for x in v]
        if op == "/" and la and not lb:
            return [self.arith("/", x, b) for x in a]
        raise type_error(f"\"{op}\" cannot be used between "
                         f"{'a list' if la else 'a number'} and {'a list' if lb else 'a number'}")

    def member(self, obj, name):
        if isinstance(obj, Namespace) and obj.name == "thisComp":
            if name == "layer":
                return Func("layer")
            if name == "activeCamera":
                return LayerRef("camera")
            if name in ("width", "height"):
                return self.p.comp[name]
            if name == "frameDuration":
                return self.p.den / self.p.num
        if isinstance(obj, Namespace) and obj.name == "Math":
            if name == "PI":
                return math.pi
            if name in ("sin", "cos", "tan", "abs", "floor", "ceil", "round", "sqrt", "pow",
                        "min", "max"):
                return Func("Math." + name)
        if isinstance(obj, LayerRef):
            if name == "transform" and obj.target != "camera":
                return obj
            if name == "effect" and obj.target != "camera":
                return Effects(obj.target)
            name = LAYER_ALIASES.get(name, name) if obj.target != "camera" else name
            props = CAMERA_PROPS if obj.target == "camera" else LAYER_PROPS
            if name in props:
                return PropRef(obj.target, name)
        if isinstance(obj, (PropRef, SettingRef)):
            if name == "value":
                return self.value_of(obj, self.frame)
            if name == "valueAtTime":
                return ("method", obj)
        raise syntax(f"\"{name}\" is not something this can be asked for")

    def call(self, fn, arg_nodes):
        if isinstance(fn, tuple) and fn[0] == "method":
            if len(arg_nodes) != 1:
                raise type_error("valueAtTime takes one time, in seconds")
            t = self.number(self.eval(arg_nodes[0]), "a time")
            if not math.isfinite(t):
                raise type_error("a time must be finite")
            return self.value_of(fn[1], self.p.frame_of(t))
        if isinstance(fn, Effects):
            return self.pick_effect(fn.target, arg_nodes)
        if isinstance(fn, FxRef):
            return self.pick_setting(fn, arg_nodes)
        if not isinstance(fn, Func):
            raise syntax("only a function can be called")
        name = fn.name
        args = [self.eval(n) for n in arg_nodes]
        if name == "layer":
            if len(args) != 1 or not isinstance(args[0], str):
                raise type_error("thisComp.layer takes a layer's id, in quotes")
            self.p.property(args[0], "position")  # a missing layer is reported here
            return LayerRef(args[0])
        if any(isinstance(a, str) for a in args) and name != "loopOut":
            raise type_error(f"{name} does not take a text")
        if name.startswith("Math."):
            nums = [self.number(a, f"{name}'s argument") for a in args]
            op = name[5:]
            want = {"pow": 2}.get(op, None if op in ("min", "max") else 1)
            if want is not None and len(nums) != want or (want is None and not nums):
                raise type_error(f"{name} takes {want or 'at least one'} number")
            if op == "round":
                return math.floor(nums[0] + 0.5)
            if op == "abs":
                return abs(nums[0])
            if op in ("floor", "ceil"):
                return getattr(math, op)(nums[0])
            if op == "sqrt":
                return math.sqrt(nums[0]) if nums[0] >= 0 else math.nan
            if op == "pow":
                try:
                    return math.pow(*nums)
                except (ValueError, OverflowError):
                    return math.nan
            if op in ("min", "max"):
                return (min if op == "min" else max)(nums)
            return getattr(math, op)(nums[0])
        if name == "posterizeTime":
            if len(args) != 1:
                raise type_error("posterizeTime takes one frame rate")
            p = self.number(args[0], "a frame rate")
            if not (math.isfinite(p) and p > 0):
                raise type_error("posterizeTime needs a frame rate greater than zero")
            self.posterize = p
            return None
        if name == "seedRandom":
            if len(args) not in (1, 2):
                raise type_error("seedRandom takes a seed and, if wanted, timeless")
            s = self.number(args[0], "a seed")
            if not math.isfinite(s):
                raise type_error("a seed must be finite")
            self.user_seed = math.floor(s)
            self.timeless = len(args) == 2 and self.number(args[1], "timeless") != 0
            return None
        if name == "wiggle":
            return self.wiggle(args)
        if name == "random":
            return self.random(args)
        if name in ("linear", "ease"):
            return self.remap(name, args)
        if name == "clamp":
            if len(args) != 3:
                raise type_error("clamp takes a value, a lowest and a highest")
            v = self.number_or_array(args[0])
            lo, hi = self.number(args[1], "the lowest"), self.number(args[2], "the highest")
            one = lambda x: min(hi, max(lo, x))
            return [one(x) for x in v] if isinstance(v, list) else one(v)
        if name == "loopOut":
            return self.loop_out(args)
        raise syntax(f"\"{name}\" cannot be called")

    def one_arg(self, arg_nodes, what):
        if len(arg_nodes) != 1:
            raise type_error(f"{what} takes a name, in quotes, or a number")
        v = self.plain(self.eval(arg_nodes[0]))
        if not isinstance(v, (str, float, int)):
            raise type_error(f"{what} takes a name, in quotes, or a number")
        return v

    def pick_effect(self, target, arg_nodes):
        """D-766: `effect(name)` by After Effects' name ("Slider Control 2" for the second
        one) or match name, or `effect(n)` counting every effect on the layer from 1."""
        v = self.one_arg(arg_nodes, "effect")
        layer = self.p.layers[target]
        effects = layer["effects"]
        seen = {}
        for i, fx in enumerate(effects):
            control = CONTROLS.get(fx["type_id"])
            shown = control[0] if control else OTHER_EFFECTS[fx["type_id"]]
            seen[shown] = seen.get(shown, 0) + 1
            name = shown if seen[shown] == 1 else f"{shown} {seen[shown]}"
            if isinstance(v, str) and (v == name or (control and v == control[1])):
                return FxRef(target, i)
            if not isinstance(v, str) and v == i + 1:
                return FxRef(target, i)
        raise ExprError("EXPRESSION_REFERENCE_MISSING",
                        f"{layer['name']} has no effect {show_arg(v)}")

    def pick_setting(self, fx_ref, arg_nodes):
        """D-766: an Expression Control's one setting, by name, match name or the number 1."""
        fx = self.p.layers[fx_ref.target]["effects"][fx_ref.index]
        control = CONTROLS.get(fx["type_id"])
        if control is None:
            raise syntax(f"{OTHER_EFFECTS[fx['type_id']]}'s settings cannot be read by an "
                         f"expression; only Expression Controls' can")
        v = self.one_arg(arg_nodes, "an effect's setting")
        if v not in (control[2], f"{control[1]}-0001", 1):
            raise ExprError("EXPRESSION_REFERENCE_MISSING",
                            f"{control[0]} has no setting {show_arg(v)}")
        if control[3] == "layer":
            # The layer it names, as `thisComp.layer(...)` gives it.
            lid = fx["parameters"].get("layer", "")
            if lid not in self.p.layers:
                raise ExprError("EXPRESSION_REFERENCE_MISSING",
                                "the Layer Control names no layer" if lid == "" else
                                f"there is no layer with the id \"{lid}\"")
            return LayerRef(lid)
        return SettingRef(fx_ref.target, fx_ref.index)

    def wiggle(self, args):
        if not 2 <= len(args) <= 5:
            raise type_error("wiggle takes a frequency and an amount, then up to three more")
        freq = self.number(args[0], "wiggle's frequency")
        amp = self.number(args[1], "wiggle's amount")
        octaves = self.number(args[2], "wiggle's octaves") if len(args) > 2 else 1
        mult = self.number(args[3], "wiggle's amount multiplier") if len(args) > 3 else 0.5
        t = self.number(args[4], "wiggle's time") if len(args) > 4 else self.time()
        for v in (freq, amp, octaves, mult, t):
            if not math.isfinite(v):
                raise type_error("wiggle's numbers must be finite")
        octaves = math.floor(octaves)
        if freq < 0 or not 1 <= octaves <= MAX_OCTAVES:
            raise type_error(f"wiggle needs a frequency of zero or more and 1 to "
                             f"{MAX_OCTAVES} octaves")
        call = self.wiggles
        self.wiggles += 1
        seed = self.seed()
        base = self.p.pre(self.target, self.prop, self.frame)
        base = base if isinstance(base, list) else [base]
        out = []
        for d, v in enumerate(base):
            total, weight, scale = 0.0, 1.0, 1.0
            for k in range(octaves):
                total += weight * noise(seed, call * 256 + d * 16 + k, t * freq * scale)
                weight *= mult
                scale *= 2.0
            out.append(v + amp * total)
        return out if self.dims > 1 else out[0]

    def random(self, args):
        if len(args) > 2:
            raise type_error("random takes at most two limits")
        call = self.randoms
        self.randoms += 1
        seed = self.seed()
        key = 0 if self.timeless else bits(float(self.time()))

        def r(d):
            return unit(mix(mix(seed, 0x10000 + call * 16 + d), key))

        if not args:
            return r(0)
        vals = [self.number_or_array(a) for a in args]
        lo, hi = (vals if len(vals) == 2 else [0.0, vals[0]])
        if isinstance(lo, list) != isinstance(hi, list) and len(vals) == 2:
            raise type_error("random's two limits must both be numbers or both be lists")
        if isinstance(hi, list):
            lo = lo if isinstance(lo, list) else [0.0] * len(hi)
            if len(lo) != len(hi):
                raise type_error("random's two lists must be the same length")
            return [a + r(d) * (b - a) for d, (a, b) in enumerate(zip(lo, hi))]
        return lo + r(0) * (hi - lo)

    def remap(self, name, args):
        if len(args) == 3:
            args = [args[0], 0.0, 1.0, args[1], args[2]]
        if len(args) != 5:
            raise type_error(f"{name} takes a time and two values, or a time, two limits and "
                             f"two values")
        t = self.number(args[0], f"{name}'s input")
        t0, t1 = self.number(args[1], "a limit"), self.number(args[2], "a limit")
        v0, v1 = self.number_or_array(args[3]), self.number_or_array(args[4])
        if isinstance(v0, list) != isinstance(v1, list) or \
                (isinstance(v0, list) and len(v0) != len(v1)):
            raise type_error(f"{name}'s two values must be the same shape")
        if t0 > t1:
            t0, t1, v0, v1 = t1, t0, v1, v0
        if t <= t0:
            f = 0.0
        elif t >= t1:
            f = 1.0
        else:
            f = (t - t0) / (t1 - t0)
            if name == "ease":
                f = f * f * (3.0 - 2.0 * f)
        if isinstance(v0, list):
            return [a + (b - a) * f for a, b in zip(v0, v1)]
        return v0 + (v1 - v0) * f

    def loop_out(self, args):
        if len(args) > 2:
            raise type_error("loopOut takes a kind and, if wanted, how many keyframes")
        kind = args[0] if args else "cycle"
        if kind not in ("cycle", "pingpong", "offset", "continue"):
            raise type_error("loopOut's kind is \"cycle\", \"pingpong\", \"offset\" or "
                             "\"continue\"")
        n = self.number(args[1], "how many keyframes") if len(args) > 1 else 0
        if not math.isfinite(n) or n < 0 or n != math.floor(n):
            raise type_error("loopOut's keyframe count is a whole number, zero or more")
        keys = self.p.property(self.target, self.prop)["keyframes"]
        f = self.frame
        pre = lambda g: self.p.pre(self.target, self.prop, g)
        if len(keys) < 2 or f <= keys[-1]["frame"]:
            return pre(f)
        last = keys[-1]["frame"]
        first = keys[0]["frame"] if n == 0 else keys[max(0, len(keys) - 1 - int(n))]["frame"]
        period = last - first
        if period <= 0:
            return pre(f)
        vec = lambda a, b, s: [x + s * y for x, y in zip(a, b)] if isinstance(a, list) \
            else a + s * b
        if kind == "cycle":
            return pre(first + (f - first) % period)
        if kind == "pingpong":
            q = f - last
            laps, r = divmod(q, period)
            return pre(last - r) if laps % 2 == 0 else pre(first + r)
        if kind == "offset":
            laps, r = divmod(f - first, period)
            gain = vec(pre(last), pre(first), -1)
            return vec(pre(first + r), gain, laps)
        step = vec(pre(last), pre(last - 1), -1)
        return vec(pre(last), step, f - last)


def evaluate(project, target, prop, frame):
    """(value in file units, error code or None, message). An error falls back to the keys."""
    run = Run(project)
    try:
        return project.to_file(prop, run.final(target, prop, frame)), None, ""
    except ExprError as e:
        return key_value(project.property(target, prop), frame), e.code, e.message


# --- the fixture project -----------------------------------------------------------------------

def prop(base, keys=(), expression=None, enabled=True):
    out = {"base": base, "keyframes": list(keys)}
    if expression is not None:
        out["expression"] = {"text": expression, "enabled": enabled}
    return out


def key(frame, value, interp="linear"):
    return {"frame": frame, "value": value, "interp": interp}


def layer(lid, name, position=(960, 540), rotation=None, opacity=None, **extra):
    record = {
        "id": lid, "kind": "raster", "name": name, "asset_id": "asset-cel",
        "enabled": True, "locked": False, "in_frame": 0, "out_frame": 49,
        "source_offset_frames": 0,
        "transform": {
            "anchor": prop([0, 0]),
            "position": extra.pop("position_prop", prop(list(position))),
            "scale": extra.pop("scale_prop", prop([100, 100])),
            "rotation": rotation or prop(0),
            "opacity": opacity or prop(1),
        },
        "exposure_spans": [{"start_frame": 0, "end_frame_exclusive": 49, "drawing_number": 1}],
        "mask": None, "matte": None, "blend_mode": "normal", "effects": [],
    }
    record.update(extra)
    return record


def project():
    layers = [
        layer("layer-spin", "Spin", rotation=prop(0, expression="time * 90")),
        # Named "Follow" and deliberately not named after the layer it follows: the reference
        # is by id, and the name is free to be anything.
        layer("layer-follow", "Follow", rotation=prop(
            0, expression='thisComp.layer("layer-spin").transform.rotation * -1')),
        layer("layer-shake", "Shake", position_prop=prop([960, 540], expression="wiggle(2, 30)")),
        layer("layer-shake-twos", "Shake on threes", position_prop=prop(
            [960, 540], expression="posterizeTime(8)\nwiggle(2, 30)")),
        layer("layer-lead", "Lead", position_prop=prop(
            [0, 540], [key(0, [0, 540]), key(48, [1920, 540])])),
        layer("layer-trail", "Trail", position_prop=prop([0, 540], expression=(
            '// a quarter of a second behind the lead\n'
            'thisComp.layer("layer-lead").position.valueAtTime(time - 0.25)'))),
        layer("layer-loop", "Loop", rotation=prop(
            0, [key(0, 0), key(12, 90), key(24, 30)], expression='loopOut("pingpong")')),
        layer("layer-fade", "Fade", opacity=prop(1, expression="linear(time, 0, 1, 0, 100)")),
        layer("layer-off", "Switched off", opacity=prop(1, expression="50", enabled=False)),
    ]
    return {
        "schema_version": 0,
        "project_id": "proj-expressions",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{
            "id": "asset-cel", "kind": "image_sequence", "name": "Cel",
            "pattern": "cel_####.png", "frames": {"1": "media/cel_0001.png"},
            "interpretation": {"color_space": "srgb", "alpha": "straight"},
        }],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": 1920, "height": 1080,
            "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 49,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 49},
            "camera": {
                "position": prop([960, 540], expression="wiggle(1, 10)"),
                "depth": prop(-1920),
                "zoom": prop(1920),
            },
            "layer_order": [lay["id"] for lay in layers],
            "layers": layers,
        }],
    }


def fx(iid, type_id, parameters):
    return {"enabled": True, "instance_id": iid, "parameters": parameters, "type_id": type_id}


def controls_project():
    """D-766: the same composition with After Effects' Expression Controls to read."""
    data = project()
    comp = data["compositions"][0]
    del comp["camera"]
    layers = [
        layer("layer-ctl", "Controls", effects=[
            fx("fx-slider", "core.slider_control",
               {"slider": prop(0, [key(0, 0), key(48, 100)])}),
            fx("fx-check", "core.checkbox_control",
               {"checkbox": prop(0, [key(0, 0, "hold"), key(24, 1, "hold")])}),
            fx("fx-angle", "core.angle_control", {"angle": 45}),
            fx("fx-point", "core.point_control", {"point": [100, 200]}),
            fx("fx-color", "core.color_control", {"color": [0.25, 0.5, 0.75]}),
            fx("fx-layer", "core.layer_control", {"layer": "layer-target"}),
            fx("fx-slider-2", "core.slider_control", {"slider": prop(0, expression="time * 10")}),
            fx("fx-blur", "core.gaussian_blur", {"sigma_px": 2}),
        ]),
        layer("layer-target", "Target", position=(300, 400)),
        layer("layer-reader", "Reader", rotation=prop(0, expression=(
            'thisComp.layer("layer-ctl").effect("Slider Control")("Slider")'))),
        # Lane q's two After Effects tutorial cases, each with its own control, written as the
        # tutorials write them.
        layer("layer-wiggle", "Wiggle", position_prop=prop(
            [960, 540], expression='wiggle(2, effect("Slider Control")("Slider"))'), effects=[
            fx("fx-amount", "core.slider_control",
               {"slider": prop(0, [key(0, 0), key(48, 100)])})]),
        layer("layer-box", "Box", opacity=prop(
            1, expression='if (effect("Checkbox Control")("Checkbox") == 1) 100 else 0'),
            effects=[fx("fx-on", "core.checkbox_control",
                        {"checkbox": prop(0, [key(0, 0, "hold"), key(24, 1, "hold")])})]),
    ]
    data["project_id"] = "proj-expression-controls"
    comp["layer_order"] = [lay["id"] for lay in layers]
    comp["layers"] = layers
    return data


# --- printing ----------------------------------------------------------------------------------

def show(v):
    if isinstance(v, list):
        return "(" + ", ".join(show(x) for x in v) + ")"
    if isinstance(v, float) and v == math.floor(v) and abs(v) < 1e15:
        return str(int(v))
    return repr(v)


def table(title, cols, rows):
    print(title)
    print()
    print("| " + " | ".join(cols) + " |")
    print("| " + " | ".join("---" for _ in cols) + " |")
    for r in rows:
        print("| " + " | ".join(str(c) for c in r) + " |")
    print()


def with_expression(data, target, name, text, enabled=True):
    """A copy of the project with one property's expression replaced."""
    data = copy.deepcopy(data)
    p = Project(data)
    record = p.property(target, name)
    record["expression"] = {"text": text, "enabled": enabled}
    return data


def outcome(data, target, name, frame):
    v, code, _ = evaluate(Project(data), target, name, frame)
    return show(v), code or "none"


def main():
    data = project()
    controls = controls_project()
    here = Path(__file__).resolve().parent.parent
    for name, d in [("expression_project.json", data),
                    ("expression_controls_project.json", controls)]:
        out = here / "Fixtures" / "projects" / name
        text = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
        if "--check" in sys.argv:
            if out.read_text(encoding="utf-8").replace("\r\n", "\n") != text:
                raise SystemExit(f"{out} is not what this file writes")
        else:
            out.write_text(text, encoding="utf-8", newline="\n")
    p = Project(data)

    def row(target, name, frames):
        return [evaluate(p, target, name, f) for f in frames]

    frames = [0, 6, 12, 24, 36, 48]
    spin = row("layer-spin", "rotation", frames)
    follow = row("layer-follow", "rotation", frames)
    table("FX-EXPR-001", ["frame", "Spin rotation", "Follow rotation"],
          [[f, show(a[0]), show(b[0])] for f, a, b in zip(frames, spin, follow)])

    frames = [0, 6, 12, 18, 24, 48]
    shake = row("layer-shake", "position", frames)
    table("FX-EXPR-002", ["frame", "Shake position"],
          [[f, show(v[0])] for f, v in zip(frames, shake)])

    frames = list(range(0, 7))
    twos = row("layer-shake-twos", "position", frames)
    table("FX-EXPR-003", ["frame", "Shake on threes position"],
          [[f, show(v[0])] for f, v in zip(frames, twos)])

    same = with_expression(data, "layer-spin", "rotation", "wiggle(2, 30)")
    seeded = with_expression(data, "layer-shake", "position", "seedRandom(5)\nwiggle(2, 30)")
    table("FX-EXPR-004", ["case", "value at frame 12"], [
        ["Shake's own expression, evaluated a second time",
         show(evaluate(Project(data), "layer-shake", "position", 12)[0])],
        ["Shake with seedRandom(5) first", show(evaluate(Project(seeded), "layer-shake",
                                                           "position", 12)[0])],
        ["wiggle(2, 30) on Spin's rotation", show(evaluate(Project(same), "layer-spin",
                                                           "rotation", 12)[0])],
    ])

    frames = [0, 6, 12, 24, 48]
    lead = row("layer-lead", "position", frames)
    trail = row("layer-trail", "position", frames)
    table("FX-EXPR-005", ["frame", "Lead position", "Trail position"],
          [[f, show(a[0]), show(b[0])] for f, a, b in zip(frames, lead, trail)])

    frames = [0, 12, 24, 30, 36, 42, 48]
    kinds = ["cycle", "pingpong", "offset", "continue"]
    looped = {k: with_expression(data, "layer-loop", "rotation", f'loopOut("{k}")')
              for k in kinds}
    table("FX-EXPR-006", ["frame"] + kinds,
          [[f] + [show(evaluate(Project(looped[k]), "layer-loop", "rotation", f)[0])
                  for k in kinds] for f in frames])

    frames = [0, 6, 12, 24, 48]
    fade = row("layer-fade", "opacity", frames)
    eased = with_expression(data, "layer-fade", "opacity", "ease(time, 0, 1, 0, 100)")
    table("FX-EXPR-007", ["frame", "linear, in the file", "ease, in the file"],
          [[f, show(v[0]), show(evaluate(Project(eased), "layer-fade", "opacity", f)[0])]
           for f, v in zip(frames, fade)])

    table("FX-EXPR-008", ["property", "expression", "value at frame 12", "diagnostic"], [
        [n, f"`{t}`", *outcome(with_expression(data, lid, n, t), lid, n, 12)]
        for lid, n, t in [
            ("layer-spin", "rotation", "[1, 2]"),
            ("layer-shake", "position", "5"),
            ("layer-shake", "position", "value + 5"),
            ("layer-spin", "rotation", "1 / 0"),
            ("layer-spin", "rotation", '"ninety"'),
            ("layer-spin", "rotation", "value[0]"),
            ("layer-shake", "position", "[1, 2] * [3, 4]"),
            ("layer-spin", "rotation", "posterizeTime(8)"),
            ("layer-fade", "opacity", "250"),
            ("camera", "zoom", "-10"),
            ("layer-spin", "rotation", "wiggle(2, 30, 40)"),
        ]])

    renamed = copy.deepcopy(data)
    Project(renamed).layers["layer-spin"]["name"] = "Something else entirely"
    gone = copy.deepcopy(data)
    gone["compositions"][0]["layers"] = [
        lay for lay in gone["compositions"][0]["layers"] if lay["id"] != "layer-spin"]
    gone["compositions"][0]["layer_order"].remove("layer-spin")
    table("FX-EXPR-009", ["case", "Follow rotation at frame 12", "diagnostic"], [
        ["as written", *outcome(data, "layer-follow", "rotation", 12)],
        ["Spin renamed", *outcome(renamed, "layer-follow", "rotation", 12)],
        ["Spin deleted", *outcome(gone, "layer-follow", "rotation", 12)],
    ])

    loop = with_expression(data, "layer-spin", "rotation",
                           'thisComp.layer("layer-follow").rotation')
    selfish = with_expression(data, "layer-spin", "rotation", "thisLayer.rotation + 1")
    own = with_expression(data, "layer-spin", "rotation",
                          "thisProperty.valueAtTime(time - 1) + value")
    table("FX-EXPR-010", ["case", "property", "value at frame 12", "diagnostic"], [
        ["Spin follows Follow, which follows Spin", "Spin rotation",
         *outcome(loop, "layer-spin", "rotation", 12)],
        ["the same", "Follow rotation", *outcome(loop, "layer-follow", "rotation", 12)],
        ["Spin reads its own finished value", "Spin rotation",
         *outcome(selfish, "layer-spin", "rotation", 12)],
        ["Spin reads its own keys through thisProperty", "Spin rotation",
         *outcome(own, "layer-spin", "rotation", 12)],
    ])

    chase = with_expression(data, "layer-spin", "rotation",
                            'thisComp.layer("layer-follow").rotation.valueAtTime(time - 1/24) + 1')
    chase = with_expression(chase, "layer-follow", "rotation",
                            'thisComp.layer("layer-spin").rotation.valueAtTime(time) + 1')
    long_text = "1" + " + 1" * 1100
    table("FX-EXPR-011", ["case", "value at frame 40", "diagnostic"], [
        ["Spin and Follow each read the other a frame earlier, forever",
         *outcome(chase, "layer-spin", "rotation", 40)],
        ["the same at frame 3, which runs out of frames before it runs out of depth",
         *outcome(chase, "layer-spin", "rotation", 3)],
        [f"`1 + 1 + ...`, {len(long_text)} bytes",
         *outcome(with_expression(data, "layer-spin", "rotation", long_text),
                  "layer-spin", "rotation", 40)],
    ])

    table("FX-EXPR-012", ["expression", "value at frame 12", "diagnostic"], [
        [f"`{t}`", *outcome(with_expression(data, "layer-spin", "rotation", t),
                            "layer-spin", "rotation", 12)]
        for t in ['require("fs")', 'eval("1")', 'system.callSystem("calc")',
                  "$.sleep(100000)", 'fetch("http://example.com")',
                  "while (1) {}", "function f() { return 1 }",
                  'thisComp.layer("layer-spin").sourceText']])

    frames = [0, 1, 12, 48]
    cases = [
        ("random()", "random()"),
        ("random(10)", "random(10)"),
        ("random(5, 10)", "random(5, 10)"),
        ("seedRandom(3, 1)\nrandom()", "seedRandom(3, 1) then random()"),
    ]
    table("FX-EXPR-013", ["frame"] + [c[1] for c in cases],
          [[f] + [show(evaluate(Project(with_expression(data, "layer-spin", "rotation", t)),
                                "layer-spin", "rotation", f)[0]) for t, _ in cases]
           for f in frames])

    frames = [0, 12, 24, 48]
    camera = row("camera", "position", frames)
    lens = with_expression(data, "layer-spin", "rotation", "thisComp.activeCamera.zoom / 100")
    table("FX-EXPR-014", ["frame", "camera position", "Spin rotation from the camera's zoom"],
          [[f, show(v[0]), show(evaluate(Project(lens), "layer-spin", "rotation", f)[0])]
           for f, v in zip(frames, camera)])

    frames = [0, 6, 12, 18, 24]
    bob = with_expression(data, "layer-shake", "position",
                          "bob = Math.sin(time * Math.PI * 2) * 20\nvalue + [0, bob]")
    table("FX-EXPR-015", ["frame", "Shake position"],
          [[f, show(evaluate(Project(bob), "layer-shake", "position", f)[0])] for f in frames])

    # Both readers get percent: an expression reads opacity in the same units whether or not the
    # opacity it reads has an expression of its own switched on.
    reads_off = with_expression(data, "layer-spin", "rotation",
                                'thisComp.layer("layer-off").opacity')
    reads_fade = with_expression(data, "layer-spin", "rotation",
                                 'thisComp.layer("layer-fade").opacity')
    table("FX-EXPR-016", ["property", "value at frame 12", "diagnostic"], [
        ["Switched off opacity", *outcome(data, "layer-off", "opacity", 12)],
        ["Spin rotation reading Switched off's opacity",
         *outcome(reads_off, "layer-spin", "rotation", 12)],
        ["Spin rotation reading Fade's opacity",
         *outcome(reads_fade, "layer-spin", "rotation", 12)],
    ])

    controls_tables(controls)


def controls_tables(data):
    """D-766: FX-EXPR-017 to 020, Expression Controls and the if, logic and comparisons."""

    def on(text, target="layer-ctl", name="rotation"):
        return with_expression(data, target, name, text)

    def cell(d, target, name, frame):
        v, code, _ = evaluate(Project(d), target, name, frame)
        return show(v) if code is None else f"{show(v)}, {code}"

    frames = [0, 12, 24, 48]
    reads = [
        'effect("Slider Control")("Slider")',
        'effect("Slider Control")(1)',
        "effect(1)(1)",
        'effect("ADBE Slider Control")("ADBE Slider Control-0001")',
        'effect("Slider Control")("Slider").value',
        'effect("Slider Control")("Slider").valueAtTime(time + 0.5)',
        'effect("Checkbox Control")("Checkbox")',
        'effect("Angle Control")("Angle")',
        'effect(3)(1) + effect("Point Control")("Point")[1]',
        'effect("Color Control")("Color")[2] + effect("Color Control")("Color")[3]',
        'effect("Layer Control")("Layer").position[0]',
        'effect("Slider Control 2")("Slider")',
        'effect(7)("Slider") + effect("Slider Control")("Slider")',
    ]
    table("FX-EXPR-017", ["Controls rotation expression"] + [f"frame {f}" for f in frames],
          [[f"`{t}`"] + [cell(on(t), "layer-ctl", "rotation", f) for f in frames]
           for t in reads] +
          [["Reader's own expression, from another layer"] +
           [cell(data, "layer-reader", "rotation", f) for f in frames]])

    frames = [0, 12, 24, 48]
    logic = [
        "if (time < 1) 10 else 20",
        "if (time < 0.5) 1\nelse if (time < 1) 2\nelse 3",
        "if (time >= 1) {\n  x = 5\n} else {\n  x = 7\n}\nx * 2",
        "y = 4; if (time > 1) { y = y + 1 }; y",
        "(time > 0 && time < 1) + (time == 0 || time == 2) * 10",
        "!(time < 1) + !0 * 10 + !5 * 100",
        "true + false * 10 + (1 != 2) * 100 + (3 === 3) * 1000 + (3 !== 3) * 10000",
        "(2 <= 2) + (2 < 2) * 10 + (2 >= 3) * 100 + (3 > 2) * 1000",
        "(0 || 7) + (5 && 0) * 10 + (5 && 6) * 100",
        'if (effect("Checkbox Control")("Checkbox") == 1) 100 else 0',
    ]
    table("FX-EXPR-018", ["Controls rotation expression"] + [f"frame {f}" for f in frames],
          [["`" + t.replace("\n", "\\n") + "`"] +
           [cell(on(t), "layer-ctl", "rotation", f) for f in frames] for t in logic])

    looped = copy.deepcopy(data)
    Project(looped).layers["layer-ctl"]["effects"][6]["parameters"]["slider"]["expression"] = {
        "text": 'effect("Slider Control 2")("Slider") + 1', "enabled": True}
    nameless = copy.deepcopy(data)
    Project(nameless).layers["layer-ctl"]["effects"][5]["parameters"]["layer"] = ""
    wrong = [
        'effect("Nothing Here")("Slider")',
        'effect("Slider Control")("Nothing Here")',
        "effect(99)(1)",
        "effect(1)(2)",
        'effect("Gaussian Blur")(1)',
        'effect("Slider Control")',
        'thisComp.layer("layer-gone").effect(1)(1)',
        "if (time < 1) 5",
        "if (time > 2) y = 3\ny",
        "[1, 2] == [1, 2]",
        "1 & 2",
        "if (time < 1) { 5",
        "while (1) {}",
    ]
    table("FX-EXPR-019", ["case", "Controls rotation at frame 36"],
          [[f"`{t}`".replace("\n", "\\n"), cell(on(t), "layer-ctl", "rotation", 36)]
           for t in wrong] +
          [["Slider Control 2's own expression reads itself; Controls rotation reads it",
            cell(with_expression(looped, "layer-ctl", "rotation",
                                 'effect("Slider Control 2")("Slider")'),
                 "layer-ctl", "rotation", 36)],
           ["the Layer Control set to None",
            cell(with_expression(nameless, "layer-ctl", "rotation",
                                 'effect("Layer Control")("Layer").position[0]'),
                 "layer-ctl", "rotation", 36)]])

    frames = [0, 6, 12, 23, 24, 36, 47]
    table("FX-EXPR-020", ["frame", "Wiggle position", "Box opacity, in the file"],
          [[f, cell(data, "layer-wiggle", "position", f), cell(data, "layer-box", "opacity", f)]
           for f in frames])


if __name__ == "__main__":
    main()
