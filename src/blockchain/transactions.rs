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

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use blockchain::traits::Hashable;
use blockchain::block::BlockId;

// NOTE: This module currently only consists of
// a dummy implementation

#[derive(Clone)]
pub enum Transaction{
    Dummy
}

impl Hashable for Transaction{

    fn to_sha3_hash(&self) -> [u8; 32]{
        [0; 32]
    }

}

/// `TxIndex` typewraps an u16 int. It denotes the index
/// in the transaction vector of a single block

// NOTE: since we can address only 2**16 transactions,
// the maximum number of transactions per block is
// limited to 65536 transactions. I chose an u16 over
// a u32, so systems with smaller memory are not
// overburdend.

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
pub struct TxIndex(pub u16);

/// `TxId` denotes a registered transaction in the chain.
/// It is composed of the `BlockId` pointing to a block
/// in the chain and a u16 index pointing to a position
/// in the list of transactions in that block.

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Copy)]
pub struct TxId{
    pub block_id: BlockId,
    pub tx_index: TxIndex,
}

impl TxId{

    /// Creates a new `TxId`

    pub fn new(block_id: BlockId, tx_index: TxIndex) -> TxId{
        TxId{block_id: block_id, tx_index: tx_index}
    }

}

/// `TxRelId` acts as a unique identifier for a relationship
/// between transactions. Transactions can relate in various
/// ways to each other. For example workloads can be used
/// to create coupons, orders are in a relationship to a
/// production output, etc

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
pub enum TxRelId{
    Dummy
}

/// `TxRel` denotes the state of a 1:1 or 1:n relationship
/// between transactions. It can either be a OneToOne or
/// a OneToMany with the following semantics:
///
/// * `OneToOne(Option<TxId>)': Used for 1:1 relationships
///         between transactions. A typical example would be
///         the relationship between workloads and labor coupons.
///         Each workload can only be claimed by exactly one
///         future coupon. If the inner value is None, it
///         means the relationship has not been claimed yet.
///
/// * `OneToMany(Vec<TxId>)`: Used for 1:n relationships
///         between transactions. Instead of a single (optional)
///         transaction id, OneToMany wraps a list of ids to future
///         transactions. This is for example used in transaction
///         states of order transactions, which can have multiple
///         workloads attached to it. It should be emphasized,
///         that it is the responsibility of the relationship
///         verification mechanism to check if possible
///         quotas are exceeded.

#[derive(Clone)]
pub enum TxRel{
    OneToOne(Option<TxId>),
    OneToMany(Vec<TxId>)
}

/// `TotalRelState` denotes the general state of relationships.
/// It is a part of `TxState` and is checked _before_ the list
/// of relationships. It can take the values:
///
/// * `Claimable`: The transaction's relationships can _generally_
///         be claimed by other transactions. This means that it
///         depends on the state of the specific relationship,
///         if an attempt to claim it is succesful
/// * `Unclaimable`: The transaction can not be claimed by any
///         other transaction (anymore). This state should
///         be used when a transaction type is deprecated
///         by a later system version or if the list of
///         relationships for the transaction is empty
/// * `Finalized`: The transaction was finalized by another
///         transaction and none of its relationships can
///         by claimed by other transactions anymore.

#[derive(Clone)]
#[derive(Debug)]
pub enum TxTotalRelState{
    Claimable,
    Unclaimable,
    Finalized(TxId),
}

/// `TxState` defines the volatile part of transactions. Transactions
/// as such can not be changed after they have been added to a block,
/// however depending on later transactions they can change their
/// state. The main purpose of managing the state is to ensure that
/// transactions relate to each other in the right way (e.g. that
/// coupons can not be double spent)

#[derive(Clone)]
pub struct TxState{
    total_rel_state: TxTotalRelState,
    relationships: HashMap<TxRelId, TxRel>
}

// ------------------------------------------------------------------------

