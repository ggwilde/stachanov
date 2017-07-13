extern crate rand;
extern crate crypto;
mod blockchain;
use self::rand::Rng;
use self::rand::OsRng;
use crypto::ed25519;
use blockchain::traits::Hashable;
use blockchain::header::BlockHeader;

fn main(){


    let mut seed = [0;32];
    let mut rand_gen = OsRng::new().expect("Failed to fetch random number generator");
    rand_gen.fill_bytes(& mut seed);


    let secpub_tuple = ed25519::keypair(&seed);
    let secret_key = secpub_tuple.0;
    let public_key = secpub_tuple.1;

    let mut block = BlockHeader::create(public_key, None, 0xDEADBEEF, [4; 32]);

    let mut n = 0;
    while !block.has_valid_pow() {
        n = n + 1;
        print!("{}: ", n);
        block.randomize_nonce();
        let bhash = block.to_sha3_hash();
        for i in 0..32 {
            print!("{:02X}", bhash[i]);
        }
        println!("");
    }

    for i in 0..32 {
        print!("{:02X}", block.nonce[i]);
    }
    println!("");

    print!("secret key: ");
    for i in 0..64 {
        print!("{:02X}", secret_key[i]);
    }
    println!("");

    print!("public_key key: ");
    for i in 0..32 {
        print!("{:02X}", public_key[i]);
    }

    println!("");
    block.sign(&secret_key);
    assert!(block.has_valid_signature());

}
