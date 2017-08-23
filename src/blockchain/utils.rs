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

extern crate crypto;
use self::crypto::sha3::Sha3;
use self::crypto::digest::Digest;

pub fn u64_to_u8le(input: u64) -> [u8; 8]{

    let mut output = [0;8];
    for i in 0..8{
        output[i] = (input >> i*8) as u8
    }
    output

}

pub fn u8le_to_u64(input: [u8; 8]) -> u64{

    let mut output: u64 = 0;
    let mut i = 0;

    while i < 8{
        output += (input[i] as u64) << (8*i);
        i += 1;
    }
    output

}

pub fn sha3_256(input: &[u8]) -> [u8; 32]{

    let mut output = [0;32];
    let mut hasher = Sha3::sha3_256();
    hasher.input(&input);
    hasher.result(& mut output);

    output

}