/// `BadClaimReason` defines possible reasons
/// for a `BadClaim` error
///
/// * `RelClaimed` is used as a reason when a relationship was
///         already claimed by another transaction. This happens
///         e.g. when someone tries to transform a workload into
///         a coupon twice. The value wraps the relationship id
///         that was supplied to claim_rel as well as the
///         transaction id that points to the transaction
///         that claimed the transaction first
/// * `TxUnclaimable` is used, when the total relationship state
///         of TxState is set to Unclaimable
/// * `TxFinalized` is used, when the total relationship state
///         of TxState is set to Finalized, meaning that it can
///         not be changed anymore. The wrapped transaction id
///         points to the transaction that finalized the state.
/// * `UnknownRelId` happens when the transaction layer tries
///         to claim a relationship that simply doesn't exist.
///         Note, that this is a programming error, not an
///         error caused by a faulty user input. The wrapped
///         `TxRelId` is the relationship id that the
///         transaction layer tried to fetch. `UnknownRelId`
///         error reasons also serve a purpose for migrating
///         from older chain version, when a newer version
///         adds new relationships to a transaction type.

#[derive(Debug)]
pub enum BadClaimReason{
    RelClaimed(TxRelId, TxId),
    TxUnclaimable,
    TxFinalized(TxId),
    UnknownRelId(TxRelId),
}

impl fmt::Display for BadClaimReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BadClaimReason::RelClaimed(ref tx_rel_id, ref tx_id) =>
                write!(f, "Relationship {:?} was already claimed by {:?}", tx_rel_id, tx_id),
            BadClaimReason::TxUnclaimable =>
                write!(f, "Transaction is unclaimable"),
            BadClaimReason::TxFinalized(ref fin_tx_id) =>
                write!(f, "Transaction was finalized by transaction {:?}", fin_tx_id),
            BadClaimReason::UnknownRelId(ref tx_rel_id) =>
                write!(f, "Transaction has no relationship {:?}.", tx_rel_id),
        }
    }
}

/// `BadClaim`s happen when transactions illegaly try to
/// claim a relationship of another transactions. For
/// possible reasons look up the docs of `BadClaimReason`

#[derive(Debug)]
pub struct BadClaim{
    pub reason: BadClaimReason
}

impl BadClaim{
    pub fn new(reason: BadClaimReason) -> BadClaim{
        BadClaim{reason: reason}
    }
}

impl Error for BadClaim{
    fn description(&self) -> &str{
        "Error while claiming relationship"
    }
}

impl fmt::Display for BadClaim{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad relationship claim. Reason: {}", self.reason)
    }
}

/// `TxProgErrorReason` defines possible reasons for
/// `TxProgError`s:
///
///  * `UnknownRelId`: The requested relationship id does
///         not exist. The wrapped value is the requested
///         relationship id. Note, that there also is a
///         `UnknownRelId` in `BadClaimReason`. The
///         semantics are the same, however the use
///         case and the handling is different.
///  * `RelIdExists`: The requested relationship id does
///         already exist. This happens, when the transaction
///         layer tries to overwrite an existing relationship
///         upon initialization. This is merely meant as a
///         safeguard, especially for migration logic.
/// * `UnknownTx`: The requested transaction does not exist.
///         Wraps the transaction id that caused the error.
/// * `RefOrderError`: This happens, when the verification
///         logic of a transaction tries to create a state
///         of the blockchain in which a later transaction
///         is claimed by an earlier transaction or a
///         transaction in the same block.

#[derive(Debug)]
pub enum TxProgErrorReason{
    UnknownRelId(TxRelId),
    RelIdExists(TxRelId),
    UnknownTx(TxId),
    RefOrderError
}

impl fmt::Display for TxProgErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TxProgErrorReason::UnknownRelId(ref tx_rel_id) =>
                write!(f, "Transaction has no relationship {:?}.", tx_rel_id),
            TxProgErrorReason::RelIdExists(ref tx_rel_id) =>
                write!(f, "Transaction already has a relationship {:?}.", tx_rel_id),
            TxProgErrorReason::UnknownTx(ref tx_id) =>
                write!(f, "Unknown transaction. Requested id was {:?}.", tx_id),
            TxProgErrorReason::RefOrderError =>
                write!(f, "The reference order of the transactions is illegal."),
        }
    }
}

