//    Rudo is a program to get privilege access on unix system
//    Copyright (C) 2021  Rémi Lauzier <remilauzier@protonmail.com>
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
#[macro_use]
extern crate log;

mod cli;
mod config;

use env_logger::{Builder, Env};
use log::LevelFilter;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Logs
    let env = Env::new().filter("RUDO_LOG");
    let mut builder = Builder::from_env(env);
    builder.filter_level(LevelFilter::Info);
    builder.init();
    info!("Starting logs");

    // Initialize the cli interface with clap
    debug!("Starting CLI initialization");
    let matches = cli::init_cli();
    debug!("CLI initialize");

    // Decide witch option to run with CLI
    if matches.is_present("shell") {
        // Run the shell
        debug!("Shell command detect");
        rudo::run_shell(matches)?;
        debug!("Shell finish");
    } else if matches.is_present("command") {
        // Run the command
        debug!("Run the supply command");
        rudo::run_command(matches)?;
        debug!("Command finish")
    }
    // End of program
    info!("End of program");
    Ok(())
}
