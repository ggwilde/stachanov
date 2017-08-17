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

extern crate crypto;
extern crate rand;
use self::crypto::ed25519;
use self::rand::Rng;
use self::rand::OsRng;
use blockchain::utils::u64_to_u8le;
use blockchain::utils::u8le_to_u64;
use blockchain::utils::sha3_256;
use blockchain::traits::Hashable;
use blockchain::errors::VerificationError;
use blockchain::errors::VerificationErrorReason::InvalidProofOfWork;
use blockchain::errors::VerificationErrorReason::InvalidIssuerSignature;
use blockchain::errors::VerificationErrorReason::InvalidChainLink;

pub struct BlockHeader{
    version: u64,
    issuer_pubkey: [u8; 32],
    prev_block_hash: [u8; 32],
    index: u64,
    timestamp: u64,
    pub content_hash: [u8; 32],
    pub nonce: [u8; 32],
    signature: [u8; 64]
}


impl Hashable for BlockHeader {

    fn to_sha3_hash(&self) -> [u8; 32]{

        let bin_block = self.message_as_bytes();
        sha3_256(&bin_block)

    }

}

impl BlockHeader{

    /// Creates a new BlockHeader
    ///
    /// * `issuer_pubkey`: The public key of the issuer node
    /// * `previous_header`: Either None, if this is the first
    ///                      header in the chain or Some(BlockHeader)
    /// * `timestamp`: u64 unix timetamp denoting the mining
    ///                start point
    /// * `version`: A version number, that signifies which rules apply
    ///              to the block
    /// * `content_hash`: The merkle tree root hash of all transactions
    ///                   in the appropriate BlockBody

    pub fn new(issuer_pubkey: [u8; 32],
               previous_header: Option<BlockHeader>,
               timestamp: u64,
               version: u64,
               content_hash: [u8; 32]) -> BlockHeader {

        let mut index = 0;
        let mut prev_block_hash = [0;32];

        match previous_header {
            Some(block) => { index = block.index + 1;
                             prev_block_hash = block.to_sha3_hash(); }
            None => {}
        }

        BlockHeader {
            version: version,
            issuer_pubkey: issuer_pubkey,
            prev_block_hash: prev_block_hash,
            index: index,
            timestamp: timestamp,
            content_hash: content_hash,
            nonce: [0; 32],
            signature: [0; 64]
        }

    }

    /// Creates a new BlockHeader from a byte vector.
    ///
    /// * `bytes`: A byte vector

    pub fn from_bytes(bytes: Vec<u8>) -> BlockHeader{


        let mut version_u8le = [0; 8];

        let mut i = 0;
        for byte in bytes[0..8].to_vec(){
            version_u8le[i] = byte;
            i = i + 1;
        }

        let version = u8le_to_u64(version_u8le);

        // --

        let mut issuer_pubkey = [0; 32];

        let mut i = 0;
        for byte in bytes[8..40].to_vec(){
            issuer_pubkey[i] = byte;
            i = i + 1;
        }

        // --

        let mut prev_block_hash = [0; 32];

        let mut i = 0;
        for byte in bytes[40..72].to_vec(){
            prev_block_hash[i] = byte;
            i = i + 1;
        }

        // --

        let mut index_u8le = [0; 8];

        let mut i = 0;
        for byte in bytes[72..80].to_vec(){
            index_u8le[i] = byte;
            i = i + 1;
        }

        let index = u8le_to_u64(index_u8le);

        // --

        let mut timestamp_u8le = [0; 8];

        let mut i = 0;
        for byte in bytes[80..88].to_vec(){
            timestamp_u8le[i] = byte;
            i = i + 1;
        }

        let timestamp = u8le_to_u64(timestamp_u8le);

        // --

        let mut content_hash = [0; 32];

        let mut i = 0;
        for byte in bytes[88..120].to_vec(){
            content_hash[i] = byte;
            i = i + 1;
        }

        // --

        let mut nonce = [0; 32];

        let mut i = 0;
        for byte in bytes[120..152].to_vec(){
            nonce[i] = byte;
            i = i + 1;
        }

        // --

        let mut signature = [0; 64];

        let mut i = 0;
        for byte in bytes[152..216].to_vec(){
            signature[i] = byte;
            i = i + 1;
        }

        BlockHeader {
            version: version,
            issuer_pubkey: issuer_pubkey,
            prev_block_hash: prev_block_hash,
            index: index,
            timestamp: timestamp,
            content_hash: content_hash,
            nonce: nonce,
            signature: signature
        }

    }

    /// Returns the complete BlockHeader (including signature)
    /// as an u8 vector

    pub fn as_bytes(&self) -> Vec<u8>{

        let bin_message = self.message_as_bytes();
        let as_vec = [&bin_message[..], &self.signature[..]].concat();
        as_vec

    }

    /// Returns the message segment of the BlockHeader
    /// as u8 vector. The message segment is all data
    /// without the trailing signature

    fn message_as_bytes(&self) -> Vec<u8>{

        let version_u8le = u64_to_u8le(self.version);
        let index_u8le = u64_to_u8le(self.index);
        let timestamp_u8le = u64_to_u8le(self.timestamp);

        let message = [&version_u8le[..],
                       &self.issuer_pubkey[..],
                       &self.prev_block_hash[..],
                       &index_u8le[..],
                       &timestamp_u8le[..],
                       &self.content_hash[..],
                       &self.nonce[..]].concat();

        message

    }

    /// Sets a nonce to the block header
    /// * `nonce`: 32 bytes of u8 integers

    pub fn set_nonce(&mut self, nonce: [u8; 32]){
        self.nonce = nonce;
    }

    /// Randomizes this BlockHeader's nonce

