use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize,Serialize};
use crate::pow::ProofOfWork;

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq,Clone)]
pub struct BlockHeader{
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
    bits: usize,
    pub nonce: usize,
}

impl BlockHeader {
    fn new(pre_hash: String,tx_hash:String,bits:usize) -> Self {
        Self {
            time: Utc::now().timestamp(),
            tx_hash: tx_hash,
            pre_hash: pre_hash,
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String, // transactions data
}

impl Block {
    pub fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header)).unwrap();
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String,pre_hash: String,bits:usize) -> Block{
        let transactions = coder::my_serialize(&data).unwrap();
        let tx_hash = coder::get_hash(&transactions[..]);
        let mut block = Block {
            header: BlockHeader::new(pre_hash, tx_hash, bits),
            hash: "".to_string(),
            data: data,
        };
        block.set_hash();

        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);
        block
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }

    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }


}
