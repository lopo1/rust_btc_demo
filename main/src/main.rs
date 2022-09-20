use core::blockchain;
fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();
    bc.add_block("a -> b: 5 btc".to_string());
    bc.add_block("c -> d: 1 btc".to_string());
    bc.blocks_info();
    // for b in bc.blocks{
    //     println!("+++++++++++++++++++++++++");
    //     println!("{:#?}",b);
    //     println!("");
    // }
    println!("Hello, world!");
}
