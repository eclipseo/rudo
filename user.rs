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

use std::sync::Arc;
use users::{Group, Users, UsersCache}; // safe library to extract data for user and group

// Stock the User and is value for later use
pub struct User {
    pub user: Arc<users::User>,
    //uid: u32,
    pub username: String,
    group: Vec<Group>,
}

// Create the User and its data for later use
impl User {
    pub fn new(usersdata: &UsersCache) -> User {
        let uid = usersdata.get_current_uid();
        let user = usersdata.get_user_by_uid(uid).unwrap();
        let username = user.name().to_str().unwrap().to_string();
        let group = user.groups().unwrap();
        User {
            user,
            //uid,
            username,
            group,
        }
    }
    // Take the vector containing the Group and search for the group supply in the configuration
    pub fn verify_group(&self, arggroup: &str) -> Result<(), &'static str> {
        let group = &self.group;
        let mut count = 0;

        //Compare the group with the vec containing the list of group
        for gr in group {
            if gr.name() == arggroup {
                count += 1;
            }
        }

        if count == 1 {
            println!("You are a member of the group {}", arggroup);
            Ok(())
        } else {
            let error = "You are not a member of group ";
            error.to_string().push_str(arggroup);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use users::Groups;

    #[test]
    fn test_verify_group() -> Result<(), &'static str> {
        let userscache = UsersCache::new();
        let userdata = User::new(&userscache);
        let gid = userdata.user.primary_group_id();
        let group = userscache.get_group_by_gid(gid).unwrap();
        let groupname = group.name().to_str().unwrap();
        userdata.verify_group(groupname)
    }
}
