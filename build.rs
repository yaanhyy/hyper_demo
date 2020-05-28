
use std::path::PathBuf;
use std::env;
fn main() {

    let base = env::var("CARGO_MANIFEST_DIR")
        .expect("should know manifest dir")
        .parse::<PathBuf>()
        .expect("should parse manifest dir as path")
        .join("test");
    println!("dir:{:?}", base);

    println!("build.rs");
    let file = std::fs::File::create("data.txt").expect("create failed");
    println!("文件创建成功:{:?}",file);
    let file = std::fs::File::create("data1.txt").expect("create failed");
    println!("文件创建成功:{:?}",file);
}