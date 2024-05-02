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

use crossterm::style::Stylize;
use winreg::{enums::*, RegKey};

use crate::types::Throwable;

const VRCHAT_EXE: &str = "VRChat";

pub(super) enum Priority {
    High = 3,
    AboveNormal = 6,
    Normal = 2,
    BelowNormal = 5,
    Low = 1,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Self::High => format!("High"),
            Self::AboveNormal => format!("Above normal"),
            Self::Normal => format!("Normal"),
            Self::BelowNormal => format!("Below normal"),
            Self::Low => format!("Low"),
        }
    }
}

impl From<u32> for Priority {
    fn from(priority: u32) -> Self {
        match priority {
            3 => Self::High,
            6 => Self::AboveNormal,
            2 => Self::Normal,
            5 => Self::BelowNormal,
            1 => Self::Low,
            _ => panic!("Unknown priority"),
        }
    }
}

pub(super) fn is_running_as_admin() -> Throwable<bool> {
    let session = Command::new("net").arg("session").output()?;
    Ok(session.status.success())
}

pub(super) fn is_vrchat_running() -> Throwable<bool> {
    let tasklist = Command::new("tasklist").output()?;

    let executable = String::from_utf8_lossy(&tasklist.stdout);
    Ok(executable.contains(VRCHAT_EXE))
}

pub(super) fn build_app_info() -> Throwable<()> {
    let mut stdout = io::stdout();

    writeln!(
        stdout,
        "{}\n{} {}\n{}\n{}",
        "Easy Anti-Cheat prevents you from setting the priority of VRChat while it is running."
            .blue(),
        "This program allows you to set the priority of VRChat".blue(),
        "on startup.".blue().underlined(),
        "By doing so, you can bypass this restriction completely.".blue(),
        "Note: Make sure to run this program before launching VRChat.".dark_blue()
    )?;
    writeln!(
        stdout,
        "\n{}\n{}\n{}\n{}\n{}\n{}\n\n{}",
        "Set VRChat priority:".yellow(),
        "3: High (recommended)".yellow(),
        "6: Above normal".yellow(),
        "2: Normal".yellow(),
        "5: Below normal".yellow(),
        "1: Low".yellow(),
        "'Realtime' priority is not possible due to Windows limitations and is not recommended for general CPU stability.".red()
    )?;

    Ok(())
}

pub(super) fn set_cpu_priority_class_for_vrchat(priority: u32) -> Throwable<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version = hklm.open_subkey_with_flags(
        Path::new("SOFTWARE")
            .join("Microsoft")
            .join("Windows NT")
            .join("CurrentVersion"),
        KEY_ALL_ACCESS,
    )?;
    let image_file_execution_options =
        current_version.open_subkey_with_flags("Image File Execution Options", KEY_ALL_ACCESS)?;

    let vrchat_exe = image_file_execution_options
        .create_subkey_with_flags(format!("{VRCHAT_EXE}.exe"), KEY_ALL_ACCESS)?;
    let vrchat_exe_perf_options = match vrchat_exe
        .0
        .open_subkey_with_flags("PerfOptions", KEY_ALL_ACCESS)
    {
        Ok(perfoptions) => perfoptions,
        Err(_) => {
            vrchat_exe
                .0
                .create_subkey_with_flags("PerfOptions", KEY_ALL_ACCESS)?
                .0
        }
    };
    Ok(vrchat_exe_perf_options.set_value("CpuPriorityClass", &priority)?)
}
