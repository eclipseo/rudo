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

// Put the data of the actual user in a struct for later use
pub struct User {
    pub user: Arc<users::User>,
    pub username: String,
    group: Vec<Group>,
}

impl User {
    pub fn new() -> User {
        // Create the user and it's data for later use
        debug!("Begin user creation");
        let userscache = UsersCache::new();
        let uid = userscache.get_current_uid();
        let user = userscache.get_user_by_uid(uid).unwrap();
        let username = user.name().to_str().unwrap().to_string();
        let group = user.groups().unwrap();
        debug!("User has been create");
        User {
            user,
            username,
            group,
        }
    }
    // Verify that the user is part of the list of authorized users
    pub fn verify_user(&self, userlist: &[String]) -> Result<(), Box<dyn Error>> {
        debug!("Begin to verify if user is authorized");
        let username = self.user.name().to_str().unwrap();
        let mut count = 0;
        for usr in userlist {
            if usr == username {
                count += 1;
            }
        }
        if count == 1 {
            Ok(())
        } else if count >= 2 {
            error!("User is present multiple time in conf file");
            Err(From::from("User is present multiple time in conf file"))
        } else {
            error!("User is not authorized");
            Err(From::from("User is not authorized"))
        }
    }
    // Take the vector containing the Group and search for the group supply in the configuration
    pub fn verify_group(&self, arggroup: &str) -> Result<(), Box<dyn Error>> {
        debug!("Beginning group verification");
        let group = &self.group;
        let mut count = 0;

        // Compare the supply group with the list of the user membership
        for gr in group {
            if gr.name() == arggroup {
                count += 1;
            }
        }
        if count == 1 {
            info!("User is a member of the authorized group: {}", arggroup);
            Ok(())
        } else if count >= 2 {
            error!("{} is list multiple time", arggroup);
            let err = format!("{} is list multiple time", arggroup);
            Err(From::from(err))
        } else {
            error!("User is not a member of the authorized group: {}", arggroup);
            let error = format!("User is not a member of the authorized group: {}", arggroup);
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
        if userdata.verify_user(&[String::from("test")]).is_err() {
            Ok(())
        } else {
            Err(From::from("The user should not correspond with test"))
        }
    }
}