/// `TxProgError`s signify problems during
/// transaction handling. For possible
/// reasons look up the docs
/// of `TxProgErrorReason`

#[derive(Debug)]
pub struct TxProgError{
    pub reason: TxProgErrorReason
}

impl TxProgError{
    pub fn new(reason: TxProgErrorReason) -> TxProgError{
        TxProgError{reason: reason}
    }
}

impl Error for TxProgError{
    fn description(&self) -> &str{
        "Programming Error in TxRel handling"
    }
}

impl fmt::Display for TxProgError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Programming Error in TxRel handling. Reason: {}", self.reason)
    }
}

// ------------------------------------------------------------------------


impl TxState{

    /// Creates a new `TxState`
    ///
    /// # Arguments
    /// * `total_rel_state`: The initial total relationship
    ///         state for the transaction

    pub fn new(total_rel_state: TxTotalRelState) -> TxState{

        let relationships = HashMap::new();
        TxState{
            total_rel_state: total_rel_state,
            relationships: relationships
        }

    }

    /// Returns a reference to the total relationship state

    pub fn get_total_rel_state(&self) -> &TxTotalRelState{
        &self.total_rel_state
    }

    /// Changes the total relationship state

    pub fn set_total_rel_state(&mut self,
                               total_rel_state: TxTotalRelState){
        self.total_rel_state = total_rel_state;
    }

    /// Returns a reference to the relationship map

    pub fn get_rel_map(&self) -> &HashMap<TxRelId, TxRel>{
        &self.relationships
    }

    /// Adds a 1:1 relationship to the state. Use this to create
    /// relationships that can be claimed by exactly one transaction.
    /// Please note, that this is a initialization / migration
    /// method. To change the state of a relationship, use the
    /// claim_rel method.
    ///
    /// Returns a TxProgError if relationship already exists
    ///
    /// # Arguments
    /// * `tx_rel_id`: A unique identifier for the new relationship

    pub fn add_one_to_one_rel(&mut self,
                              tx_rel_id: TxRelId)
                              -> Result<(), TxProgError>{

        if self.relationships.contains_key(&tx_rel_id){
            let reason = TxProgErrorReason::RelIdExists(tx_rel_id);
            let err = TxProgError::new(reason);
            return Err(err)
        }

        let tx_rel = TxRel::OneToOne(None);
        self.relationships.insert(tx_rel_id, tx_rel);
        Ok(())

    }

    /// Adds a 1:n relationship to the state. Use this to create
    /// relationships that can be claimed by multiple transactions.
    /// Please note, that this is a initialization / migration
    /// method. To change the state of a relationship, use the
    /// claim_rel method.
    ///
    /// Returns a TxProgError if relationship already exists
    ///
    /// # Arguments
    /// * `tx_rel_id`: A unique identifier for the new relationship

    pub fn add_one_to_many_rel(&mut self,
                               tx_rel_id: TxRelId)
                               -> Result<(), TxProgError>{

        if self.relationships.contains_key(&tx_rel_id){
            let reason = TxProgErrorReason::RelIdExists(tx_rel_id);
            let err = TxProgError::new(reason);
            return Err(err)
        }

        let tx_rel = TxRel::OneToMany(vec![]);
        self.relationships.insert(tx_rel_id, tx_rel);
        Ok(())

    }

    /// Returns a reference to a specific relationship. This method
    /// is provided for the transaction layer to check validity of
    /// OneToMany relationships, also for testing purposes.
    ///
    /// Returns TxProgError if the relationship doesn't exist
    ///
    /// # Arguments
    /// * `tx_rel_id`: A unique identifier for the relationship

