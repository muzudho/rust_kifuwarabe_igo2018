/// 局面ファイル☆（＾～＾）

extern crate serde_json;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

pub struct Position {
    /// コメント。
    pub comment: String,
    /// 何手目か。
    pub ply: i64,
    /// 手番の石の色☆（＾～＾） 1:黒, 2:白。
    pub turn: i8,
    /// 19路盤枠ありが入るサイズを確保しておく。使ってない数字で埋める☆（＾～＾）
    pub board: [i8; 21 * 21],
}
impl Position {
    pub fn load(path:&str) -> Position {
        let mut file = match File::open(path) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let document: Value = match serde_json::from_str(&contents) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };


        // 盤面作成。
        let mut temp_board = [0; 21 * 21];
        let mut i = 0;
        for line in document["board"].as_array().unwrap().iter() {
            let chars = line.as_str().unwrap().chars().collect::<Vec<char>>();
            for ch in &chars {
                temp_board[i] = match ch {
                    'x' => 1, // 黒。
                    'o' => 2, // 白。
                    '+' => 3, // 枠。
                    _ => 0,   // スペース。
                };
                i += 1;
            }
            // println!("Line: '{}'.", line);
        }

        Position {
            comment: document["comment"].as_str().unwrap().to_string(),
            ply: document["ply"].as_i64().unwrap(),
            turn: match document["turn"].as_str().unwrap() {
                "black" => 1,
                "white" => 2,
                _ => panic!("Undefined turn: [{}].", document["turn"].as_str().unwrap())
            },
            board: temp_board,
        }
    }
}