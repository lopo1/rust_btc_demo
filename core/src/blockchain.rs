use crate::block;
use tracing::info;

const CURR_BITS: usize = 8;
pub struct BlockChain {
    pub blocks: Vec<block::Block>,
    pub height: usize,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() -1];
        let new_block = block::Block::new_block(data,pre_block.hash.clone(),CURR_BITS);
        self.blocks.push(new_block);
        self.height +=1;
    }

    fn new_genesis_block() -> block::Block{
        block::Block::new_block("This is genesis block".to_string(), String::from(""),CURR_BITS)
    }

    pub fn new_blockchain() -> BlockChain {
        BlockChain { 
            blocks: vec![BlockChain::new_genesis_block()],
            height: 0,
        }
    }

    pub fn blocks_info(&self) {
        for block in self.blocks.iter() {
            info!("{:#?}", block);
            // println!("{:#?}",block);
        }
    }
}