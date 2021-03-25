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
use std::sync::Arc;
use users::{Group, Users, UsersCache};

// Stock the User and is value for later use
pub struct User {
    pub user: Arc<users::User>,
    //uid: u32,
    pub username: String,
    group: Vec<Group>,
}

impl User {
    // Create the user and it's data for later use
    pub fn new() -> User {
        debug!("Begin user creation");
        let userscache = UsersCache::new();
        let uid = userscache.get_current_uid();
        let user = userscache.get_user_by_uid(uid).unwrap();
        let username = user.name().to_str().unwrap().to_string();
        let group = user.groups().unwrap();
        debug!("User create");
        User {
            user,
            //uid,
            username,
            group,
        }
    }
    // Verify that the user is part of the list of authorized users
    pub fn verify_user(&self, userlist: &Vec<String>) -> Result<(), Box<dyn Error>> {
        debug!("Begin to verify user");
        let username = self.user.name().to_str().unwrap();
        let mut count = 0;
        for usr in userlist {
            if usr == username {
            count += 1;
            }
        }
        if count == 1 {
            Ok(())
        } else {
            Err(From::from("User not authorized"))
        }
    }
    // Take the vector containing the Group and search for the group supply in the configuration
    pub fn verify_group(&self, arggroup: &str) -> Result<(), Box<dyn Error>> {
        debug!("Begin group verification");
        let group = &self.group;
        let mut count = 0;

        // Compare the supply group with the list of the user membership
        for gr in group {
            if gr.name() == arggroup {
                count += 1;
            }
        }

        if count == 1 {
            info!("User is a member of authorized group");
            println!("You are a member of the group {}", arggroup);
            Ok(())
        } else {
            info!("User is not a member of authorized group");
            let error = "You are not a member of group ";
            error.to_string().push_str(arggroup);
            Err(From::from(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_group() -> Result<(), Box<dyn Error>> {
        let userdata = User::new();
        if userdata.verify_group("test").is_err() {
            Ok(())
        } else {
            Err(From::from("The group should not correspond with test"))
        }
    }
    #[test]
    fn test_verify_user() -> Result<(), Box<dyn Error>> {
        let userdata = User::new();
        if userdata.verify_user("test").is_err() {
            Ok(())
        } else {
            Err(From::from("The user should not correspond with test"))
        }
    }
}
