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
pub struct TxIndex(pub u16);

/// `TxId` denotes a registered transaction in the chain.
/// It is composed of the `BlockId` pointing to a block
/// in the chain and a u16 index pointing to a position
/// in the list of transactions in that block.

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub struct TxId{
    pub block_id: BlockId,
    pub tx_index: TxIndex,
}

impl TxId{

    /// Creates a new `TxId`

    fn new(block_id: BlockId, tx_index: TxIndex) -> TxId{
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
enum TxRel{
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