    pub fn get_rel(&self, tx_rel_id: TxRelId) -> Result<&TxRel, TxProgError>{

        match self.relationships.get(&tx_rel_id){

            Some(rel) => return Ok(rel),
            None => {
                let reason = TxProgErrorReason::UnknownRelId(tx_rel_id);
                let err = TxProgError::new(reason);
                return Err(err)
            }

        }

    }

    /// Claims a relationship.
    /// Returns `BadClaim` if relationship can not be claimed.
    /// Bad claims can have multiple reasons. Look at the docs
    /// for `BadClaimReason` to find out more.
    ///
    /// # Arguments
    /// * `tx_rel_id`: Relationship that should be claimed
    /// * `tx_id`: Id of the transaction that wants to claims it

    pub fn claim_rel(&mut self,
                     tx_rel_id: TxRelId,
                     tx_id: TxId) -> Result<(), BadClaim>{

        // Check the total relationship state first. If it is
        // Unclaimable or Finalized the claim is rejected
        // independently of the actual state of the
        // relationship in question

        match self.total_rel_state{
            TxTotalRelState::Unclaimable => {
                let reason = BadClaimReason::TxUnclaimable;
                let err = BadClaim::new(reason);
                return Err(err)
            }
            TxTotalRelState::Finalized(fin_tx_id) => {
                let reason = BadClaimReason::TxFinalized(fin_tx_id);
                let err = BadClaim::new(reason);
                return Err(err)
            }
            _ => {}
        }

        // Transaction is claimable in principle, now check
        // the relationship that should be claimed

        let relationship = self.relationships.get_mut(&tx_rel_id);

        if let Some(relationship) = relationship{

            // relationship was found,
            // check if it is claimable

            match *relationship{

                TxRel::OneToOne(ref mut claimer_tx_id) => {
                    if let Some(claimer_tx_id) = *claimer_tx_id{
                        let reason = BadClaimReason::RelClaimed(tx_rel_id, claimer_tx_id);
                        let err = BadClaim::new(reason);
                        return Err(err)
                    }
                    *claimer_tx_id = Some(tx_id);
                    return Ok(())
                },

                TxRel::OneToMany(ref mut tx_ids) => {
                    tx_ids.push(tx_id);
                    return Ok(())
                }

            }

        }

        // relationship in question was not found
        // return error

        let reason = BadClaimReason::UnknownRelId(tx_rel_id);
        let err = BadClaim::new(reason);
        return Err(err)

    }

}

#[test]
fn test_tx_claim_total_rel_state_unclaimable(){

    // check if claim gets rejected if total state is Unclaimable

    let mut tx_state = TxState::new(TxTotalRelState::Unclaimable);

    let block_id = BlockId([0; 32]);
    let tx_index = TxIndex(0);
    let tx_id = TxId::new(block_id, tx_index);

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id);
    assert!(result.is_err(), "Total rel state is Unclaimable, \
                              but claim was accepted");

    if let Err(err) = result{
        match err.reason{
            BadClaimReason::TxUnclaimable => {},
            _ => assert!(false, "Claim was rejected for the wrong reason")
        }
    }

}

#[test]
fn test_tx_claim_total_rel_state_finalized(){

    // check if claim gets rejected if total state is Finalized

    let fin_block_id = BlockId([1; 32]);
    let fin_tx_index = TxIndex(0);
    let fin_tx_id = TxId::new(fin_block_id, fin_tx_index);
    let error_correct_id = fin_tx_id;

    let mut tx_state = TxState::new(TxTotalRelState::Finalized(fin_tx_id));

    let block_id = BlockId([0; 32]);
    let tx_index = TxIndex(0);
    let tx_id = TxId::new(block_id, tx_index);

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id);
    assert!(result.is_err(), "Total relationship state is finalized, \
                              but claim was accepted");

    if let Err(err) = result{
        match err.reason{
            BadClaimReason::TxFinalized(ref tx_id) => {
                assert_eq!(*tx_id, error_correct_id,
                           "Claim was correctly rejected because the \
                            transaction is in finalized state, however \
                            the id of the finalizer transaction was not \
                            correct");
            },
            _ => assert!(false, "Claim was rejected for the wrong reason")
        }
    }

}

