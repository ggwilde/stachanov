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

/// `TxRel` denotes the relationship between two transactions.
/// Transactions can relate in various ways to each other.
/// For example workloads can be transformed into coupons,
/// coupons can be redeemed in a relation to a dispatcher
/// collective, etc

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
pub enum TxRel{
    Dummy
}

/// `TxLink` denotes the state of a link between transactions
/// It can either be a SingleLink or a MultiLink with the
/// following semantics:
///
/// * `SingleLink(Option<TxId>)': Used for 1:1 relationships
///         between transactions. A typical example would be
///         the relationship between workloads and labor coupons.
///         Each workload can only be claimed by exactly one
///         future coupon. If the inner value is None, it
///         means the relationship has not been claimed yet.
///
/// * `MultiLink(Vec<TxId>)`: Used for 1:n relationships
///         between transactions. Instead of a single (optional)
///         transaction id, MultiLink wraps a list of ids to future
///         transactions. This is for example used in linkmaps
///         for Order transactions, which can have multiple
///         workloads attached to it. It should be emphasized,
///         that it is the responsibility of the link
///         verification mechanism to check if possible
///         quotas are exceeded.

#[derive(Clone)]
enum TxLink{
    SingleLink(Option<TxId>),
    MultiLink(Vec<TxId>)
}

/// `TxLinkMapState` denotes the general link state of a
/// transaction. Depending on its value it can overwrite
/// the single link states. It can take the values:
///
/// * `Linkable`: The transaction can _generally_ be linked
///         to other transactions. This means that the linking
///         permit depends on the state of `TxLink` for
///         each relationship.
/// * `Unlinkable`: The transaction can not be linked to any
///         other transaction (anymore). This state should
///         be used when a transaction type is deprecated
///         by a later system version or if the list of
///         relationships for the transaction is empty
/// * `Finalized`: The transaction was finalized by another
///         transaction and can not link to a future target
///         transaction anymore.

#[derive(Clone)]
enum TxLinkMapState{
    Linkable,
    Unlinkable,
    Finalized(TxId),
}

/// `TxLinkMap` defines existing and possible links of a transaction
/// to future transactions. It maps transaction relationships to
/// `TxLink`s and holds a `TxLinkMapState` to denote the general
/// link state of the transaction.

#[derive(Clone)]
pub struct TxLinkMap{
    state: TxLinkMapState,
    links: HashMap<TxRel, TxLink>
}
