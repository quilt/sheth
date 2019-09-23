use arrayref::array_ref;
use sheth::process_data_blob;
use std::fs;
use std::io::{self, Read};

pub fn main() {
    let mut input = fs::read_to_string("blob").expect("File to exist");

    let args: Vec<&str> = input.split_whitespace().collect();
    let pre_state_root = hex::decode(args[0]).unwrap();
    let post_state_root = hex::decode(args[2]).unwrap();
    let mut input = hex::decode(args[1]).unwrap();

    println!("input len {}", input.len());

    // Process input data
    let post_root = process_data_blob(&mut input, array_ref![pre_state_root, 0, 32]);

    println!("post_state_root => {:?}", hex::encode(&post_root));
    println!("expected => {:?}", hex::encode(post_state_root));
}
