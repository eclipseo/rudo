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
use crate::user;
use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};
use std::error::Error;

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

    // Don't ask for password if false in the conf
    info!("{} demand authorization to use rudo", userdata.username);
    if conf.password {
        // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
        debug!("Password ask");
        let mut count = 0;
        while count < 3 {
        match context.authenticate(Flag::DISALLOW_NULL_AUTHTOK) {
            Ok(()) => break,
            Err(err) => {
            eprintln!("Error: {}", err);
            count+=1
        }
        }
        }
        debug!("Password give");
    }

    // Validate the account (is not locked, expired, etc.)
    debug!("Validate the account");
    context.acct_mgmt(Flag::DISALLOW_NULL_AUTHTOK)?;
    debug!("Account validate");

    // Change the user to root to have privilege access
    debug!("Change the user");
    context.set_user(Some(conf.user.as_str()))?;
    debug!("User change");

    Ok(context)
}
