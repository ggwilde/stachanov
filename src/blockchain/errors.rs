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

/// IdCollisionErrors happen when a new block is added, that
/// has the same sha3 hash as an already registered block

#[derive(Debug)]
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

/// VerificationErrorReason is an enum used to denote the type
/// of verification error

#[derive(Debug)]
pub enum VerificationErrorReason{
    InvalidProofOfWork,
    InvalidIssuerSignature,
    InvalidContentHash
}

impl fmt::Display for VerificationErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerificationErrorReason::InvalidProofOfWork => write!(f, "Proof of work was not valid"),
            VerificationErrorReason::InvalidIssuerSignature => write!(f, "Block header signature doesn't match issuer"),
            VerificationErrorReason::InvalidContentHash => write!(f, "Block header content hash doesn't match transaction merkle tree root")
        }
    }
}

/// VerificationErrors happen when a block could not be verified

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
