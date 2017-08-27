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

use blockchain::header::BlockHeader;
use blockchain::body::BlockBody;
use blockchain::transactions::Transaction;
use blockchain::transactions::TxIndex;
use blockchain::errors::VerificationError;
use blockchain::errors::VerificationErrorReason::InvalidContentHash;
use blockchain::traits::Hashable;

/// `BlockId` is equivalent to the sha3 hash of the block header

#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct BlockId(pub [u8; 32]);

#[derive(Clone)]
pub struct Block{
    header: BlockHeader,
    body: BlockBody<Transaction>
}

impl Block{

    /// Creates a new Block
    ///
    /// * `issuer_pubkey`: The public key of the issuer node
    /// * `previous_block`: Either None, if this is the first
    ///                     header in the chain or Some(Block)
    /// * `timestamp`: u64 unix timetamp denoting the mining
    ///                start point
    /// * `transactions`: A vec of transactions in the block

    pub fn new(issuer_pubkey: [u8; 32],
               previous_block: Option<&Block>,
               timestamp: u64,
               transactions: Vec<Transaction>) -> Block{

        let body = BlockBody::new(transactions);
        let content_hash = body.merkle_root_hash();

        let previous_header = match previous_block{
            Some(block) => Some(&block.header),
            None => None
        };

        let version = 0;

        let header = BlockHeader::new(issuer_pubkey,
                                      previous_header,
                                      timestamp,
                                      version,
                                      content_hash);

        Block{header: header, body: body}

    }

    /// Gets the unique hash identifier
    /// of this block

    pub fn get_id(&self) -> BlockId{
        let header_hash = self.header.to_sha3_hash();
        BlockId(header_hash)
    }

    /// Gets the index of the block

    pub fn get_index(&self) -> u64{
        self.header.get_index()
    }

    /// Gets the timestamp of the block

    pub fn get_timestamp(&self) -> u64{
        self.header.get_timestamp()
    }

    /// Gets a single transaction from the block.
    /// * `index`: The TxIndex of the transaction

    pub fn get_transaction(&self, tx_index: TxIndex) -> Option<Transaction> {
        let TxIndex(index) = tx_index;
        self.body.get_transaction(index as usize)
    }

    /// Verifies the internal consistency of the block.
    /// This includes:
    /// * Verification of issuer signature
    /// * Verification of proof of work
    /// * Verification of merkle hash tree

    fn verify_internal(&self) -> Result<(), VerificationError> {

        self.header.verify_internal()?;

        if self.header.content_hash != self.body.merkle_root_hash(){
            let err = VerificationError::new(InvalidContentHash);
            return Err(err)
        }
        Ok(())

    }

    /// Verifies that this block is the successor
    /// of the supplied block
    /// * `prev_block`: The preceding block

    pub fn verify_chain_link(&self, prev_block: &Block) -> Result<(), VerificationError> {

        self.header.verify_chain_link(&prev_block.header)?;
        Ok(())

    }

    /// Sets a nonce to the block header
    /// * `nonce`: 32 bytes of u8 integers

    pub fn set_nonce(&mut self, nonce: [u8; 32]){
        self.header.set_nonce(nonce);
    }

    /// Signs the Block
    ///
    /// * `secret_key`: The ed25519 secret key
    ///                 (non-detached as 64 byte array)

    pub fn sign(&mut self, secret_key: &[u8]){
        self.header.sign(secret_key);
    }

}

