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

use blockchain::traits::Hashable;
use blockchain::traits::BlockStorage;
use blockchain::block::Block;
use blockchain::block::BlockId;
use blockchain::block::BlockErrorReason;
use blockchain::transactions::TxId;
use blockchain::transactions::TxIndex;
use blockchain::transactions::Transaction;

/// Tests if a storage appends and fetches
/// `Block`s correctly
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `BlockStorage` trait

pub fn test_fetch_block<T>(storage: &mut T) where T: BlockStorage{

    let block = Block::new([0; 32], None, 0, vec![]);
    let block_id = block.get_id();

    let result = storage.append_verified_block(block);
    assert!(result.is_ok(), "Could not append block");

    let fetched_block = storage.get_block(block_id);
    assert!(fetched_block.is_some(),
            "Appended block could not be retrieved");

    if let Some(block) = fetched_block {
        let fetched_id = block.get_id();
        assert_eq!(fetched_id, block_id,
                   "Fetched block has not the required id");
    }

    let non_existent_id = BlockId([0;32]);
    let fetched_block = storage.get_block(non_existent_id);
    assert!(fetched_block.is_none(),
            "Non-existent block id returned a result");

}

/// Tests if a storage detects block id collisions
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `BlockStorage` trait

pub fn test_block_id_collision<T>(storage: &mut T) where T: BlockStorage{

    let block = Block::new([0; 32], None, 0, vec![]);
    let block_id = block.get_id();

    let cloned = block.clone();
    let result = storage.append_verified_block(cloned);
    assert!(result.is_ok(), "Could not append block");

    let result = storage.append_verified_block(block);
    assert!(result.is_err(), "Block id collision was not detected");

    if let Err(err) = result{
        match err.reason{
            BlockErrorReason::IdCollision(err_block_id) => {
                assert_eq!(err_block_id, block_id,
                           "BlockErrorReason::IdCollision wrapped the \
                            wrong block_id");
            },
            _ => {
                assert!(false, "Storage returned a wrong error \
                                when trying to provoke an id collision");
            }
        }
    }

}

/// Tests if a storage fetches dummy transactions
/// correctly
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `BlockStorage` trait

pub fn test_fetch_dummy_transaction<T>(storage: &mut T) where T: BlockStorage{

    let block = Block::new([0; 32], None, 0, vec![Transaction::Dummy]);
    let block_id = block.get_id();

    let result = storage.append_verified_block(block);
    assert!(result.is_ok(), "Could not append block");

    let dummy_id = TxId::new(block_id, TxIndex(0));
    let fetched_transaction = storage.get_transaction(dummy_id);
    assert!(fetched_transaction.is_some(),
            "Existent dummy transaction could not be fetched");

    if let Some(transaction) = fetched_transaction{

        // dummy transactions have a
        // hash value of all zeroes

        assert_eq!(transaction.to_sha3_hash(), [0; 32],
                   "Fetched dummy transaction, but
                    hashes don't match");

    }

    let non_existent_id = TxId::new(block_id, TxIndex(1));
    let fetched_transaction = storage.get_transaction(non_existent_id);
    assert!(fetched_transaction.is_none(),
            "Non-existent transaction id returned result");

}
