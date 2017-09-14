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
use blockchain::block::BlockId;
use blockchain::transactions::TxId;
use blockchain::transactions::TxIndex;
use blockchain::transactions::TxRel;
use blockchain::transactions::TxRelId;
use blockchain::transactions::TxState;
use blockchain::transactions::TxTotalRelState;
use blockchain::transactions::TxProgErrorReason;
use blockchain::transactions::Transaction;

/// Tests if a storage returns None on
/// nonexistent `TxState`s
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_nonexistent<T>(storage: &mut T) where T: ChainStorage{

    let block_id = BlockId([0; 32]);
    let tx_id = TxId::new(block_id, TxIndex(100));

    let tx_state = storage.get_transaction_state(tx_id);
    assert!(tx_state.is_none(),
            "get_transaction_state returned a result \
             for a non-existent transaction");

}

/// Tests if setting `TxState`s for
/// non-existent transaction fails
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_set_fail<T>(storage: &mut T) where T: ChainStorage{

    let block_id = BlockId([0; 32]);
    let tx_id = TxId::new(block_id, TxIndex(100));

    let tx_state = TxState::new(TxTotalRelState::Claimable);
    let result = storage.set_transaction_state(tx_id, tx_state);
    assert!(result.is_err(),
            "set_transaction_state allowed setting a \
             state for a non-existent transaction");

}

