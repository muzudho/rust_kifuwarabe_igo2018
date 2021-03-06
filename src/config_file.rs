/// 設定ファイル

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

pub struct Config {
    /// コメント。
    pub comment: String,

    /// 何路盤。
    pub board_size: usize,

    /// 大会用 out.txt 局面ファイルへのパス。
    pub out_path: String,

    /// 大会用 in.txt 局面ファイルへのパス。
    pub in_path: String,
}
impl Config {
    pub fn load(path:&str) -> Config {

        let mut file = match File::open(path) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        // https://docs.serde.rs/serde_json/value/enum.Value.html
        let document: Value = match serde_json::from_str(&contents) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        Config {
            comment: match document["comment"].as_str() {
                Some(n) => n.to_string(),
                None => "".to_string(),
            },

            board_size: match document["boardSize"].as_i64() {
                Some(n) => n as usize, // FIXME 変換方法が分からん☆（＾～＾）as使う☆（＾～＾）
                None => 19usize,
            },

            out_path: match document["outFile"].as_str() {
                Some(n) => n.to_string(),
                None => "".to_string(),
            },

            in_path: match document["inFile"].as_str() {
                Some(n) => n.to_string(),
                None => "".to_string(),
            },
        }
    }
}
