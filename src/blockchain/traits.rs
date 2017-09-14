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
use blockchain::errors::IdCollisionError;
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

    /// Append a block to the storage *without* checking its
    /// validity. Will return an IdCollisionError if there
    /// already is a block with the same block id.
    /// * `block`: The verified block

    fn append_verified_block(&mut self, block: Block)
            -> Result<(), IdCollisionError>;

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

/// The `TreeStorage` trait must be implemented by all storage
/// backends saving the fuzzy tail of the blockchain.

pub trait TreeStorage: BlockStorage {

    /// Fetches the `TxState` of a transaction
    /// at a distinct position in the tree.
    ///
    /// * `branch_tail_id`: A block id identifying
    ///         a branch in the tree
    /// * `tx_id`: the transaction id

    //         .---.      .---.    If you look at the tree on the left
    //    .--o | B | o--o | E |    you will notice, that the state of
    //    o    '---'      '---'    transactions in A is ambigous. It
    //  .---.      .---.           depends on wether we look at the
    //  | A | o--o | C |           branch A-B-E, A-C or A-D.
    //  '---'      '---'           Therefore we need to define for
    //    o    .---.               which branch we set the state.
    //    '--o | D |               The branch is identified by its
    //         '---'               tail, meaning that by setting
    //                             branch_tail_id to E, we get the
    //                             branch A-B-E, by setting it to C,
    //                             the branch A-C, etc.


    fn get_transaction_state(&self,
                             branch_tail_id: BlockId,
                             tx_id: TxId) -> Option<TxState>;

    /// Updates the `TxState` of a transaction
    /// at a distinct position in the tree.
    ///
    /// Returns a TxProgError when tx_id points to
    /// a non-existent transaction or if one of
    /// the relationships has invalid data.
    ///
    /// * `branch_tail_id`: A block id identifying
    ///         a branch in the tree
    /// * `tx_id`: the transaction id
    /// * `tx_state`: the appropriate transaction state

    fn set_transaction_state(&mut self,
                             branch_tail_id: BlockId,
                             tx_id: TxId,
                             tx_state: TxState) -> Result<(), TxProgError>;

}
