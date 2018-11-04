/// 参考:
/// https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル
/// cd  C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy
/// 
/// ### 実行
/// cargo run --release
/// ```
/// 
fn main() {
    // 設定ファイル読込。
    let mut file = match File::open("config.json") {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // https://docs.serde.rs/serde_json/value/enum.Value.html
    let v: Value = match serde_json::from_str(&contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // エントリー・ポイント取得。
    let comment = v["comment"].as_str().unwrap().to_string();

    println!("Hello, world! comment: '{}'.", comment);
}
