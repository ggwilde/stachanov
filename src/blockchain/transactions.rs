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

/// `TxLinkState` denotes the state of a (possible) link
/// between transactions
///
/// * `Unlinked`: The link points to no future target so far
/// * `SingleLinked(TxId)': The link points to a single target
///         in the future and is seen as 'spent', which means,
///         no other target transaction can be linked to the
///         source transaction in the same relationship.
/// * `MultiLinked(Vec<TxId>)`: The link points to a set of
///         transactions in the future. This is utilized by
///         transactions such as Order, that can have links
///         to multiple workloads. If a transaction is multi-
///         linked, it is the responsibility of the link
///         verification mechanism to check if possible
///         quotas are exceeded.

#[derive(Clone)]
enum TxLinkState{
    Unlinked,
    SingleLinked(TxId),
    MultiLinked(Vec<TxId>)
}

/// `TxLinkMapState` denotes the general link state of a
/// transaction. Depending on its value it can overwrite
/// the single link states. It can take the values:
///
/// * `Linkable`: The transaction can _generally_ be linked
///         to other transactions. This means that the linking
///         permit depends on the `TxLinkState` of the
///         relationship that should be established
/// * `Unlinkable`: The transaction can not be linked to any
///         other transaction (anymore). This state should
///         be used when a transaction type is deprecated
///         by a later system version or if the list of
///         (possible) links / relationships is empty
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
/// `TxLinkState`s and holds a `TxLinkMapState` to denote the general
/// link state of the transaction.

#[derive(Clone)]
pub struct TxLinkMap{
    state: TxLinkMapState,
    links: HashMap<TxRel, TxLinkState>
}
