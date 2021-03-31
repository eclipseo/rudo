# NAME
Rudo is a utility to gain privilege access on unix system with pam.

# SYNOPSIS
USAGE: \
    rudo [FLAGS] [OPTIONS] command... --edit edit --shell

FLAGS: \
    -d, --debug       Log debug messages \
    -g, --greeting    Greeting user \
    -h, --help        Prints help information \
    -s, --shell       Initialize a privilege shell \
    -V, --version     Prints version information

OPTIONS: \
    -e, --edit edit    Edit a document with the editor of user \
    -u, --user user    The user you want to impersonate

ARGS: \
    command...    Pass the command to execute


# DESCRIPTION
**Rudo** "RustUser do" allows a system administrator to give certain
users the ability to run some commands as **root** or another user while 
logging all commands and it's arguments.

# EXAMPLES
* rudo command argument
* rudo --shell

# FILES
* /etc/rudo.conf
* /etc/pam.d/rudo

# CONFIGURATION
---
* rudo:
  * impuser: unix name of the user you want to impersonate
* user:
  - username: your unix user name \
    group: the name of the group you must be a member to have privilege access \
    password: true or false, if you want to give your password each session or not \
    greeting: true or false if you want the hello message each time you run rudo \
  - username: your unix user name \
    group: the name of the group you must be a member to have privilege access \
    password: true or false, if you want to give your password each session or not \
    greeting: true or false if you want the hello message each time you run rudo \

# AUTHOR
Rémi Lauzier <remilauzier@protonmail.com>

# COPYRIGHT
    Rudo is a program to get privilege access on unix system
    Copyright (C) 2021  Rémi Lauzier <remilauzier@protonmail.com>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
