# BinVec

## Windows cross-compile hint

Install `mingw-w64-gcc`: https://archlinux.org/packages/?name=mingw-w64-gcc

Enable pkg-config to cross-compile for Windows.

```sh
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/i686-w64-mingw32/lib/pkgconfig
```

Then build with:

```sh
cargo build --release --target x86_64-pc-windows-gnu
```