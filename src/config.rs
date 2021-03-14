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
use configparser::ini::Ini; // INI configuration support
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub user: String,
    pub group: String,
    pub password: bool,
    pub shell: String,
}

impl Config {
    fn new(user: String, group: String, password: bool, shell: String) -> Self {
        Self {
            user,
            group,
            password,
            shell,
        }
    }
    pub fn update(mut self, matches: &ArgMatches) -> Self {
        if matches.value_of("user").is_some() {
            self.user = matches.value_of("user").unwrap().to_string();
        }
        if matches.value_of("group").is_some() {
            self.group = matches.value_of("group").unwrap().to_string();
        }
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            user: String::from("root"),
            group: String::from("wheel"),
            password: true,
            shell: String::from("/bin/sh"),
        }
    }
}

pub fn init_conf(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    let mut config = Ini::new();

    if fs::read(&path).is_ok() {
        config.load(path.as_path().to_str().unwrap())?;
    } else {
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
userlist = root",
        ))?;
        config.write(path.as_path().to_str().unwrap())?;
    }

    let shell = env::var("SHELL")?;

    let user = config
        .get("user", "user")
        .unwrap_or_else(|| Config::default().user);
    let group = config
        .get("access", "group")
        .unwrap_or_else(|| Config::default().group);
    let password = config
        .getbool("access", "password")?
        .unwrap_or_else(|| Config::default().password);
    let conf = Config::new(user, group, password, shell);
    Ok(conf)
}
