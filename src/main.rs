/// 参考:
/// https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use serde_json::Value;

use std::fs;
use std::fs::File;
use std::io::Read;

use std::path::Path;

use std::thread;
use std::time::Duration;

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
    let mut conf_file = match File::open("config.json") {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    let mut conf_contents = String::new();
    match conf_file.read_to_string(&mut conf_contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // https://docs.serde.rs/serde_json/value/enum.Value.html
    let conf_v: Value = match serde_json::from_str(&conf_contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // コメント取得。
    let conf_comment = conf_v["comment"].as_str().unwrap().to_string();
    println!("Config comment: '{}'.", conf_comment);

    loop {
        // ファイル確認。
        // println!("{}", Path::new("position.json").exists());
        if Path::new("position.json").exists() {
            // 指し手ファイル読込。
            let mut pos_file = match File::open("position.json") {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

            let mut pos_contents = String::new();
            match pos_file.read_to_string(&mut pos_contents) {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

            let pos_v: Value = match serde_json::from_str(&pos_contents) {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

            // コメント取得。
            let pos_comment = pos_v["comment"].as_str().unwrap().to_string();
            println!("Pos comment: '{}'.", pos_comment);

            // 盤面取得。
            for line in pos_v["board"].as_array().unwrap().iter() {
                println!("Line: '{}'.", line);
            }

            let turn = pos_v["turn"].as_str().unwrap().to_string();
            println!("Turn: '{}'.", turn);

            // 読み取ったらファイル削除。
            fs::remove_file("position.json");
        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）

}
