---
name: BinVec
description: Raster-to-vector conversion as a drawing plate.
colors:
  paper: "#F3F0E7"
  paper-raised: "#F8F5EC"
  paper-deep: "#E8E3D6"
  ink: "#15161A"
  ink-hover: "#2B2D33"
  ink-pressed: "#000000"
  muted: "#6E6A60"
  rule: "#D8D2C3"
  teal: "#00A8BE"
  teal-dark: "#00707C"
  red: "#E14134"
  red-dark: "#B3261E"
  green: "#2F6B4F"
  ochre: "#8A5A00"
  shell-dark: "#171614"
  shell-dark-raised: "#201E1A"
  shell-dark-deep: "#29251F"
  shell-dark-text: "#F1EBDD"
  shell-dark-muted: "#A39B8B"
  shell-dark-rule: "#3B3730"
  teal-night: "#3FD6E8"
  red-night: "#FF6B5E"
  green-night: "#6FD3A3"
  ochre-night: "#E0B45A"
typography:
  title:
    fontFamily: "system-ui, -apple-system, 'Segoe UI', sans-serif"
    fontSize: "20px"
    fontWeight: 600
    lineHeight: 1.2
  wordmark:
    fontFamily: "system-ui, -apple-system, 'Segoe UI', sans-serif"
    fontSize: "16px"
    fontWeight: 700
    lineHeight: 1
  body:
    fontFamily: "system-ui, -apple-system, 'Segoe UI', sans-serif"
    fontSize: "13px"
    fontWeight: 400
    lineHeight: 1.4
  label:
    fontFamily: "system-ui, -apple-system, 'Segoe UI', sans-serif"
    fontSize: "11px"
    fontWeight: 600
    lineHeight: 1.2
  data:
    fontFamily: "ui-monospace, 'SFMono-Regular', Consolas, 'Courier New', monospace"
    fontSize: "12px"
    fontWeight: 400
    lineHeight: 1.4
rounded:
  plate: "0px"
  swatch: "1px"
  control: "2px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
components:
  button-primary:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  button-primary-hover:
    backgroundColor: "{colors.ink-hover}"
    textColor: "{colors.paper}"
  button-primary-disabled:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.muted}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  button-secondary:
    backgroundColor: "transparent"
    textColor: "{colors.ink}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  segment-active:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    rounded: "{rounded.swatch}"
    padding: "8px 12px"
  button-primary-dark:
    backgroundColor: "{colors.shell-dark-text}"
    textColor: "{colors.shell-dark}"
    rounded: "{rounded.control}"
    padding: "8px 12px"
  checkbox-checked:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    rounded: "{rounded.swatch}"
    size: "16px"
  plate-frame:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
    rounded: "{rounded.plate}"
  key-column:
    backgroundColor: "{colors.paper}"
    width: "304px"
---

# Design System: BinVec

## Overview

**Creative North Star: "Plate & Reference"**

BinVec is a proofing table, not a settings form. The drawing owns the window; a narrow reference key names the controls; the plate, crop marks, and registration ticks carry the working state. Paper and ink do the structural work, while the brand's teal and red survive as ink swatches and semantic state rather than decoration.

The interface should feel like a well-kept patent drawing plate: flat, precise, handwritten nowhere, and legible from across the room. Every value is monospaced, every rule is hairlined, every surface is paper. The work is visible before the chrome is.

The system has two modes. **Light** is a paper table with a paper plate. **Dark** is an ink-room: the title block, reference key and record strip go near-black, while the proof plate remains the same lit paper so ink traces and source proofs keep their contrast. The theme control in the title block is `AUTO / LIGHT / DARK`; Auto follows the operating system, the manual choice persists, and the plate never inverts.

**Key Characteristics:**
- Light plate-paper shell and plate; dark shell with the same lit plate paper.
- One figure per window; controls live in a 304px reference key.
- `AUTO / LIGHT / DARK` title-block control; system-following by default.
- Registration and proof state, not cards or shadows, communicate progress.
- The brand appears as ink swatches and semantic color, never as a second surface treatment.
- No shadows, gradients, glass, emoji, or icon fonts.

## Colors

A paper-and-ink palette with two spot inks and two semantic inks. The neutrals are warm; the accents are used at swatch and state scale only. Light and dark share the plate; the shell swaps.