#[test]
fn test_tx_claim_unknown_rel_id(){

    // check if claim gets rejected if relationship id is unknown

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);

    let block_id = BlockId([0; 32]);
    let tx_index = TxIndex(0);
    let tx_id = TxId::new(block_id, tx_index);

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id);
    assert!(result.is_err(), "Claim for unknown relationship was accepted");

    if let Err(err) = result{
        match err.reason{
            BadClaimReason::UnknownRelId(tx_rel_id) => {
                assert_eq!(tx_rel_id, TxRelId::Dummy,
                           "Claim was correctly rejected because of \
                            an unknown relationship id, however the \
                            relationship id wrapped in the error reason \
                            is not correct");
            },
            _ => assert!(false, "Claim was rejected for the wrong reason")
        }
    }

}

#[test]
fn test_tx_claim_one_to_one_rel(){

    // check if 1:1 relationship works correctly

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_one_rel(TxRelId::Dummy);

    let block_id = BlockId([0; 32]);
    let tx_index = TxIndex(0);
    let tx_id = TxId::new(block_id, tx_index);
    let error_correct_id = tx_id;

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id);
    assert!(result.is_ok(), "Unclaimed 1:1 relationship could not get claimed");

    // a second claim should not work

    let block_id2 = BlockId([1; 32]);
    let tx_index2 = TxIndex(1);
    let tx_id2 = TxId::new(block_id2, tx_index2);

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id2);
    assert!(result.is_err(), "Claimed 1:1 relationship could get claimed");

    if let Err(err) = result{
        match err.reason{
            BadClaimReason::RelClaimed(tx_rel_id, tx_id) => {
                assert_eq!(tx_rel_id, TxRelId::Dummy,
                           "Claim was correctly rejected because the \
                            relationship is fully claimed, however the \
                            relationship id wrapped in the error reason \
                            is not correct");
                assert_eq!(tx_id, error_correct_id,
                           "Claim was correctly rejected because the \
                            relationship is fully claimed, however the \
                            id of the transaction which originally claimed \
                            the transaction is not correct");
            },
            _ => assert!(false, "Claim was rejected for the wrong reason")
        }
    }

}

#[test]
fn test_tx_claim_one_to_many_rel(){

    // check if 1:n relationship works correctly

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);
    tx_state.add_one_to_many_rel(TxRelId::Dummy);

    let block_id = BlockId([0; 32]);
    let tx_index = TxIndex(0);
    let tx_id = TxId::new(block_id, tx_index);

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id);
    assert!(result.is_ok(), "Unclaimed 1:n relationship \
                             could not get claimed");

    // a second claim should work

    let block_id = BlockId([1; 32]);
    let tx_index = TxIndex(1);
    let tx_id = TxId::new(block_id, tx_index);

    let result = tx_state.claim_rel(TxRelId::Dummy, tx_id);
    assert!(result.is_ok(), "1:n transaction that was claimed once \
                             could not get claimed a second time");

}

#[test]
fn test_tx_create_rel(){

    // check if create_*_rel methods work correctly

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);

    let first_def = tx_state.add_one_to_one_rel(TxRelId::Dummy);
    assert!(first_def.is_ok(), "add_one_to_one_rel did fail");

    let second_def = tx_state.add_one_to_one_rel(TxRelId::Dummy);
    assert!(second_def.is_err(), "add_one_to_one_rel did permit rel id overwrite");

    let mut tx_state = TxState::new(TxTotalRelState::Claimable);

    let first_def = tx_state.add_one_to_many_rel(TxRelId::Dummy);
    assert!(first_def.is_ok(), "add_one_to_many_rel did fail");

    let second_def = tx_state.add_one_to_many_rel(TxRelId::Dummy);
    assert!(second_def.is_err(), "add_one_to_many_rel did permit rel id overwrite");

}
