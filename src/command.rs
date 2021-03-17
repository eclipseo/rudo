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

pub struct Comm<'a> {
    value: Vec<&'a str>,
    pub program: String,
    pub args: Vec<&'a str>,
}

impl<'a> Comm<'a> {
    // Create the new Comm with the extract value supply by the user
    pub fn new(mut value: Vec<&'a str>) -> Result<Self, &str> {
        debug!("Verifying that value is not empty");
        // Verify that it's not empty
        if !value.is_empty() {
            debug!("Value is not empty, proceeding");
            let mut program = String::new();
            // Extract the first word then remove it
            program.push_str(value[0]);
            value.remove(0);
            // Clone the rest of the value
            let args = value.clone();
            debug!("return Comm");
            Ok(Self {
                value,
                program,
                args,
            })
        } else {
            debug!("Value is empty");
            Err("Value is empty")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comm_new() -> Result<(), &'static str> {
        let comm = Comm::new(vec!["test"]);
        if comm.is_ok() {
            Ok(())
        } else {
            Err("Test failed")
        }
    }
    #[test]
    fn test_comm_new_empty() -> Result<(), &'static str> {
        let comm = Comm::new(vec![]);
        if comm.is_err() {
            Ok(())
        } else {
            Err("Test failed")
        }
    }
}