/// Tests if total relationship state `Claimable`
/// is saved correctly in a `TxState`
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_claimable<T>(storage: &mut T) where T: ChainStorage{

    // append block with one transaction

    let block = Block::new([0; 32], None, 0, vec![Transaction::Dummy]);
    let block_id = block.get_id();
    let tx_id = TxId::new(block_id, TxIndex(0));

    let result = storage.append_verified_block(block);
    assert!(result.is_ok(), "Could not append block");

    // set total relationship state of the
    // transaction to Claimable

    let tx_state = TxState::new(TxTotalRelState::Claimable);

    let result = storage.set_transaction_state(tx_id, tx_state);
    assert!(result.is_ok(),
            "set_transaction_state returned an error \
             when trying to set a state with total rel
             state 'Claimable' and no relationships");

    // Should get a Txstate with total
    // rel state 'Claimable' back

    let fetched_tx_state = storage.get_transaction_state(tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    if let Some(fetched_tx_state) = fetched_tx_state{

        let rel_state = fetched_tx_state.get_total_rel_state();
        match *rel_state{
            TxTotalRelState::Claimable => {},
            _ => {
                assert!(false, "Total relationship state \
                                'Claimable' was not returned \
                                properly");
            }
        }

    }

}

/// Tests if total relationship state `Unclaimable`
/// is saved correctly in a `TxState`
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_unclaimable<T>(storage: &mut T) where T: ChainStorage{

    // append block with one transaction

    let block = Block::new([0; 32], None, 0, vec![Transaction::Dummy]);
    let block_id = block.get_id();
    let tx_id = TxId::new(block_id, TxIndex(0));

    let result = storage.append_verified_block(block);
    assert!(result.is_ok(), "Could not append block");

    // set total relationship state to Unclaimable

    let tx_state = TxState::new(TxTotalRelState::Unclaimable);

    let result = storage.set_transaction_state(tx_id, tx_state);
    assert!(result.is_ok(),
            "set_transaction_state returned an error \
             when trying to set a state with total rel
             state 'Unclaimable' and no relationships");

    // Should get a Txstate with total
    // rel state 'Unclaimable' back

    let fetched_tx_state = storage.get_transaction_state(tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    if let Some(fetched_tx_state) = fetched_tx_state{

        let rel_state = fetched_tx_state.get_total_rel_state();
        match *rel_state{
            TxTotalRelState::Unclaimable => {},
            _ => {
                assert!(false, "Total relationship state \
                                'Unclaimable' was not returned \
                                properly");
            }
        }

    }

}

/// Tests if total relationship state `Finalized`
/// is handled correctly by the storage
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_finalized<T>(storage: &mut T) where T: ChainStorage{

    // first, we setup a chain with 2 different blocks,
    // the first including two dummy transactions, the
    // second a single dummy transaction

    //  .---------.      .---------.
    //  | block 1 | o--o | block 2 |
    //  |---------|      |---------|
    //  |  tx 1   |      |  tx 1   |
    //  |  tx 2   |      '---------'
    //  '---------'

    let first_block = Block::new([0; 32], None, 0,
                                 vec![Transaction::Dummy,
                                      Transaction::Dummy]);

    let first_block_id = first_block.get_id();
    let first_block_tx_id = TxId::new(first_block_id, TxIndex(0));
    let first_block_tx_id2 = TxId::new(first_block_id, TxIndex(1));

    let second_block = Block::new([0; 32], Some(&first_block), 0,
                                  vec![Transaction::Dummy]);

    let second_block_id = second_block.get_id();
    let second_block_tx_id = TxId::new(second_block_id, TxIndex(0));

    // append both blocks to the storage

    let result = storage.append_verified_block(first_block);
    assert!(result.is_ok(), "Could not append block");

    let result = storage.append_verified_block(second_block);
    assert!(result.is_ok(), "Could not append block");

    // 1. Finalize first transaction of first block
    //    with first transaction of second block

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | <--- |  tx 1   |
    //     |  tx 2   |      '---------'
    //     '---------'

    let total_rel_state = TxTotalRelState::Finalized(second_block_tx_id);
    let tx_state = TxState::new(total_rel_state);

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_ok(),
            "set_transaction_state returned an error \
             when trying to set a state with total rel
             state 'Finalized' and no relationships");

    //    we should get a Txstate with total
    //    rel state 'Finalized' back

    let fetched_tx_state = storage.get_transaction_state(first_block_tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    if let Some(fetched_tx_state) = fetched_tx_state{

        let rel_state = fetched_tx_state.get_total_rel_state();

        match *rel_state{

            TxTotalRelState::Finalized(fin_tx_id) => {

                // check if the finalizer transaction id
                // was returned correctly from the store

                assert_eq!(fin_tx_id, second_block_tx_id,
                           "Setting total rel state of transaction \
                            state to Finalized failed. Id of the \
                            finalizer transaction was not returned \
                            properly");
            },

            _ => {
                assert!(false, "Total relationship state \
                                'Finalized' was not returned \
                                properly");
            }

        }

    }

    // 2. Finalizing transaction of second block with one of the
    //    transactions of the first block should fail (transactions
    //    can only be finalized by future transactions, not past ones)

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | ---> |  tx 1   |
    //     |  tx 2   |      '---------'
    //     '---------'

    let total_rel_state = TxTotalRelState::Finalized(first_block_tx_id);
    let tx_state = TxState::new(total_rel_state);

    let result = storage.set_transaction_state(second_block_tx_id, tx_state);

    assert!(result.is_err(),
            "Storage allowed finalizing a transaction by a transaction \
             from the past. Transactions can only be finalized by \
             transactions from one of the subsequent blocks");

    //    check, if error reason is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::RefOrderError => {},
            _ => {
                assert!(false, "Storage returned a wrong error \
                                when trying to finalize a transaction \
                                by a transaction in the past");
            }
        }
    }

    // 3. Finalizing transaction of first block with a transaction
    //    of the first block should fail

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | <--. |  tx 1   |
    //     |  tx 2   | ---' '---------'
    //     '---------'

    let total_rel_state = TxTotalRelState::Finalized(first_block_tx_id2);
    let tx_state = TxState::new(total_rel_state);

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);

    assert!(result.is_err(),
            "Storage allowed finalizing a transaction by a transaction \
             in the same block. Transactions can only be finalized by \
             transactions from one of the subsequent blocks");

    //    check, if error is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::RefOrderError => {},
            _ => {
                assert!(false, "Storage returned a wrong error \
                                when trying to finalize a transaction \
                                by a transaction in the same block");
            }
        }
    }

    // 4. Finalizing transaction of first block with a non-existent
    //    transaction should fail

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | <--. |  tx 1   |
    //     |  tx 2   |    | '---------'
    //     '---------'    |

    let faulty_tx_id = TxId::new(second_block_id, TxIndex(0xDEAD));

    let total_rel_state = TxTotalRelState::Finalized(faulty_tx_id);
    let tx_state = TxState::new(total_rel_state);

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);

    assert!(result.is_err(),
            "Storage allowed finalizing a transaction by a transaction \
             id that points to a nonexistent transaction");

    //    check, if error is correct

    if let Err(err) = result{

        match err.reason{

            TxProgErrorReason::UnknownTx(tx_id) => {

                // check if the id wrapped in the error
                // reason is the same as the one put
                // into the relationship

                assert_eq!(tx_id, faulty_tx_id,
                           "Storage returned the wrong transaction \
                            id when returning TxProgError with \
                            reason UnknownTxId");

            },

            _ => {
                assert!(false, "Storage returned a wrong error \
                                when trying to finalize a transaction \
                                by a non-existent transaction");
            }

        }

    }

}

