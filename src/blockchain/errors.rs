//
//    Stachanov
//    Copyright (C) 2017 Stachanov Developer Collective
//
//    This file is part of Stachanov.
//
//    This program is free software: you can redistribute it and/or
//    modify it under the terms of the GNU Affero General Public
//    License, version 3, as published by the Free Software Foundation.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU Affero General Public License for more details.
//
//    You should have received a copy of the
//               GNU Affero General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

use std::error::Error;
use std::fmt;

#[derive(Debug)]

/// IdCollisionErrors happen when a new block is added, that
/// has the same sha3 hash as an already registered block

pub struct IdCollisionError;

impl Error for IdCollisionError{
    fn description(&self) -> &str{
        "BlockId collision found. Block rejected."
    }
}

impl fmt::Display for IdCollisionError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlockId collision found. Block rejected.")
    }
}
