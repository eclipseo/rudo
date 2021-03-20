![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/remilauzier/rudo?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/rudo?style=flat-square)
![Crates.io](https://img.shields.io/crates/d/rudo?style=flat-square)
[![Rust](https://github.com/remilauzier/rudo/actions/workflows/rust.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/rust.yml)
[![GitHub issues](https://img.shields.io/github/issues/remilauzier/rudo?style=flat-square)](https://github.com/remilauzier/rudo/issues)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/remilauzier/rudo?style=flat-square)
![Lines of code](https://img.shields.io/tokei/lines/github/remilauzier/rudo?style=flat-square)
![GitHub top language](https://img.shields.io/github/languages/top/remilauzier/rudo?style=flat-square)
[![GitHub license](https://img.shields.io/github/license/remilauzier/rudo?style=flat-square)](https://github.com/remilauzier/rudo/blob/main/LICENSE)
# Rudo
Rudo "RustUser do" allows a system administrator to give certain
users (or groups of users) the ability to run some (or all) commands
as root while logging all commands and arguments. Rudo operates on a
per-command basis.  It is not a replacement for the shell.

# Problem
You need to change the ower of the binary to root for now
sudo chown root:root
sudo chmod 4755

# License
GPLv2 or later

# Warning
No guaranty of security for now
