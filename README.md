<p align="center">
    <img src="https://raw.githubusercontent.com/RouHim/binvec/main/banner.png" width="250"/>
</p>

<p align="center">
    <img alt="GitHub release (with filter)" src="https://img.shields.io/github/v/release/rouhim/binvec">
    <img alt="GitHub Release Date - Published_At" src="https://img.shields.io/github/release-date/rouhim/binvec">
    <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/rouhim/binvec/pipe.yaml">
</p>

<p align="center">
    <i>BinVec is a graphical program that converts raster graphics into vector graphics with real-time previews. BinVec is built with Rust and is available for Linux and Windows.</i>
</p>

## Features

* Convert raster graphics to vector graphics with live previewing
* Two conversion modes:
  * Black and white vectorization with threshold adjustment
  * Color vectorization with customizable color count and gradient steps
* Advanced configuration options:
  * Binarization threshold control
  * Alpha channel handling
  * Speckle filtering
  * Color precision control
  * Black/white inversion
* One-click SVG export to the same location as the source image
* Support for multiple input formats: PNG, JPEG, GIF, BMP, WebP, ICO, TIFF, AVIF, PNM, DDS, TGA
* Built with Rust using the Iced UI toolkit for a responsive and native feel
* Automatic updates

## Screenshot

<img src="https://raw.githubusercontent.com/RouHim/binvec/main/screenshot.jpg" width="500"/>

## Installation

### Pre-built Binaries

Pre-built binaries are available for the following platforms:

#### Linux
- x86_64 (64-bit Intel/AMD)
- aarch64 (64-bit ARM)
- armv7 (32-bit ARM)
- arm (32-bit ARM)

#### Windows
- x86_64 (64-bit Intel/AMD)

Download the latest release from the [GitHub releases page](https://github.com/RouHim/binvec/releases/latest).

### Linux Package Managers

#### Arch Linux (AUR)

BinVec is available in the Arch User Repository (AUR) in two versions:

```bash
# Source version (builds from source)
yay -S binvec

# Binary version (pre-compiled, faster installation)
yay -S binvec-bin
```

For other distributions, contributions to package BinVec are welcome!

## Building from Source

BinVec is written in Rust. To build from source:

1. **Install Rust**: Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Clone the repository**:
   ```bash
   git clone https://github.com/RouHim/binvec.git
   cd binvec
   ```

3. **Build the project**:
   ```bash
   # For Linux
   cargo build --release
   
   # For Windows (requires MinGW-w64)
   cargo build --release --target x86_64-pc-windows-gnu
   ```

4. **Run the application**:
   ```bash
   # For Linux
   ./target/release/binvec
   
   # For Windows
   ./target/x86_64-pc-windows-gnu/release/binvec.exe
   ```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