    pub fn randomize_nonce(& mut self){
        let mut rand_gen = OsRng::new().expect("Failed to fetch random number generator");
        rand_gen.fill_bytes(& mut self.nonce);
    }

    /// Verifies the internal structure of the header
    /// This includes:
    /// * Verification of issuer signature
    /// * Verification of proof of work

    pub fn verify_internal(&self) -> Result<(), VerificationError>{
        self.verify_signature()?;
        self.verify_pow()?;
        Ok(())
    }

    /// Verifies the issuer signature

    pub fn verify_signature(&self) -> Result<(), VerificationError>{

        let message = self.message_as_bytes();
        let issuer_verified = ed25519::verify(&message, &self.issuer_pubkey, &self.signature);

        if !issuer_verified{
            let err = VerificationError::new(InvalidIssuerSignature);
            return Err(err)
        }
        Ok(())

    }

    /// Verifies the proof of work

    pub fn verify_pow(&self) -> Result<(), VerificationError>{

        let blockhash = self.to_sha3_hash();

        // currently constant difficulty

        if !(blockhash[0] == 0 && blockhash[1] == 0){
            let err = VerificationError::new(InvalidProofOfWork);
            return Err(err)
        }
        Ok(())

    }

    /// Verifies that this block header is the successor
    /// of the supplied block header
    /// * `prev_header`: The preceding block header

    pub fn verify_chain_link(&self, prev_header: &BlockHeader) -> Result<(), VerificationError> {

        if (self.prev_block_hash != prev_header.to_sha3_hash() ||
            self.timestamp <= prev_header.timestamp ||
            self.index != prev_header.index + 1)
        {
            let err = VerificationError::new(InvalidChainLink);
            return Err(err)
        }
        Ok(())

    }

    /// Signs the BlockHeader
    ///
    /// * `secret_key`: The ed25519 secret key
    ///                 (non-detached as 64 byte array)

    pub fn sign(& mut self, secret_key: &[u8]){

        let message = self.message_as_bytes();
        self.signature = ed25519::signature(&message, secret_key);

    }

}


#[test]
fn test_blockheader_signature_validity(){

    // generate random ed25519 keypair

    let mut seed = [0;32];
    let mut rand_gen = OsRng::new().expect("Failed to fetch random number generator");
    rand_gen.fill_bytes(& mut seed);

    let (secret_key, public_key) = ed25519::keypair(&seed);

    // create block header

    let mut block = BlockHeader::new(public_key, None, 0, 0xDEADBEEF, [4; 32]);

    // check for signature validity

    block.sign(&secret_key);
    assert!(block.verify_signature().is_ok(), "Block was correctly signed, but sig check failed");

    block.sign(&[0;64]);
    assert!(block.verify_signature().is_err(), "Block was incorrectly signed, but sig check passed");

}


#[test]
fn test_blockheader_pow_validity(){

    let public_key = [0xF6, 0x78, 0x27, 0x40, 0xFE, 0xAC, 0xCB, 0x89,
                      0x2E, 0x7E, 0x17, 0xEC, 0x3E, 0x4F, 0x3C, 0xF4,
                      0x49, 0x90, 0x58, 0x66, 0x15, 0xC5, 0x3C, 0x54,
                      0xC7, 0x8C, 0x5A, 0x43, 0x7B, 0x54, 0x5F, 0x3D];

    // create block header

    let mut block = BlockHeader::new(public_key, None, 0, 0, [4; 32]);

    // check correct nonce (produces a hash with 16 leading zeroes)

    let nonce = [0x43, 0x2A, 0x50, 0x50, 0x22, 0x8A, 0xDE, 0xC3,
                 0x0C, 0x36, 0xC0, 0x69, 0x79, 0x03, 0x0A, 0x14,
                 0x6E, 0x76, 0xF5, 0x71, 0x73, 0xF5, 0xFE, 0x88,
                 0x9A, 0xCA, 0x5E, 0x5B, 0x04, 0x10, 0xA8, 0x6F];

    block.set_nonce(nonce);

    assert!(block.verify_pow().is_ok(), "Valid proof-of-work was not accepted");

    // check incorrect nonce

    block.set_nonce([0; 32]);

    assert!(block.verify_pow().is_err(), "Invalid proof-of-work was accepted");

}

#[test]
fn test_to_bytes_from_bytes(){

    // use a unique pattern to miniminize the risk for parsing errors

    let block_header = BlockHeader {

        version: u8le_to_u64([0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47]),

        issuer_pubkey:
            [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
             0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
             0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],

        prev_block_hash:
            [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
             0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
             0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
             0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f],

        index: u8le_to_u64([0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f]),
        timestamp: u8le_to_u64([0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57]),

        content_hash:
            [0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
             0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
             0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f,
             0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77],

        nonce:
            [0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f,
             0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
             0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
             0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97],

        signature:
            [0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
             0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
             0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
             0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,

             0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
             0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
             0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
             0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7]
    };

    let as_bytes = block_header.as_bytes();
    let rebuild = BlockHeader::from_bytes(as_bytes);

    assert!(block_header.issuer_pubkey == rebuild.issuer_pubkey);
    assert!(block_header.prev_block_hash == rebuild.prev_block_hash);
    assert!(block_header.version == rebuild.version);
    assert!(block_header.index == rebuild.index);
    assert!(block_header.timestamp == rebuild.timestamp);
    assert!(block_header.content_hash == rebuild.content_hash);
    assert!(block_header.nonce == rebuild.nonce);

    // array PartialEq seems to be only implemented for array with len <= 32

    let mut i = 0;
    while i < 64{
        assert!(block_header.signature[i] == rebuild.signature[i]);
        i = i + 1;
    }


}
