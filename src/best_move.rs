/// 大会用 in.txt 局面ファイル☆（＾～＾）

use std::fs;
use std::io::{BufWriter, Write};

pub struct BestMove {

}
impl BestMove {
    pub fn new() -> BestMove {
        BestMove {

        }
    }

    /// ファイル書きだし。
    /// 
    /// # Arguments.
    /// * `path` - 出力先ファイルパス。
    /// * `move_code` - 指し手の符号。XXYY書式の数字列。
    pub fn save(path:&String, move_code:usize) {
        let mut f = BufWriter::new(fs::File::create(path).unwrap());
        f.write_all(move_code.to_string().as_bytes()).unwrap();
    }
}
