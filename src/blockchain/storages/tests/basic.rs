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

use blockchain::traits::ChainStorage;
use blockchain::storages::tests::blockstorage;
use blockchain::storages::tests::chainstorage;

/// Checks basic functionality of a struct implementing
/// the `ChainStorage` trait
///
/// # Arguments
/// * `storage`: A struct implementing `ChainStorage`

pub fn test_chain_storage<T>(mut storage: &mut T) where T: ChainStorage{

    storage.reset();

    blockstorage::test_fetch_block::<T>(&mut storage);
    storage.reset();
    blockstorage::test_block_id_collision::<T>(&mut storage);
    storage.reset();
    blockstorage::test_append_orphaned::<T>(&mut storage);
    storage.reset();
    blockstorage::test_fetch_dummy_transaction::<T>(&mut storage);
    storage.reset();

    chainstorage::blocks::test_get_after::<T>(&mut storage);
    storage.reset();
    chainstorage::blocks::test_get_after_timestamp::<T>(&mut storage);
    storage.reset();

    chainstorage::txstates::test_txstate_nonexistent::<T>(&mut storage);
    storage.reset();
    chainstorage::txstates::test_txstate_set_fail::<T>(&mut storage);
    storage.reset();
    chainstorage::txstates::test_txstate_claimable::<T>(&mut storage);
    storage.reset();
    chainstorage::txstates::test_txstate_unclaimable::<T>(&mut storage);
    storage.reset();
    chainstorage::txstates::test_txstate_finalized::<T>(&mut storage);
    storage.reset();
    chainstorage::txstates::test_txstate_one_to_one_rel::<T>(&mut storage);
    storage.reset();
    chainstorage::txstates::test_txstate_one_to_many_rel::<T>(&mut storage);
    storage.reset();

}
