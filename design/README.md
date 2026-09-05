# Design

Interface design work for G1-core, per `Markdown/05_UX_Specification.md`.

Five screens only: empty project, normal editing, exposure editing, missing-media recovery, export running and failed. Plus a styled system: dark theme, type scale, spacing, state colors, icon direction.

Delivered as HTML and CSS, because under ADR-004 the design is the product rather than a mockup of it.

Do not design panels for masks, effects or cache state. Those features are parked under D-12.

Empty. Recommended after a spike proves the application launches and displays a composited frame.
