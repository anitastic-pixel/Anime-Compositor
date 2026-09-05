# Error and diagnostics catalog

Version 0.2 | 2026-09-04 | Proposed baseline

## Diagnostic contract

Diagnostics have stable machine IDs, severity, concise user message, technical detail and optional remediation action. Internal exceptions or library error strings are not user-facing identifiers. A diagnostic may include paths/IDs only after privacy-safe formatting.

Severity levels: INFO, WARNING, ERROR and FATAL. WARNING permits the current operation with explicit degradation; ERROR rejects the requested operation but keeps the app usable; FATAL means the current project/process cannot continue safely.

## Core IDs

| ID | Severity | Meaning | Required behavior |
|---|---|---|---|
| PROJECT_SCHEMA_NEWER | ERROR | Project requires newer unsupported schema | do not reinterpret; offer read-only metadata only if safe |
| PROJECT_SCHEMA_INVALID | ERROR | Project violates schema/invariants | refuse model construction; show details |
| PROJECT_SAVE_FAILED | ERROR | Save could not complete | keep previous valid save; preserve dirty state |
| PROJECT_RECOVERY_AVAILABLE | INFO | Autosave/recovery candidate exists | show timestamp/path choice |
| MEDIA_MISSING | WARNING | Referenced media unavailable | preserve reference; render transparent placeholder + warning |
| MEDIA_SEQUENCE_GAP | WARNING | Requested drawing number absent | do not substitute adjacent frame |
| MEDIA_UNSUPPORTED_FORMAT | ERROR | Decoder not supported | preserve asset record; report format |
| MEDIA_DECODE_FAILED | ERROR | Supported decoder failed on file | identify file/frame; continue other frames where safe |
| EFFECT_UNSUPPORTED | WARNING | Effect type not installed/implemented | preserve serialized record; bypass with visible warning |
| EFFECT_PARAMETER_INVALID | ERROR | Effect parameter violates contract | reject edit/load as appropriate |
| MATTE_REFERENCE_MISSING | WARNING | Matte ID unresolved | preserve reference; render defined fallback with warning |
| MATTE_CYCLE | ERROR | Matte dependency cycle detected | reject command/load render graph |
| EXPRESSION_CYCLE | ERROR | Expression dependency cycle | stop affected property evaluation |
| EXPRESSION_TIMEOUT | ERROR | Bounded evaluator limit exceeded | terminate expression deterministically |
| GPU_BACKEND_FAILED | ERROR | Production GPU path failed | use approved fallback only if explicitly supported; never silently change pixels |
| GPU_OUT_OF_MEMORY | ERROR | GPU allocation failed | cancel request, free transient resources, advise lower preview scale |
| EXPORT_WRITE_FAILED | ERROR | Output file could not be written | report completed frames and failing path |
| EXPORT_CANCELLED | INFO | User cancelled export | preserve completed-frame list; no false success |
| INVALID_PATH | ERROR | Path fails normalization/access policy | reject operation with remediation |
| DEPENDENCY_LICENSE_UNRESOLVED | ERROR for distribution | Required distribution review incomplete | block public package |

## Missing/unsupported render fallback

G1 default for an unresolved raster source is transparent pixels plus persistent warning, not a colored placeholder in final export. Viewer may overlay a non-rendering warning badge. Unknown effects are bypassed while preserved in the project, and exported output must report that fidelity is incomplete.

These fallbacks are chosen to avoid fabricating media/effect behavior. They remain subject to user validation.

## Logging

Each diagnostic log record contains timestamp, ID, project revision/job ID where applicable and sanitized detail. Default logs do not include image pixels, full project contents or secrets. Repeated frame-level warnings should be rate-limited while retaining counts/ranges.

## UI presentation

Errors that block a direct command appear adjacent to the action or in a focused dialog. Background preview/export errors remain in a diagnostics/status panel. Every actionable message states what failed and the next safe action; marketing-style vague messages are prohibited.

## Testing

T-07/T-08/T-12/T-14 explicitly assert diagnostic IDs for known failure fixtures. A test must verify both model behavior and that the user is not told the operation succeeded.

Related documents: 05, 10, 24 and 25.
