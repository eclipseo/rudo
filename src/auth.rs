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

use crate::config;
use crate::session;
use crate::tty;
use crate::user;

use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};
use std::error::Error;
use std::fs;
use std::path::Path;

static SESSION_DIR: &str = "/run/rudo/";

pub fn authentification(
    conf: &config::Config,
    userdata: &user::User,
) -> Result<(), Box<dyn Error>> {
    // Verify that the user is authorize to run rudo
    debug!("User verification begin");
    userdata.verify_user(conf.userlist.as_str())?;
    debug!("User verification finish");

    // Verify that the user is a member of the privilege group for privilege access
    debug!("Group verification begin");
    userdata.verify_group(conf.group.as_str())?;
    debug!("Group verification finish");
    Ok(())
}

pub fn auth_pam(
    conf: &config::Config,
    userdata: &user::User,
) -> Result<Context<Conversation>, Box<dyn Error>> {
    // Create the pam context
    debug!("Pam context creation");
    let mut context = Context::new(
        "rudo",
        Some(userdata.username.as_str()), // Give the name of the actual user
        Conversation::new(),
    )?;
    debug!("Pam context create");

    debug!("extract tty name");
    let tty_name = tty::get_tty_name()?;
    debug!("tty name has been extract");

    debug!("token_path will be create");
    let token_path = format!("{}{}{}", SESSION_DIR, &userdata.username, tty_name);
    let token_path = Path::new(&token_path);
    debug!("token_path has been create: {:?}", token_path);

    let mut result = false;
    debug!("Verifying if token_path exist");
    if token_path.exists() && token_path.is_file() {
        debug!("Will determine uuid of the terminal");
        let tty_uuid = tty::tty_uuid()?;
        debug!("Terminal uuid is {}", tty_uuid);

        debug!("Token will be read from file");
        let token = session::read_token_file(token_path.to_str().unwrap());

        if token.is_ok() {
        debug!("Token has been read from file");
        result = match token.unwrap().verify_token(&tty_name, tty_uuid) {
            Ok(()) => true,
            Err(err) => {
                info!("{}", err);
                false
            }
        };
        }
    } else if token_path.exists() && token_path.is_dir() {
        debug!("token_path is a directory and will be erase");
        fs::remove_dir(token_path)?;
    }

    debug!("Asking for password if token is invalid");
    if !result {
        // Don't ask for password if false in the conf
        info!("{} demand authorization to use rudo", userdata.username);
        if conf.password {
            // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
            debug!("Password will be ask");
            let mut count = 0;
            while count < 3 {
                match context.authenticate(Flag::DISALLOW_NULL_AUTHTOK) {
                    Ok(()) => break,
                    Err(err) => {
                        info!("Password was incorrect");
                        eprintln!("Error: {}", err);
                        count += 1
                    }
                }
            }
            info!("Password was given and validate");
        }
        // Validate the account (is not locked, expired, etc.)
        debug!("Validate the account");
        context.acct_mgmt(Flag::DISALLOW_NULL_AUTHTOK)?;
        debug!("Account validate");

        debug!("Creating run directory");
        session::create_dir_run(&userdata.username)?;
        debug!("Run directory has been create");

        debug!("Getting tty name");
        let tty_name = tty::get_tty_name()?;
        debug!("tty name was get");

        debug!("Will determine uuid of the terminal");
        let tty_uuid = tty::tty_uuid()?;
        debug!("Terminal uuid is {}", tty_uuid);

        debug!("Creating a new Token");
        let token = session::Token::new(tty_name, tty_uuid);
        debug!("Token was create. Will write it to file");
        token.create_token_file(&userdata.username)?;
        debug!("Token was writing to file");
    }

    // Change the user to root to have privilege access
    debug!("Change the user");
    context.set_user(Some(conf.user.as_str()))?;
    debug!("User change");

    Ok(context)
}
