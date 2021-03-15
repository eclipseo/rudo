# Rudo
Rudo (rustuser do) allows a system administrator to give certain
users (or groups of users) the ability to run some (or all) commands
as root while logging all commands and arguments. Sudo operates on a
per-command basis.  It is not a replacement for the shell.

# Problem
You need to change the ower of the binary to root for now
sudo chown root:root
sudo chmod 4755

# License
GPLv2 or later

# Warning
No guaranty of security for now
