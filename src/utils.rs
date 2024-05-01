// Copyright (C) 2024 Kawaxte
//
// This file is part of vrc-priority-rs.
//
// vrc-priority-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// vrc-priority-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with vrc-priority-rs.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

use nu_ansi_term::Color;
use winreg::{enums::*, RegKey};

const EXECUTABLE: &str = "VRChat";

pub(super) fn is_running_as_admin() -> bool {
    let session = Command::new("net")
        .arg("session")
        .output()
        .expect("Failed to execute command");
    session.status.success()
}

pub(super) fn is_vrchat_running() -> bool {
    let tasklist = Command::new("tasklist")
        .output()
        .expect("Failed to execute command");

    let vrchat_exe = String::from_utf8_lossy(&tasklist.stdout);
    vrchat_exe.contains(EXECUTABLE)
}

pub(super) fn build_app_info() {
    let stdout = io::stdout();

    let mut handle = stdout.lock();

    writeln!(
        handle,
        "{}\n{} {}\n{}\n{}\n",
        Color::LightBlue.paint(
            "Easy Anti-Cheat prevents you from setting the priority of VRChat while it is running."
        ),
        Color::LightBlue.paint("This program allows you to set the priority of VRChat"),
        Color::LightBlue.underline().paint("on startup."),
        Color::LightBlue.paint("By doing so, you can bypass this restriction completely."),
        Color::Blue.paint("Note: Make sure to run this program before launching VRChat.")
    )
    .expect("Failed to write to stdout");
    writeln!(
        handle,
        "{}\n{}\n{}\n{}\n{}\n{}\n\n{}\n",
        Color::Yellow.paint("Set VRChat priority:"),
        Color::Yellow.paint("3: High (recommended)"),
        Color::Yellow.paint("6: Above normal"),
        Color::Yellow.paint("2: Normal"),
        Color::Yellow.paint("5: Below normal"),
        Color::Yellow.paint("1: Low"),
        Color::Red.paint("'Realtime' priority is not possible due to Windows limitations and is not recommended for general CPU stability.")
    ).expect("Failed to write to stdout");
}

pub(super) fn set_cpu_priority_class_for_vrchat(priority: u32) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version = hklm
        .open_subkey_with_flags(
            Path::new("SOFTWARE")
                .join("Microsoft")
                .join("Windows NT")
                .join("CurrentVersion"),
            KEY_ALL_ACCESS,
        )
        .expect("Failed to open 'CurrentVersion' key");
    let image_file_execution_options = current_version
        .open_subkey_with_flags("Image File Execution Options", KEY_ALL_ACCESS)
        .expect("Failed to open 'Image File Execution Options' key");

    let vrchat_exe = image_file_execution_options
        .create_subkey_with_flags(format!("{EXECUTABLE}.exe"), KEY_ALL_ACCESS)
        .expect("Failed to create subkey for VRChat");
    let vrchat_exe_perf_options = match vrchat_exe
        .0
        .open_subkey_with_flags("PerfOptions", KEY_ALL_ACCESS)
    {
        Ok(perfoptions) => perfoptions,
        Err(_) => {
            vrchat_exe
                .0
                .create_subkey_with_flags("PerfOptions", KEY_ALL_ACCESS)
                .expect("Failed to create 'PerfOptions' key")
                .0
        }
    };
    vrchat_exe_perf_options
        .set_value("CpuPriorityClass", &priority)
        .expect("Failed to set 'CpuPriorityClass' for VRChat");
}
