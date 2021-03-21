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
#[macro_use]
extern crate serde;

mod cli;
mod config;
mod journal;

use std::error::Error;
use std::path::Path;

static JOURNALD_PATH: &str = "/run/systemd/journal/";

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the cli interface with clap
    let matches = cli::init_cli();
    let debug = matches.is_present("debug");

    if Path::new(JOURNALD_PATH).exists() {
        journal::log_journald(debug)?;
    } else {
        eprintln!("Missing journald file");
    }

    rudo::run(matches)?;

    // End of program
    info!("End of program");
    Ok(())
}