#[test]
fn test_verify_internal(){

    let secret_key = [0x0E, 0x51, 0x0D, 0x71, 0x3A, 0x7E, 0x08, 0x01,
                      0x3C, 0xA8, 0x1A, 0x3F, 0x79, 0x24, 0x54, 0x60,
                      0x31, 0x29, 0xAA, 0x25, 0x01, 0x30, 0x4A, 0xE0,
                      0xF9, 0xFA, 0x61, 0xA8, 0x54, 0x4E, 0xC5, 0x11,
                      0xE3, 0x70, 0x07, 0x9D, 0x71, 0xD0, 0x59, 0x6F,
                      0xE6, 0x48, 0x71, 0x85, 0x2A, 0x8E, 0xF0, 0x0C,
                      0x75, 0xDD, 0x13, 0x79, 0xFD, 0x87, 0xCF, 0xBB,
                      0x5B, 0xB7, 0x72, 0xBE, 0x90, 0xC6, 0x1E, 0xD3];

    let public_key = [0xE3, 0x70, 0x07, 0x9D, 0x71, 0xD0, 0x59, 0x6F,
                      0xE6, 0x48, 0x71, 0x85, 0x2A, 0x8E, 0xF0, 0x0C,
                      0x75, 0xDD, 0x13, 0x79, 0xFD, 0x87, 0xCF, 0xBB,
                      0x5B, 0xB7, 0x72, 0xBE, 0x90, 0xC6, 0x1E, 0xD3];


    // First subtest
    // -------------
    // Create a block with a dummy transaction. Dummy transactions have
    // an all zero hash, which means the merkle tree root will be
    // sha3_256(0....0 + 0x80...0)

    let mut block = Block::new(public_key, None, 0, vec![Transaction::Dummy]);

    let nonce = [0x2C, 0x0E, 0x2F, 0x75, 0xD0, 0x7C, 0xB7, 0x80,
                 0x3A, 0x4A, 0xC2, 0xB8, 0xF5, 0xB6, 0x10, 0x21,
                 0x01, 0x7E, 0x0B, 0xF1, 0xAF, 0x9A, 0xCA, 0xA5,
                 0xBD, 0x1E, 0xC7, 0xB7, 0xAF, 0x37, 0xC3, 0x30];

    block.set_nonce(nonce);
    block.sign(&secret_key);

    assert!(block.verify_internal().is_ok(), "Block was not classified as valid even though signature,
                                              proof of work and content hash are correct");

    // Second subtest
    // --------------
    // create a block with a correct proof of work and
    // signature, but without a correct content_hash

    let wrong_content_hash = [4; 32];
    let mut block_header = BlockHeader::new(public_key, None, 0, 0, wrong_content_hash);

    let body = BlockBody{transactions: vec![Transaction::Dummy]};
    let mut block = Block{header: block_header, body: body};

    let nonce = [0xFB, 0x7F, 0xB9, 0x56, 0x69, 0x8F, 0x21, 0x41,
                 0x6B, 0xAE, 0x23, 0x76, 0x8A, 0x3D, 0x37, 0xC5,
                 0xD1, 0x32, 0x09, 0xA0, 0xF7, 0x94, 0x6C, 0x90,
                 0x37, 0x24, 0x95, 0x82, 0xB0, 0x9A, 0xED, 0x73];

    block.set_nonce(nonce);
    block.sign(&secret_key);

    // make sure that our assumptions about
    // proof of work and signature hold true

    assert!(block.header.verify_pow().is_ok());
    assert!(block.header.verify_signature().is_ok());
    assert!(block.header.verify_internal().is_ok());

    // check if it still fails

    assert!(block.verify_internal().is_err(), "Wrong content hash, but block was
                                               classified as valid");

}

#[test]
fn test_chain_link_validation(){

    let public_key = [0xE3, 0x70, 0x07, 0x9D, 0x71, 0xD0, 0x59, 0x6F,
                      0xE6, 0x48, 0x71, 0x85, 0x2A, 0x8E, 0xF0, 0x0C,
                      0x75, 0xDD, 0x13, 0x79, 0xFD, 0x87, 0xCF, 0xBB,
                      0x5B, 0xB7, 0x72, 0xBE, 0x90, 0xC6, 0x1E, 0xD3];

    // create two subsequent blocks

    let first_block = Block::new(public_key, None, 0, vec![Transaction::Dummy]);
    let second_block = Block::new(public_key, Some(&first_block), 1, vec![Transaction::Dummy]);

    assert!(second_block.verify_chain_link(&first_block).is_ok(), "Correctly linked blocks classified as invalid");
    assert!(first_block.verify_chain_link(&second_block).is_err(), "Incorrectly linked blocks classified as valid");

    // create two subsequent blocks where the second
    // bock has a timestamp <= the timestamp of the
    // preceding one

    let first_block = Block::new(public_key, None, 0, vec![Transaction::Dummy]);
    let second_block = Block::new(public_key, Some(&first_block), 0, vec![Transaction::Dummy]);

    assert!(second_block.verify_chain_link(&first_block).is_err(), "Incorrectly timestamped block pair classified as valid");

}
