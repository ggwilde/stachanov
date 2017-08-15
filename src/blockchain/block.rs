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

use blockchain::header::BlockHeader;
use blockchain::body::BlockBody;
use blockchain::transactions::Transaction;
use blockchain::errors::VerificationError;
use blockchain::errors::VerificationErrorReason::InvalidContentHash;

/// BlockId is equivalent to the sha3 hash of the block header

pub struct BlockId([u8; 32]);

pub struct Block{
    header: BlockHeader,
    body: BlockBody<Transaction>
}

impl Block{

    /// Verifies the internal consistency of the block.
    /// This includes:
    /// * Verification of issuer signature
    /// * Verification of proof of work
    /// * Verification of merkle hash tree

    fn verify_internal(&self) -> Result<(), VerificationError> {

        self.header.verify_internal()?;

        if self.header.content_hash != self.body.merkle_root_hash(){
            let err = VerificationError::new(InvalidContentHash);
            return Err(err)
        }
        Ok(())

    }

}
