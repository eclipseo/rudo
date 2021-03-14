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

mod auth;
mod command;
mod config;
mod user;

use clap::ArgMatches;
use pam_client::Flag; // pam support
use std::error::Error;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use users::UsersCache; // safe library to extract data for user and group

//Run the main program
pub fn run_command(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from("rudo.conf");
    //Initialize rudo.conf configuration or the default value
    let conf = config::init_conf(&path)?;

    //Update configuration if necessary as CLI as the priority
    let conf = config::Config::update(conf, &matches);

    let value: Vec<&str> = matches.values_of("command").unwrap().collect();

    let data = command::Comm::new(value);

    //Create the user data for later use
    let userscache = UsersCache::new();
    let userdata = user::User::new(&userscache);

    println!("Hello {}!", userdata.username);

    match userdata.verify_user(conf.userlist.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }

    //Verify that the user is in the group for privilege access
    match userdata.verify_group(conf.group.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }

    let mut context = auth::auth_pam(&conf, userdata)?;

    // Open session and initialize credentials
    let session = context.open_session(Flag::NONE)?;

    // Run a process in the PAM environment and pass the command and the arguments
    let mut child = Command::new(data.program)
        .args(data.args)
        .envs(session.envlist().iter_tuples()) //pass the pam session
        .spawn()?;

    child.wait()?; // wait for the command to finish or the terminal end before

    Ok(())
}

pub fn run_shell(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from("rudo.conf");
    //Initialize rudo.conf configuration or the default value
    let conf = config::init_conf(&path)?;

    //Update configuration if necessary as CLI as the priority
    let conf = config::Config::update(conf, &matches);

    //Create the user data for later use
    let userscache = UsersCache::new();
    let userdata = user::User::new(&userscache);

    println!("Hello {}!", userdata.username);

    //Verify that the user is in the group for privilege access
    match userdata.verify_group(conf.group.as_str()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }

    let mut context = auth::auth_pam(&conf, userdata)?;

    // Open session and initialize credentials
    let session = context.open_session(Flag::NONE)?;

    let mut child = Command::new(conf.shell)
        .envs(session.envlist().iter_tuples()) //pass the pam session
        .spawn()?;

    child.wait()?; // wait for the command to finish or the terminal end before

    Ok(())
}
