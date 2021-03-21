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

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, DirBuilder, File};
use std::io::{Read, Write};
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;

static DEFAULT_SESSION_TIMEOUT: u64 = 600;
static SESSION_DIR: &str = "/run/rudo/";

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    tty_name: String,
    timestamp: SystemTime,
    final_timestamp: SystemTime,
}

impl Token {
    pub fn new(tty_name: String) -> Self {
        let timestamp = SystemTime::now();
        let duration = std::time::Duration::from_secs(DEFAULT_SESSION_TIMEOUT);
        let final_timestamp = timestamp.checked_add(duration).unwrap();
        Self {
            tty_name,
            timestamp,
            final_timestamp,
        }
    }
    pub fn create_token_file(&self, username: &str) -> Result<(), Box<dyn Error>> {
        debug!("Creating token_path");
        let token_path = format!("{}{}{}", SESSION_DIR, username, self.tty_name);
        let token_path = Path::new(&token_path);
        debug!(
            "token_path has been create, will verify if it exist : {:?}",
            token_path
        );
        if !token_path.exists() {
            debug!("token_path doesn't exist, will create it");
            let path = token_path.parent().unwrap();
            debug!("Create directory: {:?}", path);
            DirBuilder::new().mode(0o600).recursive(true).create(path)?;
            debug!("Put Token in a string");
            let token_file = serde_json::to_string(&self)?;
            debug!("creating the token file");
            let mut file = File::create(token_path)?;
            debug!("write the string in the file");
            file.write_all(&token_file.as_bytes())?;
            file.sync_all()?;

            debug!("Set file permission");
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            file.set_permissions(perms)?;
            debug!("File permission has been set");
        } else {
            debug!("token_path exist will erase it");
            fs::remove_file(token_path)?;
            debug!("Put Token in a string");
            let token_file = serde_json::to_string(&self)?;
            debug!("creating the token file");
            let mut file = File::create(token_path)?;
            debug!("write the string in the file");
            file.write_all(&token_file.as_bytes())?;
            file.sync_all()?;

            debug!("Set file permission");
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            file.set_permissions(perms)?;
            debug!("File permission has been set");
        }
        Ok(())
    }
    pub fn verify_token(&self, tty_name: &str) -> Result<(), Box<dyn Error>> {
        let clock = SystemTime::now();
        if self.final_timestamp <= clock {
            info!("Session has expired");
            Err(From::from("Session has expired"))
        } else if self.tty_name == tty_name {
            Ok(())
        } else {
            info!("Not the same session");
            Err(From::from("Not the same session"))
        }
    }
}

pub fn create_dir_run(username: &str) -> Result<(), Box<dyn Error>> {
    let run_path = Path::new(SESSION_DIR);
    debug!("Verify that run_path exist");
    if !run_path.exists() {
        debug!("run_path doesn't exist, creating it");
        DirBuilder::new()
            .mode(0o600)
            .recursive(true)
            .create(SESSION_DIR)?;
    }
    let metadata = fs::metadata(SESSION_DIR)?;
    let mut perms = metadata.permissions();
    debug!("Verifying permission on run_path");
    if perms.mode() != 0o600 {
        debug!("Permissions incorect, adjusting it");
        perms.set_mode(0o600);
        fs::set_permissions(SESSION_DIR, perms)?;
    }

    let user_path = format!("{}{}/", SESSION_DIR, username);
    let user_path = Path::new(&user_path);
    debug!("Verifying that user_path exist: {:?}", user_path);
    if !user_path.exists() {
        debug!("user_path doesn't exist, creating it");
        DirBuilder::new()
            .mode(0o600)
            .recursive(true)
            .create(user_path)?;
    } else if user_path.is_file() {
        error!("Error: {:?} is not a directory", user_path);
        let err = format!("Error: {:?} is not a directory", user_path);
        return Err(From::from(err));
    }
    let metadata = fs::metadata(user_path)?;
    let mut perms = metadata.permissions();
    debug!("Verifying user_path permmisions");
    if perms.mode() != 0o600 {
        debug!("Permissions are incorect, adjusting it");
        perms.set_mode(0o600);
        fs::set_permissions(user_path, perms)?;
    }

    Ok(())
}

pub fn read_token_file(token_path: &str) -> Result<Token, Box<dyn Error>> {
    let mut file = File::open(token_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let token: Token = serde_json::from_str(&buffer)?;
    Ok(token)
}
