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

mod auth;
mod cli;
mod command;
mod config;
mod journal;
mod session;
mod tty;
mod user;

use clap::ArgMatches;
use pam_client::Flag;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

static CONFIG_PATH: &str = "/etc/rudo.conf";

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize rudo.conf configuration
    debug!("Start configuration with rudo.conf");
    let path = PathBuf::from(CONFIG_PATH);
    let conf = config::init_conf(&path)?;
    debug!("Configuration has been initialize");

    // Update configuration if necessary as CLI as the priority
    debug!("Update configuration with CLI option");
    let conf = config::Config::update(conf, &matches);
    debug!("Configuration has been update");

    // Create the user data for later use
    debug!("User information extract");
    let userdata = user::User::new();
    debug!("User extraction finish");

    // Greeting to the user
    if conf.greeting {
        debug!("Start user greeting");
        println!("Hello {}!", userdata.username);
        debug!("User greeting finish");
    }

    debug!("Authenticate the user");
    auth::authentification(&conf, &userdata)?;
    debug!("User has been authenticate");

    // Create the pam context
    debug!("Pam context initialization and identification of user");
    let mut context = auth::auth_pam(&conf, &userdata)?;
    debug!("Pam context create and user authenticate");

    // Open session and initialize credentials
    debug!("Session initialize with credential");
    let session = context.open_session(Flag::NONE)?;
    debug!("Session has been create");

    if matches.is_present("command") {
        // Extract the command in two part. First the name of the program then it's arguments.
        debug!("Extracting the supply command for further use");
        let command: Vec<&str> = matches.values_of("command").unwrap().collect();
        let data = command::Command::new(command).unwrap();
        debug!("Extraction has finish");
        // Run a process in the PAM environment and pass the command and the arguments
        info!(
            "{} has been authorized. Command: {} {:?}",
            userdata.username, data.program, data.args
        );
        debug!("Start supply command");
        let mut child = Command::new(data.program)
            .args(data.args)
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .spawn()?;

        // Wait for the command to finish or the terminal end before
        child.wait()?;
        debug!("End of the supply command");
    } else if matches.is_present("shell") {
        // Run a process in the PAM environment and create a new shell
        info!("{} has been authorized. Shell granted", userdata.username);
        debug!("Starting shell");
        let mut child = Command::new(conf.shell)
            .arg("-l") // Login shell
            .arg("-p") // Necessary to have privilege in the new shell
            .env_clear() // Clear env of the user
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .spawn()?;

        // Wait for the command to finish or the terminal end before
        child.wait()?;
        debug!("End of the shell");
    }

    Ok(())
}
