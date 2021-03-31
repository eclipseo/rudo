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
use std::error::Error;
use std::fs::{self, DirBuilder, File};
use std::io::{Read, Write};
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::SystemTime;

static DEFAULT_SESSION_TIMEOUT: u64 = 600;
static SESSION_DIR: &str = "/run/rudo/";

// Create a struct to containt the uuid of the terminal and the timestamp to determine
// if the session is valid for further use
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    tty_name: String,
    tty_uuid: String,
    timestamp: SystemTime,
    final_timestamp: SystemTime,
}

impl Token {
    pub fn new(tty_name: String, tty_uuid: String) -> Self {
        debug!("Create the timestamp");
        let timestamp = SystemTime::now();
        // Create the timestamp where the session become invalid
        debug!("Create the final timestamp to determine the mas duration of the session");
        let duration = std::time::Duration::from_secs(DEFAULT_SESSION_TIMEOUT);
        let final_timestamp = timestamp.checked_add(duration).unwrap();
        Self {
            tty_name,
            tty_uuid,
            timestamp,
            final_timestamp,
        }
    }
    // Create the file that will contain the token
    pub fn create_token_file(&self, username: &str) -> Result<(), Box<dyn Error>> {
        // Create the path of the file with the name of the program, the username to distinguish user
        // and the name of tty to let user have multiple session, on multiple terminal
        debug!("Creating token_path");
        let token_path = format!("{}{}{}", SESSION_DIR, username, self.tty_name);
        let token_path = Path::new(&token_path);
        debug!(
            "Token_path has been create, will verify if it exist : {:?}",
            token_path
        );

        // Verify the existence of the path to act accordingly
        if !token_path.exists() {
            debug!("Token_path doesn't exist, will create it");
            let path = token_path.parent().unwrap();

            // Create the directory with mode 600 to restreint access
            debug!("Create directory: {:?}", path);
            DirBuilder::new().mode(0o600).recursive(true).create(path)?;

            // Put the token data in a string of yaml
            debug!("Put Token in a string");
            let token_file = serde_yaml::to_string(&self)?;

            // Creating the file for the token
            debug!("creating the token file");
            let mut file = File::create(token_path)?;

            // Write the token data in the file
            debug!("write the string in the file");
            file.write_all(&token_file.as_bytes())?;

            // Sync data to be sure everything is writing on drive
            debug!("Syncing data on drive");
            file.sync_all()?;

            // Put file permission to 600 to restreint access
            debug!("Set file permission to 600");
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            file.set_permissions(perms)?;
            debug!("File permission has been set");
        } else {
            // Erase the ancient file and create new one
            debug!("Token_path exist will erase it");
            fs::remove_file(token_path)?;

            // Put the token data in a string of yaml
            debug!("Put Token in a string");
            let token_file = serde_yaml::to_string(&self)?;

            // Creating the file for the token
            debug!("Creating the token file");
            let mut file = File::create(token_path)?;

            // Write the token data in the file
            debug!("Write the string in the file");
            file.write_all(&token_file.as_bytes())?;

            // Sync data to be sure everything is writing on drive
            debug!("Syncing data on drive");
            file.sync_all()?;

            // Put file permission to 600 to restreint access
            debug!("Set file permission to 600");
            let mut perms = file.metadata()?.permissions();
            perms.set_mode(0o600);
            file.set_permissions(perms)?;
            debug!("File permission has been set");
        }
        Ok(())
    }
    // Verify that the token is valid to decide if we must reuse the session
    pub fn verify_token(&self, tty_name: &str, tty_uuid: String) -> Result<(), Box<dyn Error>> {
        let clock = SystemTime::now();
        if self.final_timestamp <= clock {
            info!("Session has expired");
            Err(From::from("Session has expired"))
        } else if self.tty_name == tty_name
            && self.tty_uuid == tty_uuid
            && self.final_timestamp > clock
        {
            info!("Session is valid, will reuse it");
            Ok(())
        } else {
            info!("Not the same session");
            Err(From::from("Not the same session"))
        }
    }
}

// Create the directory containing the file to facilitate further use
pub fn create_dir_run(username: &str) -> Result<(), Box<dyn Error>> {
    // Create the first part of the path
    let run_path = Path::new(SESSION_DIR);

    // Verify that the first part of the path exist first
    debug!("Verify that {:?} exist", run_path);
    if !run_path.exists() {
        info!("{:?} doesn't exist, creating it", run_path);
        // Create the path with permissions of 600 to restreint access
        DirBuilder::new()
            .mode(0o600)
            .recursive(true)
            .create(SESSION_DIR)?;
    }
    // Extract permissions from the directory for further use
    let metadata = fs::metadata(SESSION_DIR)?;
    let mut perms = metadata.permissions();

    // Verify the permissions of the directory and act accordingly
    debug!("Verifying permission on {:?}", run_path);
    if perms.mode() != 0o600 {
        info!("Permissions are incorect and will be adjust");
        perms.set_mode(0o600);
        fs::set_permissions(SESSION_DIR, perms)?;
    }

    // Create the second part of the path for further use
    let user_path = format!("{}{}/", SESSION_DIR, username);
    let user_path = Path::new(&user_path);

    // Verifying if the path exist and act accordingly
    debug!("Verifying that user_path exist: {:?}", user_path);
    if !user_path.exists() {
        info!("{:?} doesn't exist, creating it", user_path);
        // Create the path with permissions of 600 to restreint access
        DirBuilder::new()
            .mode(0o600)
            .recursive(true)
            .create(user_path)?;
    } else if user_path.is_file() {
        error!("Error: {:?} is not a directory", user_path);
        let err = format!("Error: {:?} is not a directory", user_path);
        return Err(From::from(err));
    }
    // Extract permissions from the directory for further use
    let metadata = fs::metadata(user_path)?;
    let mut perms = metadata.permissions();
    // Verifying if the permission of the directory and act accordingly
    debug!("Verifying user_path permmisions");
    if perms.mode() != 0o600 {
        debug!("Permissions are incorect, adjusting it");
        perms.set_mode(0o600);
        fs::set_permissions(user_path, perms)?;
    }

    Ok(())
}
// Extract the token from the file with serde_yaml
pub fn read_token_file(token_path: &str) -> Result<Token, Box<dyn Error>> {
    // Open the file and extract it's contents in a buffer
    debug!(
        "Open the file at {} and put it's content in a buffer",
        token_path
    );
    let mut file = File::open(token_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Transform the buffer to the token struct
    debug!("Transform the buffer to the token struct");
    let token: Token = serde_yaml::from_str(&buffer)?;
    Ok(token)
}