### Primary
- **Proof Teal** (#00A8BE): Slider fill, ready-state square, the "Bin" wordmark in its deep cut. The bright teal is never used for small text on paper.
- **Deep Teal** (#00707C): Text-safe teal for the wordmark and active labels; 5.11:1 on plate paper.

### Secondary
- **Errata Red** (#E14134): Out-of-register state, destructive/error marks, the "Vec" wordmark's bright cut. Not for body text.
- **Deep Errata** (#B3261E): Error text and the failed-state square; 5.74:1 on plate paper.

### Neutral
- **Plate Paper** (#F3F0E7): Window, key, and plate frame ground.
- **Raised Paper** (#F8F5EC): Secondary button hover.
- **Deep Paper** (#E8E3D6): Checkerboard ground and pressed secondary buttons.
- **India Ink** (#15161A): Text, primary fills, plate frame, crop marks; 15.87:1 on paper.
- **Hairline** (#D8D2C3): Rules, rails, and separators.
- **Muted Ink** (#6E6A60): Helper text and metadata; 4.73:1 on paper.
- **Trace Green** (#2F6B4F): Saved confirmation.

### Modes
- **Light:** plate paper `#F3F0E7` shell, ink `#15161A` type, paper plate. Primary actions are ink fills with paper text.
- **Dark:** ink-room shell `#171614`, warm paper type `#F1EBDD`, muted `#A39B8B`, rules `#3B3730`. Primary actions invert to a paper fill with ink text. The plate stays `#F3F0E7` with `#15161A` ink.
- **Auto:** follows `iced::system::theme`; a system `None` resolves to Light. `LIGHT` and `DARK` are explicit overrides; the choice is persisted under the platform config directory (`binvec/theme`).
- **Dark accents:** teal `#3FD6E8`, red `#FF6B5E`, green `#6FD3A3`, ochre `#E0B45A`. Dark muted text is 6.56:1 on the shell; dark primary text is 15.21:1 on the paper fill.
- The OS title bar is outside the app surface; a manual override themes the client area only.

### Named Rules
**The Lit Plate Rule.** Dark mode darkens the shell, never the proof. The plate remains paper so ink traces and source proofs keep their contrast.

**The Single-Plot Rule.** One filled ink swatch and one word name the active mode. A second accent never competes with the drawing.

**The Text-Safe Teal Rule.** Bright teal is a fill and a square, never small text. Use deep teal when the accent must be read.

## Typography

**Display Font:** system UI sans (platform default)
**Body Font:** system UI sans (platform default)
**Label / Mono Font:** system UI monospace

**Character:** A workhorse pairing. The sans carries instructions and labels; the mono carries every value, path, dimension, status code, and mode name. No webfont is bundled.

### Hierarchy
- **Title** (600, 20px, 1.2): Empty, loading, and failure plate titles only.
- **Wordmark** (700, 16px, 1): BINVEC, with the two ink colors split across the syllables.
- **Body** (400, 13px, 1.4): Instructions, errors, and the source file name.
- **Label** (600, 11px, 1.2, uppercase): Section headings, button labels, status text, and metadata keys.
- **Data** (400, 12px, 1.4, mono): Slider values, dimensions, paths, zoom, and timestamps.

### Named Rules
**The One Scale Rule.** Labels, values, and captions share one compact size. Hierarchy comes from rules, weight, and ink—not from inflated display type.

## Layout

A two-column shell below a 56px title block and above a 34px record strip. The title block holds the brand, the `AUTO / LIGHT / DARK` theme control, the live status, and the open/save actions. The left key is fixed at 304px with a 1px hairline; the right figure takes the rest. The key scrolls; the title block, record strip, and figure do not. The plate interior fits the media with a 24px margin at `FIT`.

Spacing uses 4 / 8 / 12 / 16 / 24. The key pads 24, sections are 24 apart, and control groups are 8 or 12 apart. The window is 950×700 by default and 800×600 at minimum; at minimum width the title block drops the "RASTER → VECTOR" suffix and the plate re-fits. Nothing in the figure is pushed below the fold.

## Elevation & Depth

Flat by rule. There are no shadows. Depth comes from the 1px ink plate frame, the paper/paper-deep checkerboard, and 45-degree hatching in the no-data field. The interface is a sheet on a table, not a stack of panels.

### Named Rules
**The Flat-By-Default Rule.** A shadow appears only if it describes a physically raised object. Nothing in BinVec is physically raised.

## Shapes

Sharp plate, soft controls. The plate frame, checkerboard, crop marks, and hatching are square (0px). Swatches, segment fills, checkboxes, and slider handles use 1px; buttons and scrollbar scrollers use 2px. Borders are 1px ink or hairline; never 2px except the drag-over plate signal.

The mode swatch is a 16×16 split square: ink/paper for BINARY, teal/red for COLOR, with a 1px ink frame.

## Components

### Buttons
- **Shape:** 2px radius, 1px border, 8×12 padding.
- **Primary (light):** India ink fill, plate-paper text. Hover #2B2D33, pressed #000000.
- **Primary (dark):** Paper fill, ink-room text. Hover #FFF8E8, pressed #D8CFBD. The action inverts; the plate does not.
- **Disabled:** plate-paper fill, hairline border, muted ink text. The label stays visible.
- **Secondary:** transparent fill, ink text, 1px ink border. Hover raised paper; pressed deep paper.
- **Segment:** a 1px ink frame with 2px padding; the active half is ink fill with paper text, the inactive half is transparent with ink text. The active half is non-interactive.
- **Ghost:** transparent, no border; used only inside the zoom group; hover deep paper.

### Inputs / Fields
- **Checkbox:** 16px square, 1px border (muted at rest, ink on hover/checked), ink fill + paper check when checked; 12px label.
- **Slider:** 2px rail; teal filled segment, hairline unfilled segment; 8px rectangular paper handle with a 1px ink border; deep teal on hover, ink handle while dragging.
- **Focus:** Iced 0.14 exposes no button focus style; the control set is mouse-first. Keyboard reachability is provided by the native widget tree where the toolkit supports it.

### Plate
- **Frame:** 1px India ink, square corners, paper ground.
- **Ground:** 16px checkerboard in paper and deep paper.
- **Crop marks:** 14px arms at a 12px inset, 1px ink.
- **Edge register ticks:** 8px arms at the midpoint of each edge, 1px ink; they share the registration offset with the corner marks.
- **State copy:** the empty, tracing, and failure blocks sit on a borderless paper knockout, so muted ink is never set directly over the checkerboard or the hatch.
- **Tracing:** 3px outward registration offset on all four corners and all four edge ticks; a 1px teal scan line with a 6px teal wash sweeps the plate. When the render lands, the ticks settle back into register over ~96ms.
- **No data / failure:** 45-degree hairline hatching at 14px spacing over the checkerboard.
- **Zoom:** `FIT` is the default; `−` / `+` step by 1.25× between 5% and 400%, stepping from the unclamped current scale and never moving against the control: below 5%, `−` holds and `+` clamps to 5%; above 400%, `+` holds and `−` clamps to 400%; the plate scrolls to pan when the media exceeds the viewport. The vector proof's display box is capped at 2048 px on its longest edge and 4.2 M logical px of area, so its re-rasterized pixmap stays bounded at high zoom; the source proof keeps the full range.

### Theme Control
A compact 2px-padded segment in the title block: `AUTO / LIGHT / DARK`. The active half is the current primary fill (ink in light, paper in dark); the frame is a hairline in the shell. Auto is the default; the explicit choice is persisted. The control sits before the live status and always shows the selected policy, while the client area shows the resolved mode.

### Navigation
There is no navigation model. The only mode switch is the BINARY/COLOR segment; the proof switch is VECTOR/SOURCE inside the figure caption. Both state the active side with an ink fill.

### Signature Component: The Registration Ticks
The four crop corners are the state display. They sit 3px out of register while a trace is running and snap back when it is ready. The scan line and the ticks are the only automatic motion in the app; `BINVEC_REDUCE_MOTION=1` disables both the sweep and the settle.

## Do's and Don'ts

### Do:
- **Do** keep the drawing as the largest, quietest surface in the window.
- **Do** set every value, path, dimension, status, and mode name in mono.
- **Do** show SOURCE and VECTOR at the same scale so the comparison is honest, except above the vector proof's display cap.
- **Do** keep the key scrollable and the title/record strips fixed at 800×600.
- **Do** use the brand teal and red as swatches and semantic state, not as text on paper.
- **Do** place state copy on a paper knockout. Muted ink directly over the checkerboard is below AA.
- **Do** let the shell go dark while the proof stays paper. The plate is the lit artifact in both modes.

### Don't:
- **Don't** wrap controls in cards or add shadows to separate regions. Hairlines and spacing do that work.
- **Don't** use the bright teal for small text; use deep teal.
- **Don't** add a second accent for a single widget or state.
- **Don't** use emoji, glyph icons, gradients, glass, or decorative status dots.
- **Don't** hide the current mode, the source file, or the export path behind an interaction.
- **Don't** invert the plate or the trace in dark mode. A black trace on a dark checkerboard is unreadable.
