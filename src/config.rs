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
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::error::Error;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::io::{Read, Write};

static CONFIG_PATH: &str = "/etc/rudo.conf";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub user: String,
    pub group: String,
    pub password: bool,
    pub userlist: Vec<String>,
    pub greeting: bool,
}

impl Config {
    fn create_config_file(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Path::new(CONFIG_PATH);
        let config_file = serde_yaml::to_string(&self)?;
        let mut file = File::create(config_path)?;
        file.write_all(&config_file.as_bytes())?;
        file.sync_all()?;
        debug!("Set file permission");
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms)?;
        debug!("File permission has been set");

        Ok(())
    }
    fn read_config_file(&self) -> Result<Self, Box<dyn Error>> {
        let config_path = Path::new(CONFIG_PATH);
        let mut file = File::open(config_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let config: Config = serde_yaml::from_str(&buffer)?;
        Ok(config)
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
            userlist: vec![String::from("root")],
            greeting: true,
        }
    }
}
// Initialize the configuration with rudo.conf or defaults
pub fn init_conf() -> Result<Config, Box<dyn Error>> {
    // Initialize configuration
    debug!("Begin initializing configuration");
    let mut conf = Config::default();
    debug!("Finish initializing configuration");

    // Verify that the file is there or write to it with the defaults
    let path = Path::new(CONFIG_PATH);
    debug!("Verifying that {} exist", CONFIG_PATH);
    if path.exists() && path.is_file() {
        debug!("Loading /etc/rudo.conf");
        let result = conf.read_config_file();
        if let Err(err) = result {
            fs::remove_file(path)?;
            let config = Config::default();
            config.create_config_file()?;
            return Ok(config);
        } else {
            conf = result.unwrap();
        }
        debug!("Finish loading");
    } else if path.exists() && path.is_dir() {
        error!("Error: /etc/rudo.conf is a directory");
        return Err(From::from("Error: /etc/Rudo.conf is a directory"));
    } else if !path.exists() {
        debug!("/etc/rudo.conf doesnt exist! Creating it");
        eprintln!("/etc/rudo.conf doesnt exist! Creating it");
        conf.create_config_file()?;
        debug!("Creation finish");
    }
    Ok(conf)
}
