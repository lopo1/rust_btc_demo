use anyhow::Result;
use bincode;
use serde::{Deserialize,Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

use crate::errors::BlockchainError;

pub fn my_serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>, BlockchainError> 
    where T: Serialize,
{
    // let seialized = .unwrap();
    Ok(bincode::serialize(value).unwrap())
}

pub fn my_deserialize<'a,T>(bytes: &'a[u8]) -> Result<T, BlockchainError>
    where T: Deserialize<'a>
{
    let desrialized = bincode::deserialize(bytes).unwrap();
    Ok(desrialized)
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}

// for test
#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct Point {
    x: i32,
    y:i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_serialize,my_deserialize};
    #[test]
    fn coder_works() {
        let point = Point{x:1,y:1};
        if let Ok(se) = my_serialize(&point){
            // if let Ok(de: Point) = my_deserialize(&se){
            //     assert_eq!(de,point);
            // }
            
        }
        
        
       
    }
}