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

mod utils;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() {
    // "Access is denied. (os error 5)" without said privileges.
    if !utils::is_running_as_admin() {
        eprintln!("Please run this program with administrator privileges.");

        return;
    }

    // Changing priority during runtime can cause instability.
    if utils::is_vrchat_running() {
        eprintln!("Please close VRChat before running this program again.");

        return;
    }

    let mut input = String::new();

    utils::build_app_info();

    print!("Input: ");

    // Ensure prompt is displayed before reading input.
    io::stdout().flush().expect("Failed to flush stdout");

    // So, what are you going to enter?
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Are you trying to enter words instead of numbers?
    let priority_num = input.trim().parse::<u32>().expect("Failed to parse input");
    if priority_num >= 1 && priority_num <= 6 {
        utils::set_cpu_priority_class_for_vrchat(priority_num);
    } else {
        // Take a break for a second.
        thread::sleep(Duration::from_millis(1000));

        eprint!("Please enter a number between 1 and 6.");

        return;
    }

    print!("VRChat priority has been set to {input}");
}
