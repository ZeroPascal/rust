//Run this with 'cargo run --bin hash'
use std::path::PathBuf;
use blake2::{Blake2s256, Digest};
use file_hashing::get_hash_file;
use directories::UserDirs;

fn main() {
    let start_time = std::time::Instant::now();
    println!("{:#?}",start_time);
    let dir = UserDirs::new();
    println!("{:?}",dir);
    let path = PathBuf::from("./Frida.mov");

    let mut hash = Blake2s256::new();
    let result = get_hash_file(&path, &mut hash).unwrap();
    println!("{}",result);
    let delta_time = start_time.elapsed();
    println!("Delta: {:#?}",delta_time);
}