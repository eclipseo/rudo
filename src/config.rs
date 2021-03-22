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

use clap::ArgMatches;
use configparser::ini::Ini;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct Config {
    pub user: String,
    pub group: String,
    pub password: bool,
    pub shell: String,
    pub userlist: String,
    pub greeting: bool,
}

impl Config {
    // Create the new configuration
    fn new(
        user: String,
        group: String,
        password: bool,
        shell: String,
        userlist: String,
        greeting: bool,
    ) -> Self {
        Self {
            user,
            group,
            password,
            shell,
            userlist,
            greeting,
        }
    }
    pub fn update(mut self, matches: &ArgMatches) -> Self {
        if matches.value_of("user").is_some() {
            debug!("User value will be update");
            self.user = matches.value_of("user").unwrap().to_string();
        }
        if matches.is_present("greeting") {
            debug!("greeting value will be update");
            self.greeting = true;
        }
        self
    }
}
// Default value for configuration
impl Default for Config {
    fn default() -> Self {
        Self {
            user: String::from("root"),
            group: String::from("wheel"),
            password: true,
            shell: String::from("/bin/sh"),
            userlist: String::from("root"),
            greeting: true,
        }
    }
}
// Initialize the configuration with rudo.conf or defaults
pub fn init_conf(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    // Initialize configuration
    debug!("Begin initializing configuration");
    let mut config = Ini::new();
    debug!("Finish initializing configuration");

    // Verify that the file is there or write to it with the defaults
    let path = Path::new(path);
    debug!("Verifying that /etc/rudo.conf exist");
    if path.exists() && path.is_file() {
        debug!("Loading /etc/rudo.conf");
        config.load(path.to_str().unwrap())?;
        debug!("Finish loading");
    } else if path.exists() && path.is_dir() {
        error!("Error: /etc/rudo.conf is a directory");
        return Err(From::from("Error: /etc/Rudo.conf is a directory"));
    } else if !path.exists() {
        debug!("/etc/rudo.conf doesnt exist! Creating it");
        eprintln!("/etc/rudo.conf doesnt exist! Creating it");
        config.read(String::from(
            "[user]
# The user you want to impersonate
user = root

[access]
# The group the user must be a part of to have privilege access
group = wheel
# Do we demand for the user password. Risky to set to false, can't guaranty security.
password = true
# The list of users authorized to gain privileged access
userlist = root
[miscellaneous]
# User greeting
greeting = true",
        ))?;
        config.write(path.to_str().unwrap())?;
        debug!("Creation finish");
    }

    // Extract the various element to create the config
    debug!("Extract the config from rudo.conf");
    let shell = env::var("SHELL").unwrap_or_else(|_| Config::default().shell);
    let user = config
        .get("user", "user")
        .unwrap_or_else(|| Config::default().user);
    let group = config
        .get("access", "group")
        .unwrap_or_else(|| Config::default().group);
    let password = config
        .getbool("access", "password")?
        .unwrap_or_else(|| Config::default().password);
    let userlist = config
        .get("access", "userlist")
        .unwrap_or_else(|| Config::default().userlist);
    let greeting = config
        .getbool("miscellaneous", "greeting")?
        .unwrap_or_else(|| Config::default().greeting);
    let conf = Config::new(user, group, password, shell, userlist, greeting);
    debug!("Config has been extract");
    Ok(conf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() -> Result<(), Box<dyn Error>> {
        let conf = Config::new(
            String::from("test"),
            String::from("test"),
            false,
            String::from("test"),
            String::from("test"),
            false,
        );
        if conf.user == "test"
            && conf.group == "test"
            && !conf.password
            && conf.shell == "test"
            && conf.userlist == "test"
            && !conf.greeting
        {
            Ok(())
        } else {
            Err(From::from("Test failed to create Config struct"))
        }
    }
}
