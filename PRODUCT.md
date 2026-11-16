# Product

<!-- impeccable:product-schema 1 -->

## Platform

desktop

## Stack

Rust + Iced 0.14 native desktop (existing; preserve).

## Users

- **Makers and crafters** preparing raster artwork for laser cutting, engraving, or vinyl cutting. They need clean binary silhouettes, threshold control, and speckle removal.
- **Designers and developers** converting bitmap logos, icons, and artwork to SVG for editing, print, or web use. They need color fidelity, fast iteration, and a preview they can trust.

Both work in short, focused sessions on one asset at a time, locally, without accounts or uploads.

## Product Purpose

BinVec converts one raster image into an SVG in real time in a single native window. Success means the user trusts the preview, tunes a small set of conversion controls, and exports an SVG next to the source without leaving the window.

## Positioning

A local, single-window vectorizer with live preview and domain-specific controls (binary threshold, invert, alpha handling, speckle filter, color precision, gradient step). The preview and the controls are one instrument; no upload, no account, no batch pipeline.

## Operating Context

- Local files from the user's machine; supported formats: PNG, JPEG, GIF, BMP, WebP, ICO, TIFF, AVIF, PNM, DDS, TGA.
- One image loaded at a time. Conversion runs off the UI thread; parameter changes trigger automatic re-rendering.
- Export writes `<source-stem>.svg` beside the source file; there is no save dialog and no path picker.
- Desktop window, mouse and keyboard, Linux and Windows; current window 950x700 with a 800x600 minimum.
- The app checks for and installs updates on launch.

## Capabilities and Constraints

- Binary mode: threshold 0-255, invert, ignore alpha.
- Color mode: color precision 0-8, gradient step 0-128.
- Shared: speckle/detail filter 0-128.
- SVG preview is held in memory and rendered by the Iced svg widget.
- Light and dark modes: `AUTO` follows the operating system, `LIGHT` and `DARK` are explicit overrides. The choice persists under the platform config directory. The proof plate stays lit paper in both modes so the trace remains readable.
- No batch conversion, no undo history, no image editing, no cloud, no telemetry.
- A render can take noticeable time; parameter changes made while a render is in flight must not leave the preview disagreeing with the controls (approved behavior fix).

## Brand Commitments

- Name: BinVec. Existing logo/icon: black rounded tile with teal `#00BAD1` and red `#F24236` diagonal stripes; wordmark uses teal for "Bin" and red for "Vec".
- Native, lightweight, offline, cross-platform (Linux and Windows). MIT licensed.
- The brand mark is preserved. The UI palette may be re-derived for this redesign (user approved re-palette).

## Evidence on Hand

- `README.md` feature list and installation instructions.
- `banner.png` and `.desktop/icon.png` (brand mark), `screenshot.jpg` and `screenshot-dark.jpg` (current UI).
- `CHANGELOG.md` (release history).
- No user testimonials, benchmarks, customer names, or pricing. Do not fabricate any.

## Product Principles

1. **The preview is the product.** Every control is judged by what it does to the canvas.
2. **One asset, one window.** No navigation, modal detours, or setup steps in the core open-preview-export flow.
3. **Truthful state.** Never show a preview that disagrees with the controls; always name what is happening.
4. **Local and fast.** No uploads, no accounts; the only wait is the conversion itself.

## Accessibility & Inclusion

WCAG AA contrast in both modes, no color-only state encoding, and a reduced-motion path are required for this redesign. The dark shell keeps text at 15.21:1, muted at 6.56:1, and the lit plate keeps its light-mode contrast. The theme control is labelled `AUTO / LIGHT / DARK`, so mode is never communicated by color alone. The approved scope was a visual overhaul with preview affordances, not keyboard-first navigation; Iced 0.14 exposes no button focus style, so the control path remains pointer-first. Keyboard-first traversal and visible focus rings are a separate follow-up, not a hidden part of this build. The OS title bar is outside the app surface and may follow the system while the client area is manually overridden. No formal external audit has been established.
