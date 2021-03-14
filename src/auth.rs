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
use pam_client::conv_cli::Conversation; // CLI implementation of pam
use pam_client::{Context, Flag}; // pam support
use std::error::Error;

pub fn auth_pam(
    conf: &config::Config,
    userdata: user::User,
) -> Result<Context<Conversation>, Box<dyn Error>> {
    // Create the pam context
    let mut context = Context::new(
        "sudo",                           // Service name "sudo" for now
        Some(userdata.username.as_str()), // give the name of the actual user
        Conversation::new(),
    )?;

    if conf.password {
        // Authenticate the user (ask for password, 2nd-factor token, fingerprint, etc.)
        context.authenticate(Flag::DISALLOW_NULL_AUTHTOK)?;
    }

    // Validate the account (is not locked, expired, etc.)
    context.acct_mgmt(Flag::DISALLOW_NULL_AUTHTOK)?;

    // Change the user to root to have privilege access
    context.set_user(Some(conf.user.as_str()))?;

    Ok(context)
}
