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

use blockchain::block::Block;
use blockchain::block::BlockId;
use blockchain::block::BlockError;
use blockchain::errors::BinFormatError;
use blockchain::header::BlockHeader;
use blockchain::transactions::TxId;
use blockchain::transactions::TxState;
use blockchain::transactions::TxProgError;
use blockchain::transactions::Transaction;

pub trait Hashable {
    fn to_sha3_hash(&self) -> [u8; 32];
}


/// `BlockStorage` is the parent trait for `ChainStorage`
/// and `TreeStorage`. It defines an interface for adding
/// blocks as well as for retrieving blocks and transactions

pub trait BlockStorage{

    /// Fetches a block identified by its unique block id
    /// * `block_id`: The block identifier (equivalent to the
    ///               the sha3 hash of the block header)

    fn get_block(&self, block_id: BlockId) -> Option<Block>;

    /// Fetches a block header identified by its unique id
    ///
    /// # Arguments
    /// * `block_id`: The block identifier (equivalent to the
    ///               the sha3 hash of the block header)

    fn get_header(&self, block_id: BlockId) -> Option<BlockHeader>;

    /// Append a block to the storage *without* checking its
    /// validity.
    ///
    /// Will return a BlockError with reason IdCollision when
    /// there already is a block with the same block id, a
    /// BlockError with reason OrphanedBlock if the block
    /// has no valid predecessor
    ///
    /// # Arguments
    ///
    /// * `block`: The verified block

    fn append_verified_block(&mut self, block: Block)
            -> Result<(), BlockError>;

    /// Fetches a transaction identifier by its id
    /// * `tx_id`: the transaction id

    fn get_transaction(&self, tx_id: TxId)
            -> Option<Transaction>;

    /// Removes all data from the storage

    fn reset(&mut self);

}

/// The `ChainStorage` trait must be implemented by all storage
/// backends saving the chain consensus. In contrast to `TreeStorage`,
/// all blocks in `ChainStorage` (except the last one) have a distinct
/// subsequent block, so we can implement iterator primitives.

pub trait ChainStorage: BlockStorage {

    /// Fetches the block *after* the supplied block id. This
    /// is mainly an interface for block iteration.
    /// * `block_id`: The block identifier (equivalent to the
    ///               the sha3 hash of the block header)

    fn get_after(&self, block_id: BlockId) -> Option<Block>;

    /// Fetches the block that has a timestamp greater or
    /// equal to the specified timestamp.
    /// * `timestamp`: A unix timestamp

    // NOTE: This is a difference to the bitcoin timestamp
    // implementation. In stachanov a block header must have a
    // timestamp greater than the timestamp of its predecessor

    fn get_after_timestamp(&self, timestamp: u64) -> Option<Block>;

    /// Fetches the last block in the whole chain

    fn get_tail_block(&self) -> Option<Block>;

    /// Fetches the `TxState` of a transaction
    /// * `tx_id`: the transaction id

    fn get_transaction_state(&self, tx_id: TxId) -> Option<TxState>;

    /// Updates the `TxState` of a transaction.
    ///
    /// Returns a TxProgError when tx_id points to
    /// a non-existent transaction or if one of
    /// the relationships has invalid data.
    ///
    /// * `tx_id`: the transaction id
    /// * `tx_state`: the appropriate transaction state

    fn set_transaction_state(&mut self,
                             tx_id: TxId,
                             tx_state: TxState) -> Result<(), TxProgError>;

}

/// `BinFormat` defines an interface for serializing
/// and deserializing objects to/from a raw byte format

pub trait BinFormat<T>{

    fn as_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: Vec<u8>) -> Result<T, BinFormatError>;

}
