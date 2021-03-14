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

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
    ArgMatches,
}; // CLI support

//Initialize the CLI and take the argument supply by the user
pub fn init_cli() -> ArgMatches<'static> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .value_name("command")
                .help("Pass the command to execute")
                .conflicts_with("shell")
                .required_unless("shell")
                .index(1) //Be sure that the command is the first so we dont have to write "-c" to take a command
                .multiple(true) //To be able to have the command and it's list of argument
                .takes_value(true),
        )
        //Take the name of the group the user must be in to have privilege access
        .arg(
            Arg::with_name("group")
                .short("g")
                .long("group")
                .value_name("group")
                .help("Sets the group the user must be in to have privilege access")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .value_name("shell")
                .help("Initialize a privilege shell")
                .conflicts_with("command")
                .required_unless("command")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .value_name("user")
                .help("The user you want to be in your command")
                .takes_value(true),
        )
        .get_matches();
    matches
}
