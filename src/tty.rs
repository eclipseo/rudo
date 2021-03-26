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

use libc::{isatty, ttyname};
use std::env;
use std::error::Error;
use std::ffi::CStr;

/// Safe wrapper to get the name of the current tty
/// and return it as a Rust string
pub fn get_tty_name() -> Result<String, Box<dyn Error>> {
    unsafe {
        if isatty(0) == 0 {
            error!("Rudo must be called from a TTY!");
            return Err(From::from("Rudo must be called from a TTY!"));
        }
        let ttyname_c = ttyname(0);
        // Verify that call didn't fail
        if ttyname_c.is_null() {
            error!("ttyname() call failed!");
            return Err(From::from("ttyname() call failed!"));
        }
        let ttyname_rust = CStr::from_ptr(ttyname_c).to_string_lossy().into_owned();
        Ok(ttyname_rust)
    }
}

// WINDOWID is the least trust beacause of is small size and don't change for different tabs
// It only change the last five number most of the time
// But it is used by st, xterm, sakura, kitty, xfce terminal, mate terminal and terminology
// Rox terminal use a value that change only the last six number but change for tabs
// Qterminal is unsecure as it put 0 in WINDOWID
// Guake, lxterminal, elementary terminal and deepin terminal as no uuid to use
pub fn tty_uuid() -> Result<String, Box<dyn Error>> {
    if env::var("GNOME_TERMINAL_SCREEN").is_ok() {
        let uuid = env::var("GNOME_TERMINAL_SCREEN")?;
        Ok(uuid)
    } else if env::var("SHELL_SESSION_ID").is_ok() {
        let uuid = env::var("SHELL_SESSION_ID")?;
        Ok(uuid)
    } else if env::var("TERMINATOR_UUID").is_ok() {
        let uuid = env::var("TERMINATOR_UUID")?;
        Ok(uuid)
    } else if env::var("TILIX_ID").is_ok() {
        let uuid = env::var("TILIX_ID")?;
        Ok(uuid)
    } else if env::var("ROXTERM_ID").is_ok() {
        let uuid = env::var("ROXTERM_ID")?;
        Ok(uuid)
    } else if env::var("WINDOWID").is_ok() {
        let uuid = env::var("WINDOWID")?;
        if uuid.parse::<u32>().unwrap() == 0 {
            error!("Error: terminal has a uuid of zero");
            return Err(From::from("Error: terminal has a uuid of zero"));
        }
        Ok(uuid)
    } else {
        error!("Couldn't determine the terminal uuid");
        Err(From::from("Couldn't determine the terminal uuid"))
    }
}
