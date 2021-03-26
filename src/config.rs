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
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

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
        debug!("Creating default data for config file");
        let config_file = serde_yaml::to_string(&self)?;
        debug!("Creating config file at {}", CONFIG_PATH);
        let mut file = File::create(config_path)?;
        debug!("Writing to file");
        file.write_all(&config_file.as_bytes())?;
        debug!("Syncing data to drive");
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
        debug!("Opening config file at {}", CONFIG_PATH);
        let mut file = File::open(config_path)?;
        let mut buffer = String::new();
        debug!("Putting data in a string for futher use");
        file.read_to_string(&mut buffer)?;
        debug!("Transform data to a struct with serde");
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
    debug!("Begin initializing default configuration");
    let mut conf = Config::default();
    debug!("Finish initializing default configuration");

    // Verify that the file is there or write to it with the defaults
    let path = Path::new(CONFIG_PATH);
    debug!("Verifying that {} exist", CONFIG_PATH);
    if path.exists() && path.is_file() {
        debug!("Loading {}", CONFIG_PATH);
        let result = conf.read_config_file();
        if let Err(err) = result {
            eprintln!("{}", err);
            error!("{}", err);
            debug!("Removing file");
            fs::remove_file(path)?;
            let config = Config::default();
            debug!("Creating new file with defaults");
            config.create_config_file()?;
            return Ok(config);
        } else {
            debug!("Returning the content of the file");
            conf = result.unwrap();
        }
        debug!("Finish loading");
    } else if path.exists() && path.is_dir() {
        let err = format!("Error: {} is a directory", CONFIG_PATH);
        error!("{}", err);
        return Err(From::from(err));
    } else if !path.exists() {
        debug!("{} doesnt exist! Creating it", CONFIG_PATH);
        eprintln!("{} doesnt exist! Creating it", CONFIG_PATH);
        conf.create_config_file()?;
        debug!("Creation has finish");
    }
    Ok(conf)
}
