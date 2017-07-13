extern crate crypto;
extern crate rand;
use self::crypto::ed25519;
use self::rand::Rng;
use self::rand::OsRng;
use blockchain::utils::u64_to_u8le;
use blockchain::utils::sha3_256;
use blockchain::traits::Hashable;

pub struct BlockHeader{
    issuer_pubkey: [u8; 32],
    prev_block_hash: [u8; 32],
    body_type: u64,
    index: u64,
    timestamp: u64,
    content_hash: [u8; 32], 
    pub nonce: [u8; 32],
    signature: [u8; 64]
}


impl Hashable for BlockHeader {

    fn to_sha3_hash(&self) -> [u8; 32]{

        let bin_block = self.as_bytes();
        sha3_256(&bin_block)

    }

}

impl BlockHeader{

    /// Creates a new BlockHeader
    ///
    /// * `issuer_pubkey`: The public key of the issuer node
    /// * `previous_header`: Either None, if this is the first
    ///                      header in the chain or Some(BlockHeader)
    /// * `body_type`: Some constant to denote the transaction type in the
    ///                body (e.g. 'commodity transaction', 'key signature')
    /// * `content_hash`: The merkle tree root hash of all transactions
    ///                   in the appropriate BlockBody

    pub fn create(issuer_pubkey: [u8; 32],
                  previous_header: Option<BlockHeader>,
                  body_type: u64,
                  content_hash: [u8; 32]) -> BlockHeader {

        let mut index = 0;
        let mut prev_block_hash = [0;32];

        match previous_header {
            Some(block) => { index = block.index + 1;
                             prev_block_hash = block.to_sha3_hash(); }
            None => {}
        }

        BlockHeader {
            issuer_pubkey: issuer_pubkey,
            prev_block_hash: prev_block_hash,
            body_type: body_type,
            index: index,
            timestamp: 0, // TODO
            content_hash: content_hash,
            nonce: [0; 32],
            signature: [0; 64]
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

        let index_u8le = u64_to_u8le(self.index);
        let timestamp_u8le = u64_to_u8le(self.timestamp);
        let body_type_u8le = u64_to_u8le(self.body_type);

        let message = [&self.issuer_pubkey[..],
                       &self.prev_block_hash[..],
                       &body_type_u8le[..],
                       &index_u8le[..],
                       &timestamp_u8le[..],
                       &self.content_hash[..],
                       &self.nonce[..]].concat();

        message

    }

    /// Randomizes this BlockHeader's nonce

    pub fn randomize_nonce(& mut self){
        let mut rand_gen = OsRng::new().expect("Failed to fetch random number generator");
        rand_gen.fill_bytes(& mut self.nonce);
    }

    /// Returns true if the proof of work is valid
    /// (this is done by checking if the zero
    /// prefix is long enough)

    pub fn has_valid_pow(&self) -> bool{

        let blockhash = self.to_sha3_hash();

        // currently constant PoW

        blockhash[0] == 0 && blockhash[1] == 0
        
    }

    /// Returns true if BlockHeader signature
    /// is valid, false otherwise

    pub fn has_valid_signature(&self) -> bool{

        let message = self.message_as_bytes();
        ed25519::verify(&message, &self.issuer_pubkey, &self.signature)

    }

    /// Returns true if the BlockHeader is valid
    /// (= signature and proof of work are valid)

    pub fn is_valid(&self) -> bool{
        self.has_valid_signature() && self.has_valid_pow()
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

    let mut block = BlockHeader::create(public_key, None, 0xDEADBEEF, [4; 32]);

    // check for signature validity

    block.sign(&secret_key);
    assert!(block.has_valid_signature(), "Block was correctly signed, but sig check failed");

    block.sign(&[0;64]);
    assert!(!block.has_valid_signature(), "Block was incorrectly signed, but sig check passed");

}


#[test]
fn test_blockheader_pow_validity(){

    let public_key = [0xEE, 0xF9, 0x2A, 0x8F, 0xF3, 0xD0, 0x95, 0x1F,
                      0xE3, 0x49, 0x74, 0xDE, 0xA3, 0x03, 0xC6, 0x17,
                      0xCE, 0xA8, 0x8C, 0xF0, 0x70, 0x8F, 0x1D, 0xA3,
                      0x87, 0x04, 0x7A, 0x62, 0x04, 0xE9, 0x23, 0xF2];

    // create block header

    let mut block = BlockHeader::create(public_key, None, 0xDEADBEEF, [4; 32]);

    // check correct nonce (produces a hash with 16 leading zeroes)

    block.nonce = [0x4F, 0x43, 0x7B, 0x7F, 0x74, 0xB7, 0x6E, 0xCC,
                   0xEF, 0x06, 0xB7, 0xBA, 0xE4, 0x0A, 0x31, 0x12,
                   0xDA, 0x43, 0xDA, 0xF2, 0x74, 0xC8, 0x79, 0x2C,
                   0xD5, 0x5C, 0x46, 0x93, 0xCE, 0x39, 0x88, 0x17];

    assert!(block.has_valid_pow(), "Valid proof-of-work was not accepted");

    // check incorrect nonce

    block.nonce = [0; 32];

    assert!(!block.has_valid_pow(), "Invalid proof-of-work was accepted");

}
