## [1.9.15](https://github.com/RouHim/binvec/compare/1.9.14...1.9.15) (2025-06-17)


### Bug Fixes

* remove sudo from pacman commands in AUR pipeline ([1d031c6](https://github.com/RouHim/binvec/commit/1d031c6a62f3d32f76a881180d208596f7f865da))

## [1.9.14](https://github.com/RouHim/binvec/compare/1.9.13...1.9.14) (2025-06-17)


### Bug Fixes

* streamline AUR pipeline by consolidating package installations and adding jq ([ccfcaef](https://github.com/RouHim/binvec/commit/ccfcaef38621684259291e5c5049391cc93b4b5d))

## [1.9.13](https://github.com/RouHim/binvec/compare/1.9.12...1.9.13) (2025-06-16)


### Bug Fixes

* update AUR pipeline to use Arch Linux container and install necessary packages with pacman ([3a5fa5d](https://github.com/RouHim/binvec/commit/3a5fa5d31ccd79254e62a667580516538c633f95))

## [1.9.12](https://github.com/RouHim/binvec/compare/1.9.11...1.9.12) (2025-06-16)


### Bug Fixes

* update AUR installation steps to use build-essential and add rust toolchain ([38504dd](https://github.com/RouHim/binvec/commit/38504ddce4bfb7d5299812ce4329f0f89b41ab7e))

## [1.9.11](https://github.com/RouHim/binvec/compare/1.9.10...1.9.11) (2025-06-16)


### Bug Fixes

* add installation of necessary packages for AUR in pipeline ([10cb2c1](https://github.com/RouHim/binvec/commit/10cb2c1ea6281a876ef807e52f6e9e2370e89628))

## [1.9.10](https://github.com/RouHim/binvec/compare/1.9.9...1.9.10) (2025-06-16)


### Bug Fixes

* update window settings and dependencies in main application ([7fcda4d](https://github.com/RouHim/binvec/commit/7fcda4dba6fcff7aa4e3d3224d9f7d51cc85adf0))

## [1.9.9](https://github.com/RouHim/binvec/compare/1.9.8...1.9.9) (2025-06-16)


### Bug Fixes

* update version handling and source URLs in PKGBUILD for Arch Linux packages ([af4b4c9](https://github.com/RouHim/binvec/commit/af4b4c9ef5560795e9c532a731a944f19524537b))

## [1.9.8](https://github.com/RouHim/binvec/compare/1.9.7...1.9.8) (2025-06-16)


### Bug Fixes

* update binary source URLs and installation logic for Arch Linux architectures ([772ebfa](https://github.com/RouHim/binvec/commit/772ebfaabe03251669a41f86ad2bac42acf05104))

## [1.9.7](https://github.com/RouHim/binvec/compare/1.9.6...1.9.7) (2025-06-16)


### Bug Fixes

* correct escape sequence in build script for CARGO_BUILD_TARGET extraction ([b627922](https://github.com/RouHim/binvec/commit/b62792282d9e1e87884f46ce2321192b527ac97d))

## [1.9.6](https://github.com/RouHim/binvec/compare/1.9.5...1.9.6) (2025-06-16)


### Bug Fixes

* update version substitution syntax in Cargo.toml ([4329407](https://github.com/RouHim/binvec/commit/43294071f6ab0147135e8ab73a763d7455a76e0f))

## [1.9.5](https://github.com/RouHim/binvec/compare/v1.9.4...1.9.5) (2025-06-16)


### Bug Fixes

* add tagFormat configuration to .releaserc ([287b24b](https://github.com/RouHim/binvec/commit/287b24bbd56ace39569d76b305f4146fb3ac8d78))

## [1.9.4](https://github.com/RouHim/binvec/compare/v1.9.3...v1.9.4) (2025-06-16)


### Bug Fixes

* improve error handling for fetching latest release information in CI pipeline ([3004ac1](https://github.com/RouHim/binvec/commit/3004ac176dc493ce911d120113c8fa5bf94d9cbc))

## [1.9.3](https://github.com/RouHim/binvec/compare/v1.9.2...v1.9.3) (2025-06-16)


### Bug Fixes

* update binary renaming logic and improve error handling in CI pipeline ([7923b85](https://github.com/RouHim/binvec/commit/7923b85db4a63a3821875c000895019c34815635))

## [1.9.2](https://github.com/RouHim/binvec/compare/v1.9.1...v1.9.2) (2025-06-16)


### Bug Fixes

* update pkgver to use VERSION variable in PKGBUILD files ([2ecc685](https://github.com/RouHim/binvec/commit/2ecc68598fbf384b989f135827cd72bcd6911ab1))

## [1.9.1](https://github.com/RouHim/binvec/compare/v1.9.0...v1.9.1) (2025-06-15)

# [1.9.0](https://github.com/RouHim/binvec/compare/v1.8.0...v1.9.0) (2025-06-15)


### Features

* add AUR binary package publishing workflow ([855b9f3](https://github.com/RouHim/binvec/commit/855b9f3f40a39a6dd0bf40be83c7b177e40275cf))

# [1.8.0](https://github.com/RouHim/binvec/compare/v1.7.0...v1.8.0) (2025-06-15)


### Features

* remove riscv64gc target from build matrix in pipeline configuration ([a0b85cd](https://github.com/RouHim/binvec/commit/a0b85cdf480c4422ca62c0c3d4252fb5ad0885d8))

# [1.7.0](https://github.com/RouHim/binvec/compare/v1.6.21...v1.7.0) (2025-06-15)


### Features

* add AUR publishing workflow and update README with installation instructions ([efdc6b7](https://github.com/RouHim/binvec/commit/efdc6b70e1b01882221a216d45d6f973b3db764b))
* add build feature branch workflow for AUR package publishing ([66c75b1](https://github.com/RouHim/binvec/commit/66c75b1e595d0ea0beb9fd8e2f06467812a4233b))
* add initial implementation of Iced GUI for image processing and SVG generation ([4a57ed0](https://github.com/RouHim/binvec/commit/4a57ed0c057f42eb8f42be5fe8859562c96102cd))
* enhance AUR publishing workflow with binary package support and update README ([38d74e4](https://github.com/RouHim/binvec/commit/38d74e4d611db25276d91f56e5c1fd79b7ca9176))
* enhance Iced GUI with async image loading and SVG rendering ([3a6cd4a](https://github.com/RouHim/binvec/commit/3a6cd4a66e125a19945eb3850bccbf59a2a4b980))
* enhance Iced GUI with vector image configuration and save functionality ([9fed7d9](https://github.com/RouHim/binvec/commit/9fed7d9eb14283c296026b46b32dc7b301b502d3))
* expand image format support in file dialog and update installation instructions ([dbe5f54](https://github.com/RouHim/binvec/commit/dbe5f54d186627b6885c85f54fa6eefc9e1c3d50))
* implement self-update functionality and improve CI workflow ([ef281be](https://github.com/RouHim/binvec/commit/ef281be8888128e8c4d6699b3d002a1f89e28ac9))
* refactor SVG generation to use VectorImageConfig for improved configuration management ([c8868a8](https://github.com/RouHim/binvec/commit/c8868a863ab1b410f013af34f4d6982983e0bfb4))
* refactor SVG rendering logic and update self-update messages ([3f73d41](https://github.com/RouHim/binvec/commit/3f73d4183b9e28833698b20f85caa1b4e6c566a6))
* remove npm cache configuration from setup-node action in pipeline ([fd0be9a](https://github.com/RouHim/binvec/commit/fd0be9a1e4b145454d457c5410f70d297ff56544))
* update Iced GUI to improve vector image configuration and UI layout ([fdfd824](https://github.com/RouHim/binvec/commit/fdfd824a87f33200bb624510e165a557e050bd6f))

## [1.6.21](https://github.com/RouHim/binvec/compare/v1.6.20...v1.6.21) (2024-12-02)

## [1.6.20](https://github.com/RouHim/binvec/compare/v1.6.19...v1.6.20) (2024-12-02)

## [1.6.19](https://github.com/RouHim/binvec/compare/v1.6.18...v1.6.19) (2024-11-17)

## [1.6.18](https://github.com/RouHim/binvec/compare/v1.6.17...v1.6.18) (2024-11-17)

## [1.6.17](https://github.com/RouHim/binvec/compare/v1.6.16...v1.6.17) (2024-11-10)

## [1.6.16](https://github.com/RouHim/binvec/compare/v1.6.15...v1.6.16) (2024-11-10)

## [1.6.15](https://github.com/RouHim/binvec/compare/v1.6.14...v1.6.15) (2024-11-09)

## [1.6.14](https://github.com/RouHim/binvec/compare/v1.6.13...v1.6.14) (2024-10-22)

## [1.6.13](https://github.com/RouHim/binvec/compare/v1.6.12...v1.6.13) (2024-10-22)

## [1.6.12](https://github.com/RouHim/binvec/compare/v1.6.11...v1.6.12) (2024-10-21)

## [1.6.11](https://github.com/RouHim/binvec/compare/v1.6.10...v1.6.11) (2024-10-21)

## [1.6.10](https://github.com/RouHim/binvec/compare/v1.6.9...v1.6.10) (2024-10-21)

## [1.6.9](https://github.com/RouHim/binvec/compare/v1.6.8...v1.6.9) (2024-10-20)

## [1.6.8](https://github.com/RouHim/binvec/compare/v1.6.7...v1.6.8) (2024-10-20)


### Bug Fixes

* updater ([f9f73a8](https://github.com/RouHim/binvec/commit/f9f73a81af9dce2db18fc58295131367e651360d))

## [1.6.7](https://github.com/RouHim/binvec/compare/v1.6.6...v1.6.7) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([0065c72](https://github.com/RouHim/binvec/commit/0065c72a68981af436f641893ce919b7a88e0e0d))

## [1.6.6](https://github.com/RouHim/binvec/compare/v1.6.5...v1.6.6) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([5f035ff](https://github.com/RouHim/binvec/commit/5f035ff5965b0a821fc29974f53a5c07a296eece))

## [1.6.5](https://github.com/RouHim/binvec/compare/v1.6.4...v1.6.5) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([50e811d](https://github.com/RouHim/binvec/commit/50e811dde26511a55d25247c803010aeb671e6bc))

## [1.6.4](https://github.com/RouHim/binvec/compare/v1.6.3...v1.6.4) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([33a7f04](https://github.com/RouHim/binvec/commit/33a7f04d7981fdabc3ba0e1c50ed2a87847bfb55))

## [1.6.3](https://github.com/RouHim/binvec/compare/v1.6.2...v1.6.3) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([e097ce2](https://github.com/RouHim/binvec/commit/e097ce2fab4ddc9535d75c5ae649898b90f96e18))

## [1.6.2](https://github.com/RouHim/binvec/compare/v1.6.1...v1.6.2) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([a7fd4c2](https://github.com/RouHim/binvec/commit/a7fd4c224bfa15dd35b7ea1c27b6be28f60bd5e8))

## [1.6.1](https://github.com/RouHim/binvec/compare/v1.6.0...v1.6.1) (2024-10-20)


### Bug Fixes

* ci build for tauri 2 ([96eb90a](https://github.com/RouHim/binvec/commit/96eb90ab519f7316ce733ac2c435130da8f788b5))

# [1.6.0](https://github.com/RouHim/binvec/compare/v1.5.0...v1.6.0) (2024-10-20)


### Features

* update to tauri 2 ([57c9e38](https://github.com/RouHim/binvec/commit/57c9e381370f75878dcc81ed404aa40547213c2c))

# [1.5.0](https://github.com/RouHim/binvec/compare/v1.4.0...v1.5.0) (2024-10-20)


### Features

* update to tauri 2 ([6a9d364](https://github.com/RouHim/binvec/commit/6a9d364ce900e029c3af796339645059db0f2633))

# [1.4.0](https://github.com/RouHim/binvec/compare/v1.3.32...v1.4.0) (2024-10-20)


### Features

* update to tauri 2 ([d1e0046](https://github.com/RouHim/binvec/commit/d1e00468da749cf2c4f3ff97f11a099ad5ecbf35))
* update to tauri 2 ([bf0a776](https://github.com/RouHim/binvec/commit/bf0a776c10cbdc6b8f70126ddc95e1ff04d2dee2))

## [1.3.32](https://github.com/RouHim/binvec/compare/v1.3.31...v1.3.32) (2024-10-19)

## [1.3.31](https://github.com/RouHim/binvec/compare/v1.3.30...v1.3.31) (2024-10-19)

## [1.3.30](https://github.com/RouHim/binvec/compare/v1.3.29...v1.3.30) (2024-10-18)

## [1.3.29](https://github.com/RouHim/binvec/compare/v1.3.28...v1.3.29) (2024-10-18)

## [1.3.28](https://github.com/RouHim/binvec/compare/v1.3.27...v1.3.28) (2024-10-17)

## [1.3.27](https://github.com/RouHim/binvec/compare/v1.3.26...v1.3.27) (2024-10-16)

## [1.3.26](https://github.com/RouHim/binvec/compare/v1.3.25...v1.3.26) (2024-10-15)

## [1.3.25](https://github.com/RouHim/binvec/compare/v1.3.24...v1.3.25) (2024-09-27)

## [1.3.24](https://github.com/RouHim/binvec/compare/v1.3.23...v1.3.24) (2024-09-17)

## [1.3.23](https://github.com/RouHim/binvec/compare/v1.3.22...v1.3.23) (2024-09-17)

## [1.3.22](https://github.com/RouHim/binvec/compare/v1.3.21...v1.3.22) (2024-09-16)

## [1.3.21](https://github.com/RouHim/binvec/compare/v1.3.20...v1.3.21) (2024-09-16)

## [1.3.20](https://github.com/RouHim/binvec/compare/v1.3.19...v1.3.20) (2024-08-24)

## [1.3.19](https://github.com/RouHim/binvec/compare/v1.3.18...v1.3.19) (2024-08-24)

## [1.3.18](https://github.com/RouHim/binvec/compare/v1.3.17...v1.3.18) (2024-08-24)

## [1.3.17](https://github.com/RouHim/binvec/compare/v1.3.16...v1.3.17) (2024-08-23)

## [1.3.16](https://github.com/RouHim/binvec/compare/v1.3.15...v1.3.16) (2024-08-21)

## [1.3.15](https://github.com/RouHim/binvec/compare/v1.3.14...v1.3.15) (2024-07-07)

## [1.3.14](https://github.com/RouHim/binvec/compare/v1.3.13...v1.3.14) (2024-07-02)

## [1.3.13](https://github.com/RouHim/binvec/compare/v1.3.12...v1.3.13) (2024-07-02)

## [1.3.12](https://github.com/RouHim/binvec/compare/v1.3.11...v1.3.12) (2024-07-02)

## [1.3.11](https://github.com/RouHim/binvec/compare/v1.3.10...v1.3.11) (2024-07-01)

## [1.3.10](https://github.com/RouHim/binvec/compare/v1.3.9...v1.3.10) (2024-06-30)

## [1.3.9](https://github.com/RouHim/binvec/compare/v1.3.8...v1.3.9) (2024-06-25)

## [1.3.8](https://github.com/RouHim/binvec/compare/v1.3.7...v1.3.8) (2024-06-06)

## [1.3.7](https://github.com/RouHim/binvec/compare/v1.3.6...v1.3.7) (2024-06-06)

## [1.3.6](https://github.com/RouHim/binvec/compare/v1.3.5...v1.3.6) (2024-06-06)

## [1.3.5](https://github.com/RouHim/binvec/compare/v1.3.4...v1.3.5) (2024-06-06)

## [1.3.4](https://github.com/RouHim/binvec/compare/v1.3.3...v1.3.4) (2024-06-05)

## [1.3.3](https://github.com/RouHim/binvec/compare/v1.3.2...v1.3.3) (2024-06-05)

## [1.3.2](https://github.com/RouHim/binvec/compare/v1.3.1...v1.3.2) (2024-06-05)


### Bug Fixes

* **pipeline:** remove unsed dep check ([ca0dd50](https://github.com/RouHim/binvec/commit/ca0dd50a048d6db75b734ad5e5a4ea3f38c82aa7))

## [1.3.1](https://github.com/RouHim/binvec/compare/v1.3.0...v1.3.1) (2024-03-14)

# [1.3.0](https://github.com/RouHim/binvec/compare/v1.2.17...v1.3.0) (2024-03-13)


### Features

* update 3rd party libs ([37c6cd1](https://github.com/RouHim/binvec/commit/37c6cd17d1a6011b1f20a2ccf73160cb1405eee4))

## [1.2.17](https://github.com/RouHim/binvec/compare/v1.2.16...v1.2.17) (2024-02-22)


### Bug Fixes

* **pipeline:** use correct node version for sem release ([9facfec](https://github.com/RouHim/binvec/commit/9facfec342d7f21b30695011c206bc2e75bd26ff))

## [1.2.16](https://github.com/RouHim/binvec/compare/v1.2.15...v1.2.16) (2023-12-20)

## [1.2.15](https://github.com/RouHim/binvec/compare/v1.2.14...v1.2.15) (2023-10-23)

## [1.2.14](https://github.com/RouHim/binvec/compare/v1.2.13...v1.2.14) (2023-09-30)

## [1.2.13](https://github.com/RouHim/binvec/compare/v1.2.12...v1.2.13) (2023-09-30)

## [1.2.12](https://github.com/RouHim/binvec/compare/v1.2.11...v1.2.12) (2023-09-29)

## [1.2.11](https://github.com/RouHim/binvec/compare/v1.2.10...v1.2.11) (2023-09-28)

## [1.2.10](https://github.com/RouHim/binvec/compare/v1.2.9...v1.2.10) (2023-09-28)

## [1.2.9](https://github.com/RouHim/binvec/compare/v1.2.8...v1.2.9) (2023-09-27)

## [1.2.8](https://github.com/RouHim/binvec/compare/v1.2.7...v1.2.8) (2023-09-19)

## [1.2.7](https://github.com/RouHim/binvec/compare/v1.2.6...v1.2.7) (2023-09-17)

## [1.2.6](https://github.com/RouHim/binvec/compare/v1.2.5...v1.2.6) (2023-09-14)

## [1.2.5](https://github.com/RouHim/binvec/compare/v1.2.4...v1.2.5) (2023-09-09)

## [1.2.4](https://github.com/RouHim/binvec/compare/v1.2.3...v1.2.4) (2023-09-04)

## [1.2.3](https://github.com/RouHim/binvec/compare/v1.2.2...v1.2.3) (2023-08-29)


### Bug Fixes

* **pipeline:** use angular commit schema ([0f2f4e0](https://github.com/RouHim/binvec/commit/0f2f4e08910a264948b6e3c812223cfcbdf6a482))

## [1.2.2](https://github.com/RouHim/binvec/compare/v1.2.1...v1.2.2) (2023-08-26)

## [1.2.1](https://github.com/RouHim/binvec/compare/v1.2.0...v1.2.1) (2023-08-26)

# [1.2.0](https://github.com/RouHim/binvec/compare/v1.1.16...v1.2.0) (2023-08-25)


### Features

* **ui:** add app version to title ([847e16a](https://github.com/RouHim/binvec/commit/847e16a8aa20a41360402a9a5833d98f6ea4b0fe))

## [1.1.16](https://github.com/RouHim/binvec/compare/v1.1.15...v1.1.16) (2023-08-25)


### Bug Fixes

* **pipeline:** remove macos build for now ([b9030f1](https://github.com/RouHim/binvec/commit/b9030f12559e0bddd55d184633a4134a616d3b3e))

## [1.1.15](https://github.com/RouHim/binvec/compare/v1.1.14...v1.1.15) (2023-08-25)


### Bug Fixes

* **pipeline:** debug macos upload ([049e74f](https://github.com/RouHim/binvec/commit/049e74fc58c3ef5513e17f957e56a9d2f6af65e6))

## [1.1.14](https://github.com/RouHim/binvec/compare/v1.1.13...v1.1.14) (2023-08-25)


### Bug Fixes

* **pipeline:** debug macos upload ([2658a1a](https://github.com/RouHim/binvec/commit/2658a1a2b03097c60b0b388d3a3aee80b973abf0))

## [1.1.13](https://github.com/RouHim/binvec/compare/v1.1.12...v1.1.13) (2023-08-25)


### Bug Fixes

* **pipeline:** wrong macos ending ([5ec5571](https://github.com/RouHim/binvec/commit/5ec5571fe89b02f518bae6b6c3b8d2282bfe00da))

## [1.1.12](https://github.com/RouHim/binvec/compare/v1.1.11...v1.1.12) (2023-08-25)


### Bug Fixes

* **pipeline:** debug macos ([ea00541](https://github.com/RouHim/binvec/commit/ea00541d58b97748f6fc5f991fb8faab45c1f28a))
* **pipeline:** debug macos ([8b068cd](https://github.com/RouHim/binvec/commit/8b068cda64b343dd72a810e43ebd49b9e411d585))
* **pipeline:** debug macos ([992f225](https://github.com/RouHim/binvec/commit/992f22520148f6eb936d35be4d99606037dcbcec))

## [1.1.11](https://github.com/RouHim/binvec/compare/v1.1.10...v1.1.11) (2023-08-25)


### Bug Fixes

* **pipeline:** macos version detection ([c6b39f2](https://github.com/RouHim/binvec/commit/c6b39f25effa96542b8ab9df35bd1e5c7a41c6b4))

## [1.1.10](https://github.com/RouHim/binvec/compare/v1.1.9...v1.1.10) (2023-08-25)


### Bug Fixes

* **pipeline:** macos bundling ([61264f9](https://github.com/RouHim/binvec/commit/61264f93b6318f7ec5bcae1f4a7946e833e35573))

## [1.1.9](https://github.com/RouHim/binvec/compare/v1.1.8...v1.1.9) (2023-08-25)


### Bug Fixes

* **pipeline:** release version detection on macos ([0983574](https://github.com/RouHim/binvec/commit/098357498dd2fd3bb9b234ef7d946eeba4b5ff5b))

## [1.1.8](https://github.com/RouHim/binvec/compare/v1.1.7...v1.1.8) (2023-08-25)


### Bug Fixes

* **pipeline:** artifacts names ([9033f87](https://github.com/RouHim/binvec/commit/9033f87787160b12f68aa47e8c6864d0215799d7))

## [1.1.7](https://github.com/RouHim/binvec/compare/v1.1.6...v1.1.7) (2023-08-25)


### Bug Fixes

* **pipeline:** install latest bash on macos ([26aabfa](https://github.com/RouHim/binvec/commit/26aabfaa63143a4aeac465b220ffbc8a5ba5e373))

## [1.1.6](https://github.com/RouHim/binvec/compare/v1.1.5...v1.1.6) (2023-08-25)

## [1.1.5](https://github.com/RouHim/binvec/compare/v1.1.4...v1.1.5) (2023-08-25)

## [1.1.4](https://github.com/RouHim/binvec/compare/v1.1.3...v1.1.4) (2023-08-25)

## [1.1.3](https://github.com/RouHim/binvec/compare/v1.1.2...v1.1.3) (2023-08-25)

## [1.1.2](https://github.com/RouHim/binvec/compare/v1.1.1...v1.1.2) (2023-08-25)


### Bug Fixes

* **pipeline:** use jq instead of sec ([60f4723](https://github.com/RouHim/binvec/commit/60f47239f4d6371f37c92ed7584916da5010e6f3))

## [1.1.1](https://github.com/RouHim/binvec/compare/v1.1.0...v1.1.1) (2023-08-25)


### Bug Fixes

* **pipeline:** awk instead of sed ([19eadfc](https://github.com/RouHim/binvec/commit/19eadfc46686ed634242da657052cd7bad91e05d))

# [1.1.0](https://github.com/RouHim/binvec/compare/v1.0.156...v1.1.0) (2023-08-25)


### Bug Fixes

* **pipeline:** fix binstall ([5b54a8b](https://github.com/RouHim/binvec/commit/5b54a8bba6cf4f4407a75c1b1c5a7b855b49c572))
* **pipeline:** needed step ([b687035](https://github.com/RouHim/binvec/commit/b687035bdb7cf0f8079555833ba4b177768491c0))


### Features

* **pipeline:** move to sem release ([81ec041](https://github.com/RouHim/binvec/commit/81ec041d40c281cd1e9be92a0cd66a819d1a1573))
