#!/usr/bin/env bash

          mv "./macos/BinVec.app" "release-artifacts/BinVec_${VERSION}.app"
          mv "./macos/BinVec.app.tar.gz" "release-artifacts/BinVec_${VERSION}.app.tar.gz"
          mv "./macos/BinVec.app.tar.gz.sig" "release-artifacts/BinVec_${VERSION}.app.tar.gz.sig"

          mv "./appimage/bin-vec_${VERSION}_amd64.AppImage" "release-artifacts/BinVec_${VERSION}.AppImage"
          mv "./appimage/bin-vec_${VERSION}_amd64.AppImage.tar.gz" "release-artifacts/BinVec_${VERSION}.AppImage.tar.gz"
          mv "./appimage/bin-vec_${VERSION}_amd64.AppImage.tar.gz.sig" "release-artifacts/BinVec_${VERSION}.AppImage.tar.gz.sig"

          mv "./msi/BinVec_${VERSION}_x64_en-US.msi" "release-artifacts/BinVec_${VERSION}.msi"
          mv "./msi/BinVec_${VERSION}_x64_en-US.msi.tar.gz" "release-artifacts/BinVec_${VERSION}.msi.tar.gz"
          mv "./msi/BinVec_${VERSION}_x64_en-US.msi.tar.gz.sig" "release-artifacts/BinVec_${VERSION}.msi.tar.gz.sig"