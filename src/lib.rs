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
mod command;
mod config;
mod user;

use clap::ArgMatches;
use pam_client::Flag;
use std::error::Error;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use users::UsersCache;

// Run the command
pub fn run_command(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize rudo.conf configuration
    debug!("Start configuration with rudo.conf");
    let path = PathBuf::from("/etc/rudo.conf");
    let conf = config::init_conf(&path)?;
    debug!("Configuration initialize");

    // Update configuration if necessary as CLI as the priority
    debug!("Update configuration");
    let conf = config::Config::update(conf, &matches);
    debug!("Configuration has been update");

    // Extract the command in two part. First the name of the program then it's arguments.
    debug!("Extracting the supply command for further use");
    let value: Vec<&str> = matches.values_of("command").unwrap().collect();
    let data = command::Comm::new(value).unwrap();
    debug!("Extraction finish");

    // Create the user data for later use
    debug!("User information extract");
    let userscache = UsersCache::new();
    let userdata = user::User::new(&userscache);
    debug!("User extraction finish");

    // Greeting to the user
    debug!("Start user greeting");
    if conf.greeting {
    println!("Hello {}!", userdata.username);
    }
    debug!("Greeting finish");

    // Verify that the user is authorize to run rudo
    debug!("User verification begin");
    match userdata.verify_user(conf.userlist.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    debug!("User verification finish");

    // Verify that the user is a member of the privilege group for privilege access
    debug!("Group verification begin");
    match userdata.verify_group(conf.group.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    debug!("Group verification finish");

    // Create the pam context
    debug!("Pam context initialization");
    let mut context = auth::auth_pam(&conf, &userdata)?;
    debug!("Pam context create");

    // Open session and initialize credentials
    debug!("Session initialize with credential");
    let session = context.open_session(Flag::NONE)?;
    debug!("Session create");

    // Run a process in the PAM environment and pass the command and the arguments
    info!("{} has been authorized. Command: {} {:?}", userdata.username, data.program, data.args);
    let mut child = Command::new(data.program)
        .args(data.args)
        .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
        .spawn()?;
    // Wait for the command to finish or the terminal end before
    child.wait()?;
    debug!("End of the supply command");

    Ok(())
}

pub fn run_shell(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    // Initialize rudo.conf configuration
    debug!("Start configuration with rudo.conf");
    let path = PathBuf::from("/etc/rudo.conf");
    let conf = config::init_conf(&path)?;
    debug!("Configuration initialize");


    // Update configuration if necessary as CLI as the priority
    debug!("Update configuration");
    let conf = config::Config::update(conf, &matches);
    debug!("Configuration has been update");

    // Create the user data for later use
    debug!("User information extract");
    let userscache = UsersCache::new();
    let userdata = user::User::new(&userscache);
    debug!("User extraction finish");

    // Greeting to the user
    debug!("Start user greeting");
    if conf.greeting {
    println!("Hello {}!", userdata.username);
    }
    debug!("Greeting finish");

    // Verify that the user is authorize to run rudo
    debug!("User verification begin");
    match userdata.verify_user(conf.userlist.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    debug!("User verification finish");

    // Verify that the user is a member of the privilege group for privilege access
    debug!("Group verification begin");
    match userdata.verify_group(conf.group.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    debug!("Group verification finish");

    // Create the pam context
    debug!("Pam context initialization");
    let mut context = auth::auth_pam(&conf, &userdata)?;
    debug!("Pam context create");

    // Open session and initialize credentials
    debug!("Session initialize with credential");
    let session = context.open_session(Flag::NONE)?;
    debug!("Session create");

    // Run a process in the PAM environment and create a new shell
    info!("{} has been authorized. Shell granted", userdata.username);
    let mut child = Command::new(conf.shell)
        .arg("-l")
        .arg("-p") // Necessary to have privilege in the new shell
        .env_clear()
        .envs(session.envlist().iter_tuples()) // Pass the pam session to the new proccess
        .spawn()?;
    // Wait for the command to finish or the terminal end before
    child.wait()?;
    debug!("End of the shell");

    Ok(())
}
