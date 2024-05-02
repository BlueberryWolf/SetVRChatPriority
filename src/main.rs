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

mod types;
mod utils;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use types::Throwable;

use crate::utils::Priority;

fn main() -> Throwable<()> {
    if !utils::is_running_as_admin()? {
        eprint!("Please run this program with administrator privileges.");

        return Ok(());
    }
    if utils::is_vrchat_running()? {
        eprint!("Please close VRChat before running this program again.");

        return Ok(());
    }

    utils::build_app_info()?;

    loop {
        let mut input = format!("");

        print!("\nInput: ");

        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(number) if number >= 1 && number <= 6 => {
                utils::set_cpu_priority_class_for_vrchat(number)?;
                println!(
                    "VRChat priority has been set to {:?}.",
                    Priority::from(number).to_string()
                );

                break;
            }
            _ => {
                thread::sleep(Duration::from_millis(1000));
                eprintln!("{:?} is not a valid priority level.", input.trim());
            }
        }
    }

    Ok(())
}
