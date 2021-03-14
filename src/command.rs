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
    pub fn new(mut value: Vec<&'a str>) -> Self {
        let mut program = String::new();
        program.push_str(value[0]);
        value.remove(0);
        let args = value.clone();
        Self {
            value,
            program,
            args,
        }
    }
}
