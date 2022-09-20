use core::blockchain;
fn main() {
    // let mut bc = blockchain::BlockChain::new_blockchain();
    // bc.add_block("a -> b: 5 btc".to_string());
    // bc.add_block("c -> d: 1 btc".to_string());
    // bc.blocks_info();


    tracing_subscriber::fmt().init();

    let mut bc = blockchain::BlockChain::new_blockchain();

    bc.add_block("Justin -> Bob 2 btc".to_string());
    // bc.mine_block("Justin -> Bruce 2 btc");

    bc.blocks_info();
    // for b in bc.blocks{
    //     println!("+++++++++++++++++++++++++");
    //     println!("{:#?}",b);
    //     println!("");
    // }
    // println!("Hello, world!");
}
