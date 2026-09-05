# ADR-012: Render trace diagnostics

Status: ACCEPTED
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

When a composited frame is wrong, the normal debugging path is to read code, set breakpoints and inspect values. The owner cannot do any of that, and an agent working in a terminal cannot see the screen by default. Without a deliberate facility, a wrong picture becomes an argument between two parties who each cannot check the other.

## Decision

The renderer supports a trace mode that writes every intermediate layer buffer to a PNG, tagged with layer ID, composition frame and pipeline stage, into a `trace/` directory.

Trace output is diagnostic. It is never on by default, it is never part of export, and `trace/` is not committed.

## Rationale

A directory of intermediate images converts a debugging problem into a looking problem. The owner can see which layer went wrong and at which stage, and can say so in ordinary words. An agent can read those PNGs directly and reason about them.

It is also the cheapest possible verification artifact: the images are a by-product of the render the agent already ran, so producing them costs almost nothing.

## Consequences

Every stage that produces a buffer must be able to name itself, which is mild pressure toward a legible pipeline. That pressure is welcome.

Trace mode writes many files quickly and can fill a disk on a long composition. It is bounded by an explicit frame range, never left running.

Trace images are written in the working space and tagged with their alpha mode and color space, like every other buffer in this system. An untagged trace image would mislead exactly when precision matters most.
