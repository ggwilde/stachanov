extern crate crypto;
use self::crypto::sha3::Sha3;
use self::crypto::digest::Digest;

pub fn u64_to_u8le(input: u64) -> [u8; 8]{

    let mut output = [0;8];
    for i in 0..8{
        output[i] = (input >> i*8) as u8
    }
    output

}

pub fn sha3_256(input: &[u8]) -> [u8; 32]{

    let mut output = [0;32];
    let mut hasher = Sha3::sha3_256();
    hasher.input(&input);
    hasher.result(& mut output);

    output

}
