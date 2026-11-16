---
version: 1
slug: "src-main-rs"
primary_target: "src/main.rs"
related_targets: []
---

# BinVec — main window

Scope: the application's only screen, including empty, tracing, ready, saved, and error states.
Mode: Operate.
Audience: makers preparing laser and vinyl cuts; designers and developers converting bitmap assets.
Task: open one raster, tune the conversion, trust the preview, and write an SVG beside the source.
Constraints: single window, existing conversion semantics, no batch or cloud, 800x600 minimum, WCAG AA, pointer-first controls. Keyboard-first traversal is a separate, unapproved scope; Iced 0.14 exposes no button focus style.

## Direction contract

THESIS: A conversion is a plate, not a form. The preview is the drawing; the controls are its reference key; registration state carries the work, and no card chrome competes with the figure.

OWN-WORLD: Plate paper `#F3F0E7`, India ink `#15161A`, hairline rules `#D8D2C3`, brand teal and red as ink swatches and semantic state, 45-degree hatching for empty/disabled fields, 1px ink frame, crop and registration ticks, 2px control radius, system sans for language and mono for every value, path, dimension, and status code. Light is a paper shell and plate; dark is an ink-room shell (`#171614`, warm paper text `#F1EBDD`, rule `#3B3730`) with the same lit plate paper. `AUTO` follows the OS; `LIGHT`/`DARK` override and persist.

STORY: The user sees a drawing on a plate, reads the source and the active mode at a glance, tunes the key, watches the plate settle into register, checks the source against the trace, and writes the SVG next to the file. They believe the preview because the app shows both the raster and the vector and never lets the key disagree with the figure.

FIRST VIEWPORT: Top title block with the brand tile, BINVEC wordmark, plate number, an `AUTO / LIGHT / DARK` theme control, live status, OPEN IMAGE and SAVE SVG. A 304px reference key at left holds SOURCE, MODE, KEY, and PLATE RECORD in one scrollable column. The proof plate stays lit paper in both modes. The figure region holds a caption line (FIG. 1, proof toggle) above an ink-framed plate; the plate interior is a checkerboard ground with corner crop marks, registration ticks, and the trace centered at fit. Zoom controls and the export path sit in the bottom record strip.

FORM: candidate 6 of 7 grounded directions, patent-office drawing plates; the roll's seed:d91b3c699ce068ae.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance.