/// Tests if 1:1 relationships are handled
/// correctly by the storage
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_one_to_one_rel<T>(storage: &mut T) where T: ChainStorage{

    // first, we setup a chain with 2 different blocks,
    // the first including two dummy transactions, the
    // second a single dummy transaction

    //  .---------.      .---------.
    //  | block 1 | o--o | block 2 |
    //  |---------|      |---------|
    //  |  tx 1   |      |  tx 1   |
    //  |  tx 2   |      '---------'
    //  '---------'

    let first_block = Block::new([0; 32], None, 0,
                                 vec![Transaction::Dummy,
                                      Transaction::Dummy]);

    let first_block_id = first_block.get_id();
    let first_block_tx_id = TxId::new(first_block_id, TxIndex(0));
    let first_block_tx_id2 = TxId::new(first_block_id, TxIndex(1));

    let second_block = Block::new([0; 32], Some(&first_block), 0,
                                  vec![Transaction::Dummy]);

    let second_block_id = second_block.get_id();
    let second_block_tx_id = TxId::new(second_block_id, TxIndex(0));

    // append both blocks to the storage

    let result = storage.append_verified_block(first_block);
    assert!(result.is_ok(), "Could not append block");

    let result = storage.append_verified_block(second_block);
    assert!(result.is_ok(), "Could not append block");

    // 1. Check if a 1:1 relationship in a `TxState` 
    //    is returned correctly, when it is unclaimed

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_one_rel(TxRelId::Dummy).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_ok(),
            "Setting transaction state for an existing \
             transaction returned an error when an unclaimed \
             1:1 relationship was used in the state");

    //    we should get a Txstate with unclaimed 1:1
    //    dummy relationship back

    let fetched_tx_state = storage.get_transaction_state(first_block_tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    let fetched_tx_state = fetched_tx_state.unwrap();
    let relationship = fetched_tx_state.get_rel(TxRelId::Dummy).unwrap();

    match *relationship{
        TxRel::OneToOne(ref_tx_id) => {
            assert!(ref_tx_id.is_none(),
                    "After setting an unclaimed 1:1 relationship the \
                     storage returned a claimed relationship");
        }
        _ => {
            assert!(false, "Storage returned a wrong type of relationship \
                            for TxState. Saved a OneToOne relationship, \
                            but got a different relationship back");
        }
    }

    // 2. Claim the dummy relationship of tx 1 in block 1,
    //    save it and check if result is correct

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | <--- |  tx 1   |
    //     |  tx 2   |      '---------'
    //     '---------'

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_one_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, second_block_tx_id).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_ok(),
            "Setting transaction state for an existing \
             transaction returned an error when a legally \
             claimed 1:1 relation was used in the state");

    //    We should get a Txstate with claimed 1:1
    //    dummy relationship back

    let fetched_tx_state = storage.get_transaction_state(first_block_tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    let fetched_tx_state = fetched_tx_state.unwrap();
    let relationship = fetched_tx_state.get_rel(TxRelId::Dummy).unwrap();

    match *relationship{

        TxRel::OneToOne(ref_tx_id) => {

            // check, if the id of the claimer
            // transaction has not changed in
            // the storage

            let ref_tx_id = ref_tx_id.unwrap();
            assert_eq!(ref_tx_id, second_block_tx_id,
                       "After setting a claimed 1:1 relationship the \
                        storage returned a wrong value for the claimer \
                        transaction id");

        }

        _ => {
            assert!(false, "Storage returned a wrong type of relationship \
                            for TxState. Saved a OneToOne relationship, \
                            but got a different relationship back");
        }

    }

    // 3. Check if storage returns an error, if a transaction
    //    is claimed by a transaction from the past.

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | ---> |  tx 1   |
    //     |  tx 2   |      '---------'
    //     '---------'

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_one_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, first_block_tx_id).unwrap();

    let result = storage.set_transaction_state(second_block_tx_id, tx_state);
    assert!(result.is_err(),
            "1:1 relationship was claimed by a transaction in the past, \
             however the storage did not return an error");

    //    check, if the error reason is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::RefOrderError => {},
            _ => {
                assert!(false,
                        "Storage returned a wrong error reason \
                         when we tried to claim a relationship by \
                         a transaction living in the past. Reason \
                         should be RefOrderError");
            }
        }
    }

    // 4. check if storage returns an error if a relationship
    //    is claimed by a transaction from the same block

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | <--. |  tx 1   |
    //     |  tx 2   | ---' '---------'
    //     '---------'

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_one_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, first_block_tx_id).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id2, tx_state);
    assert!(result.is_err(),
            "1:1 relationship was claimed by a transction in the same, \
             block, however the storage did not return error");

    //    check, if the error reason is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::RefOrderError => {},
            _ => {
                assert!(false,
                        "Storage returned a wrong error reason \
                         when we tried to claim a relationship by \
                         a transaction living in the same block. \
                         Reason should be RefOrderError");
            }
        }
    }
    
    // 5. check if storage returns an error, if the transaction
    //    id of the claimer doesn't point to a real transaction

    //     .---------.      .---------.
    //     | block 1 | o--o | block 2 |
    //     |---------|      |---------|
    //     |  tx 1   | <--. |  tx 1   |
    //     |  tx 2   |    | '---------'
    //     '---------'    |

    let faulty_tx_id = TxId::new(second_block_id, TxIndex(0xDEAD));

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_one_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, faulty_tx_id).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_err(),
            "1:1 relationship was claimed by a non-existent transaction \
             however the storage did not return error");

    //    check, if the error reason is correct

    if let Err(err) = result{

        match err.reason{

            TxProgErrorReason::UnknownTx(tx_id) => {

                // check if the id wrapped in the error
                // reason is the same as the one put
                // into the relationship

                assert_eq!(tx_id, faulty_tx_id,
                           "Storage returned the wrong transaction \
                            id when returning TxProgError with \
                            reason UnknownTxId");

            },

            _ => {
                assert!(false,
                        "Storage returned a wrong error reason \
                         when we tried to claim a relationship by \
                         a non-existing transaction. Reason \
                         should be UnknownTx");
            }

        }

    }

}

