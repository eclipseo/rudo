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

mod auth;
mod command;
mod config;
mod session;
mod tty;
mod user;

use clap::ArgMatches;
use pam_client::Flag;
use std::env;
use std::error::Error;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize rudo.conf configuration
    debug!("Start configuration with rudo.conf");
    let conf = config::init_conf()?;
    debug!("Configuration has been initialize");

    // Update configuration if necessary as CLI as the priority
    debug!("Update configuration with CLI option");
    let conf = config::Config::update(conf, &matches);
    debug!("Configuration has been update");

    // Create the user data for later use
    debug!("User information extract");
    let userdata = user::User::new();
    debug!("User extraction finish");

    debug!("Extract uid and gid of the impersonate user");
    let impuser = users::get_user_by_name(&conf.user).unwrap();
    let impuser_uid = impuser.uid();
    let impuser_gid = impuser.primary_group_id();
    debug!("Extraction finish");

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
            .uid(impuser_uid) // Necessary to have full access
            .gid(impuser_gid) // Necessary to have full access
            .spawn()?;

        // Wait for the command to finish or the terminal end before
        child.wait()?;
        debug!("End of the supply command");
    } else if matches.is_present("shell") {
        debug!("Extracting shell environment variable");
        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));
        // Run a process in the PAM environment and create a new shell
        info!("{} has been authorized to use {}", userdata.username, shell);
        debug!("Starting shell");
        let mut child = Command::new(shell)
            .arg("-l") // Login shell
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .uid(impuser_uid) // Necessary to have full access
            .gid(impuser_gid) // Necessary to have full access
            .spawn()?;

        // Wait for the command to finish or the terminal end before
        child.wait()?;
        debug!("End of the shell");
    } else if matches.is_present("edit") {
        debug!("Extracting editor environment variable");
        let editor = env::var("EDITOR")?;
        debug!("Extracting arguments give to the editor");
        let arg = matches.value_of("edit").unwrap();
        info!(
            "{} has been authorized to use {}",
            userdata.username, editor
        );
        debug!("Starting editor");
        let mut child = Command::new(editor)
            .arg(arg)
            .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
            .uid(impuser_uid) // Necessary to have full access
            .gid(impuser_gid) // Necessary to have full access
            .spawn()?;

        // Wait for the command to finish or the terminal end before
        child.wait()?;
        debug!("End of the editor");
    }

    Ok(())
}
