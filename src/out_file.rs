/// 大会用 out.txt 局面ファイル☆（＾～＾）

use std::fs::File;
use std::io::Read;
use board::Board;

pub struct OutFile {
    /// 盤面。
    pub board: Board,
    /// 黒のアゲハマ。
    pub black_age_hama: i16,
    /// 白のアゲハマ。
    pub white_age_hama: i16,
    /// 黒の累計思考時間（秒）。
    pub black_seconds: i32,
    /// 白の累計思考時間（秒）。
    pub white_seconds: i32,
    /// コウの番地。'XXYY'書式の数。無ければ 0。
    pub ko: i16,
    /// 手番の石の色☆（＾～＾） 1:黒, 2:白。
    pub turn: i8,
}
impl OutFile {
    pub fn load(path:&str) -> OutFile {
        let mut file = match File::open(path) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(n) => n,
            Err(err) => panic!("File open error. {:?}", err),
        };
        // '\r'があったら削除☆（＾～＾）
        contents.retain(|c| c != '\r');

        // '\n'で分割☆（＾～＾）
        let lines: Vec<&str> = contents.split('\n').collect();

        // 盤面作成☆（＾～＾）
        let mut temp_board = Board::new();
        let mut temp_black_age_hama = 0;
        let mut temp_white_age_hama = 0;
        let mut temp_black_seconds = 0;
        let mut temp_white_seconds = 0;
        let mut temp_ko = 0;
        let mut temp_turn = 0;
        let mut cell = 0;
        for (row, line) in lines.iter().enumerate() {
            // 0行～20行まで盤上☆（＾～＾）
            if row < 21 {
                let numbers: Vec<i8> = line.split(',').filter_map(|k| k.parse().ok()).collect::<Vec<i8>>();
                for stone in numbers {
                    temp_board.set(cell, stone);
                    cell += 1;
                }
            } else if row == 21 {
                // 最終行。
                let numbers: Vec<i32> = line.split(',').filter_map(|k| k.parse().ok()).collect::<Vec<i32>>();
                temp_black_age_hama = numbers[0];
                temp_white_age_hama = numbers[1];
                temp_black_seconds = numbers[2];
                temp_white_seconds = numbers[3];
                temp_ko = numbers[4];
                temp_turn = numbers[5];
            }
            // 以降の行は無視。
        }

        OutFile {
            board: temp_board,
            black_age_hama: temp_black_age_hama as i16,
            white_age_hama: temp_white_age_hama as i16,
            black_seconds: temp_black_seconds,
            white_seconds: temp_white_seconds,
            ko: temp_ko as i16,
            turn: temp_turn as i8,
        }
    }
}