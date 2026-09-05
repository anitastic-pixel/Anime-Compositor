# ADR-001: Windows 11 x64 only, on a declared reference machine

Status: ACCEPTED
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

Version 0.2 left the target platform open. The project has one developer, one machine, no test lab and no human code reviewer. Every platform added multiplies the verification surface, and verification time is the actual bottleneck on this project.

## Decision

Windows 11 x64 is the only supported platform. The reference machine is declared explicitly: Ryzen 9 9900X (12 cores), 64 GB RAM, RTX 4070 Ti Super 16 GB.

Every performance number in this pack is a claim about that machine and no other. Numbers are recorded with the machine named, never as portable characteristics of the software.

No cross-platform support is claimed, designed for, or promised in any user-facing text.

## Rationale

Naming the machine is what makes the performance targets falsifiable. "Scrub latency under 100 ms" is unverifiable as a general statement and perfectly verifiable as a statement about this hardware.

Windows-only also removes a class of problems the owner cannot debug: platform-specific file path behavior, per-platform color management, three sets of packaging and signing, and rendering differences between GUI backends.

## Consequences

Platform-specific code stays behind narrow interfaces. This is to keep a future port possible, not because one is planned, and it must not be used to justify an abstraction layer with one implementation.

Japanese filenames and Unicode paths are still first-class, because the reference shot exercises them. Windows-only is not an excuse to ignore encoding.

If the reference machine changes, prior performance numbers are invalidated, not translated. They are re-measured or struck.

A future port is a new decision with new evidence, not a debt this ADR acknowledges.
