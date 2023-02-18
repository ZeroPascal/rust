//Run This with 'cargo run --bin command_line'
use std::process::Command;
fn main(){
    let cmd = "-ss 00:00:00.00 -i './Loop.mp4' -frames:v 1 zero.jpg";// "./ffmpeg -ss 00:00:00.00 -i './Loop.mp4' -frames:v 1 zero.jpg";
    let output = if cfg!(target_os = "windows") {
        Command::new("D:/Git Repos/rust/ffmpeg")
                .args([cmd])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    //stdout is a Bitsteam, need to decode it with from_utf8
    let hello = std::str::from_utf8(&output.stdout);
    println!("{:?}",hello);
    
}