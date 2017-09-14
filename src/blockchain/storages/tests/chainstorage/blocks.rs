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
use blockchain::block::Block;

/// Tests if a storage implements `get_after` correctly
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_get_after<T>(storage: &mut T) where T: ChainStorage{

    // append a chain of different blocks (all unverified)

    let mut blocks = vec![];
    let chain_len = 10;

    for _i in 0..chain_len {

        let block;

        if let Some(previous_block) = blocks.last(){
            block = Block::new([0; 32], Some(&previous_block), 0, vec![]);
        }else{
            block = Block::new([0; 32], None, 0, vec![]);
        }

        let cloned = block.clone();
        let result = storage.append_verified_block(cloned);
        assert!(result.is_ok(), "Could not append block");

        blocks.push(block);

    }

    // check if they get returned in the same
    // order by get_after

    for i in 0..chain_len{

        let current_block_id = blocks[i].get_id();
        let next_block = blocks.get(i+1);

        let fetched_block = storage.get_after(current_block_id);

        if let Some(next_block) = next_block{

            assert!(fetched_block.is_some(),
                    "non-tail block doesn't have a \
                     successor according to get_after");

            let assumed_id = next_block.get_id();
            let fetched_id = fetched_block.unwrap().get_id();
            assert_eq!(fetched_id, assumed_id,
                       "get_after doesn't fetch blocks \
                        in the order they were added");

        }else{

            assert!(fetched_block.is_none(),
                    "tail block has a successor \
                     according to get_after");

        }

    }

}

/// Tests if a storage implements `get_after_timestamp`
/// correctly
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_get_after_timestamp<T>(storage: &mut T) where T: ChainStorage{

    let mut blocks = vec![];
    let chain_len = 10;

    // we create a list of blocks, that all have a time difference
    // of 2 seconds, starting at 1, so we have an order of timestamps 
    // like 1, 3, 5, 7, ... get_from_timestamp should work when we
    // provide a timestamp that hits a block as well as when it hits
    // a gap between 2 blocks

    for i in 0..chain_len {

        let block;
        let timestamp = (i*2) + 1;

        if let Some(previous_block) = blocks.last(){
            block = Block::new([0; 32], Some(&previous_block), timestamp, vec![]);
        }else{
            block = Block::new([0; 32], None, timestamp, vec![]);
        }

        let cloned = block.clone();
        let result = storage.append_verified_block(cloned);
        assert!(result.is_ok(), "Could not append block");

        blocks.push(block);

    }

    // test timestamps that hit blocks exactly (get_after_timestamp
    // then behaves similar to get_after, getting the block following
    // the identified block). 

    for i in 0..chain_len{

        let timestamp = (i*2) + 1;

        let fetched_block = storage.get_after_timestamp(timestamp);

        if let Some(block) = fetched_block{

            assert!(i < (chain_len - 1),
                    "get_after_timestamp returned a block \
                     for the timestamp of the tail block");

            let fetched_id = block.get_id();
            let block_index = i as usize + 1;
            let assumed_id = blocks[block_index].get_id();

            assert_eq!(fetched_id, assumed_id,
                       "get_after_timestamp doesn't return \
                        blocks in the right order.");

        }else{

            assert!(i == (chain_len - 1),
                    "get_after_timestamp returned a None \
                     value in a case where a block should \
                     be returned");

        }

    }

    // test timestamps that hit the gaps between the blocks

    for i in 0..chain_len {

        let timestamp = (i*2);
        let block_index = i as usize;
        let assumed_id = blocks[block_index].get_id();
        let fetched_block = storage.get_after_timestamp(timestamp);
        let fetched_id = fetched_block.unwrap().get_id();
        assert_eq!(fetched_id, assumed_id,
                   "get_after_timestamp doesn't return
                    blocks in the correct order");

    }

}
