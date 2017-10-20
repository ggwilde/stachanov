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

/// `VerificationErrorReason` is an enum used to denote the type
/// of verification error

#[derive(Debug)]
pub enum VerificationErrorReason{
    InvalidIssuerSignature,
    InvalidContentHash,
    InvalidChainLink
}

impl fmt::Display for VerificationErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerificationErrorReason::InvalidIssuerSignature => write!(f, "Block header signature doesn't match issuer"),
            VerificationErrorReason::InvalidContentHash => write!(f, "Block header content hash doesn't match transaction merkle tree root"),
            VerificationErrorReason::InvalidChainLink => write!(f, "Chain link is invalid (prev_block_hash, timestamp or index incorrect)")
        }
    }
}

/// `VerificationError`s happen when a block could not be verified

#[derive(Debug)]
pub struct VerificationError{
    reason: VerificationErrorReason
}

impl VerificationError{
    pub fn new(reason: VerificationErrorReason) -> VerificationError{
        VerificationError{reason: reason}
    }
}

impl Error for VerificationError{
    fn description(&self) -> &str{
        "Block could not be verified. Block rejected"
    }
}

impl fmt::Display for VerificationError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block rejected. Reason: {}", self.reason)
    }
}

/// `BinFormatErrorReason` is an enum used to denote
/// the type of `BinFormatError`

#[derive(Debug)]
pub enum BinFormatErrorReason{
    InvalidDataSize,
    UnsupportedVersion,
    InvalidFieldData(String)
}

impl fmt::Display for BinFormatErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinFormatErrorReason::InvalidDataSize =>
                write!(f, "Supplied data has an invalid size"),
            BinFormatErrorReason::UnsupportedVersion =>
                write!(f, "Supplied data has an invalid version."),
            BinFormatErrorReason::InvalidFieldData(ref field_name) =>
                write!(f, "Field {} contains invalid data", field_name),
        }
    }
}

/// `BinFormatError`s happen when faulty binary data
/// is deserialized. This typically happens, when
/// invalid data is retrieved over the network

#[derive(Debug)]
pub struct BinFormatError{
    reason: BinFormatErrorReason
}

impl BinFormatError{
    pub fn new(reason: BinFormatErrorReason) -> BinFormatError{
        BinFormatError{reason: reason}
    }
}

impl Error for BinFormatError{
    fn description(&self) -> &str{
        "Faulty binary data"
    }
}

impl fmt::Display for BinFormatError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not create object from binary data: \
                   Reason: {}", self.reason)
    }
}