/// Tests if 1:n relationships are handled
/// correctly by the storage
///
/// # Arguments
/// * `storage`: A storage object that implements
///              the `ChainStorage` trait

pub fn test_txstate_one_to_many_rel<T>(storage: &mut T) where T: ChainStorage{

    // append 3 blocks with the following transactions

    //  .---------.      .---------.      .---------.
    //  | block 1 | o--o | block 2 | o--o | block 3 |
    //  |---------|      |---------|      |---------|
    //  |  tx 1   |      |  tx 1   |      |  tx 1   |
    //  '---------'      |  tx 2   |      '---------'
    //                   '---------'                 


    let first_block = Block::new([0; 32], None, 0,
                                 vec![Transaction::Dummy]);

    let first_block_id = first_block.get_id();
    let first_block_tx_id = TxId::new(first_block_id, TxIndex(0));

    let second_block = Block::new([0; 32], Some(&first_block), 0,
                                  vec![Transaction::Dummy,
                                       Transaction::Dummy]);

    let second_block_id = second_block.get_id();
    let second_block_tx_id = TxId::new(second_block_id, TxIndex(0));
    let second_block_tx_id2 = TxId::new(second_block_id, TxIndex(1));

    let third_block = Block::new([0; 32], Some(&second_block), 0,
                                  vec![Transaction::Dummy]);

    let third_block_id = third_block.get_id();
    let third_block_tx_id = TxId::new(third_block_id, TxIndex(0));

    // append all three blocks to the storage

    let result = storage.append_verified_block(first_block);
    assert!(result.is_ok(), "Could not append block");

    let result = storage.append_verified_block(second_block);
    assert!(result.is_ok(), "Could not append block");

    let result = storage.append_verified_block(third_block);
    assert!(result.is_ok(), "Could not append block");

    // 1. check if unclaimed 1:n relationships are
    //    returned back correctly

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_ok(),
            "set_transaction_state returned an error when \
             an unclaimed 1:n relationship was used in the \
             state of an existing transaction");

    //    We should get a Txstate with unclaimed 1:n
    //    dummy relationship back

    let fetched_tx_state = storage.get_transaction_state(first_block_tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    let fetched_tx_state = fetched_tx_state.unwrap();
    let relationship = fetched_tx_state.get_rel(TxRelId::Dummy).unwrap();

    match *relationship{
        TxRel::OneToMany(ref ref_tx_ids) => {
            assert!(ref_tx_ids.is_empty(),
                    "After setting an unclaimed 1:n relationship the \
                     storage returned a claimed relationship");
        }
        _ => {
            assert!(false, "Storage returned a wrong type of relationship \
                            for TxState. Saved a OneToMany relationship, \
                            but got a different relationship back");
        }
    }

    // 2. check if claiming a 1:n relationships once
    //    works correctly

    //    .---------.      .---------.      .---------.
    //    | block 1 | o--o | block 2 | o--o | block 3 |
    //    |---------|      |---------|      |---------|
    //    |  tx 1   | <--- |  tx 1   |      |  tx 1   |
    //    '---------'      |  tx 2   |      '---------'
    //                     '---------'                 

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, second_block_tx_id).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_ok(),
            "set_transaction_state returned an error when \
             a claimed 1:n relationship with one claimer \
             was used in the state of an existing transaction");

    //    We should get a TxState with a claimed 1:n
    //    dummy relationship back

    let fetched_tx_state = storage.get_transaction_state(first_block_tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    let fetched_tx_state = fetched_tx_state.unwrap();
    let relationship = fetched_tx_state.get_rel(TxRelId::Dummy).unwrap();

    match *relationship{
        TxRel::OneToMany(ref ref_tx_ids) => {

            assert!(ref_tx_ids.len() == 1,
                    "After setting a claimed 1:n relationship with one \
                     claimer the storage returned a relationship with \
                     a claimer count unequal to one");

            let ref_tx_id = ref_tx_ids[0];
            assert_eq!(ref_tx_id, second_block_tx_id,
                       "Storage returned a wrong claimer id after \
                        setting a TxState with a dummy 1:n relationship");

        }
        _ => {
            assert!(false, "Storage returned a wrong type of relationship \
                            for TxState. Saved a OneToMany relationship, \
                            but got a different relationship back");
        }
    }

    // 3. check if claiming a 1:n relationships twice
    //    works correctly

    //    .---------.      .---------.      .---------.
    //    | block 1 | o--o | block 2 | o--o | block 3 |
    //    |---------|      |---------|      |---------|
    //    |  tx 1   | <-.- |  tx 1   |      |  tx 1   |
    //    '---------'   '- |  tx 2   |      '---------'
    //                     '---------'                 

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, second_block_tx_id).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, second_block_tx_id2).unwrap();

    let result = storage.set_transaction_state(first_block_tx_id, tx_state);
    assert!(result.is_ok(),
            "set_transaction_state returned an error when \
             a claimed 1:n relationship with two claimers \
             was used in the state of an existing transaction");

    //    We should get a TxState with claimed 1:n
    //    dummy relationship back

    let fetched_tx_state = storage.get_transaction_state(first_block_tx_id);
    assert!(fetched_tx_state.is_some(),
            "Could not retrieve existing
             transaction state");

    let fetched_tx_state = fetched_tx_state.unwrap();
    let relationship = fetched_tx_state.get_rel(TxRelId::Dummy).unwrap();

    match *relationship{
        TxRel::OneToMany(ref ref_tx_ids) => {

            assert!(ref_tx_ids.len() == 2,
                    "After setting a claimed 1:n relationship with two \
                     claimers the storage returned a relationship with \
                     a claimer count unequal to two");

            let ref_tx_id = ref_tx_ids[0];
            assert_eq!(ref_tx_id, second_block_tx_id,
                       "Storage returned a wrong first claimer id after \
                        setting a TxState with a dummy 1:n relationship");

            let ref_tx_id2 = ref_tx_ids[1];
            assert_eq!(ref_tx_id2, second_block_tx_id2,
                       "Storage returned a wrong second claimer id after \
                        setting a TxState with a dummy 1:n relationship");


        }
        _ => {
            assert!(false, "Storage returned a wrong type of relationship \
                            for TxState. Saved a OneToMany relationship, \
                            but got a different relationship back");
        }
    }

    // 4. check if claiming a 1:n relationships with two
    //    claimers - one living in a past block - fails

    //    .---------.      .---------.      .---------.
    //    | block 1 | o--o | block 2 | o--o | block 3 |
    //    |---------|      |---------|      |---------|
    //    |  tx 1   | ---> |  tx 1   | <--- |  tx 1   |
    //    '---------'      |  tx 2   |      '---------'
    //                     '---------'                 

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, third_block_tx_id).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, first_block_tx_id).unwrap();

    let result = storage.set_transaction_state(second_block_tx_id, tx_state);
    assert!(result.is_err(),
            "set_transaction_state didn't return an error when \
             we tried to claim an 1:n relationship by a transaction \
             living in a past block");

    //    check, if the error reason is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::RefOrderError => {},
            _ => {
                assert!(false,
                        "Storage returned a wrong error reason \
                         when we tried to claim a 1:n relationship by \
                         a transaction living in a preceding block. \
                         Reason should be RefOrderError");
            }
        }
    }

    // 5. check if claiming a 1:n relationships with two
    //    claimers - one living in the same block - fails

    //    .---------.      .---------.      .---------.
    //    | block 1 | o--o | block 2 | o--o | block 3 |
    //    |---------|      |---------|      |---------|
    //    |  tx 1   | .--> |  tx 1   | <--- |  tx 1   |
    //    '---------' '--- |  tx 2   |      '---------'
    //                     '---------'                 

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, third_block_tx_id).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, second_block_tx_id2).unwrap();

    let result = storage.set_transaction_state(second_block_tx_id, tx_state);
    assert!(result.is_err(),
            "set_transaction_state didn't return an error when \
             we tried to claim an 1:n relationship by a transaction \
             living in the same block");

    //    check, if the error reason is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::RefOrderError => {},
            _ => {
                assert!(false,
                        "Storage returned a wrong error reason \
                         when we tried to claim a 1:n relationship by \
                         a transaction living in the same block. \
                         Reason should be RefOrderError");
            }
        }
    }

    // 6. check if setting an 1:n relationship to two
    //    different transaction ids, where one of them
    //    points to an unknown transaction, fails

    //    .---------.      .---------.      .---------.
    //    | block 1 | o--o | block 2 | o--o | block 3 |
    //    |---------|      |---------|      |---------|
    //    |  tx 1   | .--> |  tx 1   | <--- |  tx 1   |
    //    '---------' |    |  tx 2   |      '---------'
    //                |    '---------'                 

    let faulty_tx_id = TxId::new(second_block_id, TxIndex(0xDEAD));

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, third_block_tx_id).unwrap();
    tx_state.claim_rel(TxRelId::Dummy, faulty_tx_id).unwrap();

    let result = storage.set_transaction_state(second_block_tx_id, tx_state);
    assert!(result.is_err(),
            "set_transaction_state didn't return an error when \
             we tried to claim an 1:n relationship by a transaction \
             that doesn't exist");

    //    check, if the error reason is correct

    if let Err(err) = result{
        match err.reason{
            TxProgErrorReason::UnknownTx(tx_id) => {

                // check if the id wrapped in the error
                // reason is the same as the one put
                // into the relationship

                assert_eq!(tx_id, faulty_tx_id,
                           "Storage returned the wrong transaction \
                            id when returning TxProgError with \
                            reason UnknownTxId");

            },
            _ => {
                assert!(false,
                        "Storage returned a wrong error reason \
                         when we tried to claim a 1:n relationship by \
                         a transaction that doesn't exist. \
                         Reason should be UnknownTx");
            }
        }
    }
}
