// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::Path;

use tauri_winres::WindowsResource;

fn main() {
    let manifest = Path::new("res")
        .join("windows")
        .join("manifest.xml")
        .to_path_buf();

    let target_os =
        &dotenvy::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not found in environment");
    if target_os == "windows" {
        let mut res = WindowsResource::new();
        res.set_manifest_file(manifest.to_string_lossy().as_ref());
        res.compile().expect("Failed to compile resource");
    }
}
