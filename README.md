# Rudo
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/remilauzier/rudo?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/rudo?style=flat-square)
![Crates.io](https://img.shields.io/crates/d/rudo?style=flat-square)
[![Rust](https://github.com/remilauzier/rudo/actions/workflows/rust.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/rust.yml)
[![GitHub issues](https://img.shields.io/github/issues/remilauzier/rudo?style=flat-square)](https://github.com/remilauzier/rudo/issues)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/remilauzier/rudo?style=flat-square)
![Lines of code](https://img.shields.io/tokei/lines/github/remilauzier/rudo?style=flat-square)
![GitHub top language](https://img.shields.io/github/languages/top/remilauzier/rudo?style=flat-square)
[![GitHub license](https://img.shields.io/github/license/remilauzier/rudo?style=flat-square)](https://github.com/remilauzier/rudo/blob/main/LICENSE)
# Description
Rudo "RustUser do" allows a system administrator to give certain
users (or groups of users) the ability to run some commands
as root while logging all commands and arguments.

# Functionality
You can give rudo a command to execute like `rudo some-command with-args`
you can invoke a shell with `rudo -s` or `rudo --shell`
you can change the user to impersonate with `rudo -u some-user` or `rudo --user some-user`
you can edit document with the editor specify in your environment variable with `rudo -e some-document` or `rudo --edit some-document`
you can log debug journal to systemd with `rudo -d` or `rudo --debug`
you can start the user greeting with `rudo -g` or `rudo --greeting`

# Configuration
The config file is in `yaml` and must be at `/etc/rudo.conf` or it will be create
invalid file will be remove and replaced with default
you can change the user to impersonate
you can change the group the user must be member to have authorization
you can remove the password obligation **at your own risk**
you can remove the greeting of the user
you can decide which user is authorized to use rudo

# Problem
You need to change the owner of the binary to root for now to make it work
sudo chown root:root
sudo chmod 4755

# License
GPLv2 or later

# Warning
No guaranty of security for now
