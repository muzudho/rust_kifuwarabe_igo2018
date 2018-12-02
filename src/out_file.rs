/// 大会用 out.txt 局面ファイル☆（＾～＾）

use std::fs::File;
use std::io::Read;
use std::{thread, time};

use *;
use board::Board;
use position::Position;

pub struct OutFile {

}
impl OutFile {

    /// # Returns.
    /// - 局面。
    /// - 相手が打った場所の符号。
    pub fn load_out(board_size:usize, path:&str) -> (Position, i16) {

        // out.txt ファイルを読取に行く。別のプロセスが使用していて、エラーになることがよくある。
        let mut file;
        loop {
            match File::open(path) {
                Ok(n) => {file = n; break;},
                Err(err) => {
                    println!("File open error. {:?}", err);
                    // 0.3秒ぐらい待機してから繰り返し。
                    thread::sleep(time::Duration::from_millis(300));
                }
            };
        }

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
        let mut temp_board = Board::default(board_size);
        // 黒のアゲハマ。
        let mut _temp_black_age_hama = 0;
        // 白のアゲハマ。
        let mut _temp_white_age_hama = 0;
        // 黒の累計思考時間（秒）。
        let mut _temp_black_seconds = 0;
        // 白の累計思考時間（秒）。
        let mut _temp_white_seconds = 0;
        // コウの場所の符号。
        let mut temp_ko_code = 0;
        let mut temp_turn = 0;
        // 相手が打った場所の符号。
        let mut temp_pre_move = 0;

        let mut cell = 0;
        for (row, line) in lines.iter().enumerate() {
            // 0行～20行まで盤上☆（＾～＾）
            if row < 21 {
                let numbers: Vec<i8> = line.split(',').filter_map(|k| k.parse().ok()).collect::<Vec<i8>>();
                for stone in numbers {
                    temp_board.set_stone(cell, stone);
                    cell += 1;
                }
            } else if row == 21 {
                // 最終行。
                let numbers: Vec<i32> = line.split(',').filter_map(|k| k.parse().ok()).collect::<Vec<i32>>();
                _temp_black_age_hama = numbers[0];
                _temp_white_age_hama = numbers[1];
                _temp_black_seconds = numbers[2];
                _temp_white_seconds = numbers[3];
                temp_ko_code = numbers[4] as i16;
                temp_turn = numbers[5];
                temp_pre_move = numbers[6] as i16;
            }
            // 以降の行は無視。
        }

        (Position::default(temp_board, convert_code_to_address(temp_ko_code, board_size) as i16, temp_turn as i8), temp_pre_move)
    }
}