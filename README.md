<!--
 Copyright (C) 2024 Kawaxte
 
 This file is part of vrc-priority-rs.
 
 vrc-priority-rs is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 vrc-priority-rs is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with vrc-priority-rs.  If not, see <https://www.gnu.org/licenses/>.
-->

<div align="center">
    <img src="res/example.png" alt="Example" style="width:75%; height:auto;">
</div>

# VRChat Prioritiser

[![gh_release](https://img.shields.io/github/v/release/Kawaxte/vrc-priority-rs?sort=date&logo=github&label=latest&style=for-the-badge)](https://github.com/Kawaxte/vrc-priority-rs/releases/latest)
[![gh_release_pre](https://img.shields.io/github/v/release/Kawaxte/vrc-priority-rs?include_prereleases&sort=date&logo=github&label=pre-release&style=for-the-badge)](https://github.com/Kawaxte/vrc-priority-rs/releases)

VRChat not too long ago added Easy Anti-Cheat, which prevents you from changing the priority of VRChat.exe through Task Manager. All *you* have to do is run it once and it will permanently set the priority of VRChat.exe to what you chose. 簡単簡単！

## Building from Source

[![rust](https://img.shields.io/badge/dynamic/json?logo=rust&label=Rust&color=A72145&style=for-the-badge&query=%24.tag_name&url=https%3A%2F%2Fapi.github.com%2Frepos%2Frust-lang%2Frust%2Freleases%2Flatest)](https://www.rust-lang.org)
[![cargo](https://img.shields.io/badge/cargo-555555?logo=rust&style=for-the-badge)](https://doc.rust-lang.org/cargo)

### Prerequisites

Before you start, make sure you've got [Rust](https://www.rust-lang.org) on your computer. You can check if you've got Rust by typing `rustc --version` in your terminal.

In the meantime, if you're new to Rust, [check out this simplified guide](https://github.com/Dhghomon/easy_rust). I recommend this over the official Rust Book as it's easier to understand and doesn't use programmer jargon that most English speakers, native or not, may not understand.

### Compilation

Begin by typing `cargo build --release` to create an optimised executable, or `cargo build` to create an unoptimised executable with debug symbols.

After that, the `--release` version will be in `target/release` directory while latter will be in `target/debug` directory.

## Licence

[![gh_licence](https://img.shields.io/github/license/Kawaxte/vrc-priority-rs?logo=github&style=for-the-badge)](LICENSE)

This project is licenced under the [GNU General Public License v3.0](LICENSE). You can use the project for any purpose, but you must include the original copyright and licence.
