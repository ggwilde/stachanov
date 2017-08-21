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

use blockchain::utils::sha3_256;
use blockchain::traits::Hashable;

#[derive(Clone)]
pub struct BlockBody<T: Clone>{
    pub transactions: Vec<T>
}


impl<T: Hashable + Clone> BlockBody<T>{
    
    /// Creates a new BlockBody
    ///
    /// * `transactions`: A vec of transactions in the block body

    pub fn new(transactions: Vec<T>) -> BlockBody<T>{
        BlockBody{transactions: transactions}
    }

    /// Returns the merkle root hash over all transactions.
    /// (The root hash is saved in the block head to guarantee
    /// data integrity)

    pub fn merkle_root_hash(&self) -> [u8; 32]{
        
        // compute leaves

        let mut preimage: Vec<[u8; 32]> = Vec::new();

        for transaction in &self.transactions{

            let t_hash = transaction.to_sha3_hash();
            preimage.push(t_hash);

        }

        // we add a constant delimiter to allow secure zero-padding.
        // according to ISO/IEC 7816-4

        preimage.push([0x80, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0]);

        // pad the preimage size to a power of two to defend
        // against CVE-2012-2459

        let unpadded_len = preimage.len();
        let exp_f64 = (unpadded_len as f64).log(2.0);
        let exp_u32 = exp_f64.ceil() as u32;

        let mut diff = 2u32.pow(exp_u32) - unpadded_len as u32;
        while diff > 0{
            preimage.push([0; 32]);
            diff = diff - 1;
        }

        // combine pairs of two with a sha3 into
        // new hashes until only one hash remains

        while preimage.len() > 1{

            let mut temp: Vec<[u8; 32]> = Vec::new();
            let mut i = 0;

            while i < preimage.len(){

                let concatted = [&preimage[i][..], &preimage[i+1][..]].concat();
                let result = sha3_256(&concatted);
                temp.push(result);
                i += 2;

            }

            preimage = temp;
            
        }

        preimage[0]

    }   
    
}

#[test]
fn test_merkle_tree(){

    impl Hashable for u8{

        fn to_sha3_hash(&self) -> [u8; 32]{
            let as_array = [*self];
            sha3_256(&as_array[..])
        }

    }

    // try a transaction vector without subsequent padding

    let transactions = [0x14, 0x22, 0x41, 0xfb, 0xdf, 0x2a, 0x9b, 0xcf,
                        0x0a, 0xb2, 0x6a, 0xdb, 0xb4, 0x39, 0x44, 0x0f,
                        0x22, 0x49, 0xba, 0xda, 0x13, 0xff, 0xaf, 0x2a,
                        0x5f, 0x9a, 0x2a, 0xa9, 0xf5, 0x2c, 0x33].to_vec();

    let block_body: BlockBody<u8> = BlockBody{ transactions: transactions };

    let root_hash = block_body.merkle_root_hash();

    let assumed = [0x15, 0x6C, 0xEA, 0x94, 0xA2, 0xA2, 0x65, 0xEB,
                   0xD8, 0x43, 0x9D, 0xF6, 0x25, 0x5F, 0xFF, 0xEC,
                   0x8A, 0xAA, 0xED, 0x78, 0x79, 0x76, 0x61, 0x1D,
                   0xB3, 0xF7, 0x74, 0x5A, 0x74, 0x76, 0xE3, 0xCC];

    assert!(root_hash == assumed);

    // try a transaction vector with subsequent padding

    let transactions = [0x89, 0x2b, 0x4c, 0x8b, 0xd4,
                        0x17, 0x42, 0x2c, 0xaf, 0x59,
                        0x09, 0x7b, 0x37, 0xab, 0x8d,
                        0x69, 0xcd, 0xfe, 0x62, 0xe3,
                        0x32, 0x81, 0xfa, 0x27, 0x13,
                        0x21, 0x7d, 0xfc, 0x2f, 0x06,
                        0x64, 0x1d, 0x0a, 0x0f, 0x2a,
                        0x08, 0x24, 0x43, 0xc4, 0xde].to_vec();

    let block_body: BlockBody<u8> = BlockBody{ transactions: transactions };

    let root_hash = block_body.merkle_root_hash();

    let assumed = [0x08, 0xAA, 0x7C, 0xD4, 0xA4, 0xA5, 0x75, 0x76,
                   0x59, 0xFD, 0x21, 0x7A, 0xE2, 0x15, 0xD9, 0xFA,
                   0x29, 0x72, 0x45, 0x13, 0xA5, 0xCD, 0xD1, 0xD8,
                   0x44, 0xE2, 0x55, 0xD0, 0x87, 0x7E, 0x03, 0x9A];

    assert!(root_hash == assumed);

}
