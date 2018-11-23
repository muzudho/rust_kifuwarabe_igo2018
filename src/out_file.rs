/// 大会用 out.txt 局面ファイル☆（＾～＾）

use std::fs::File;
use std::io::Read;
use board::Board;
use position::Position;

impl Position {
    pub fn load_out(board_size:usize, path:&str) -> Position {
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
        let mut temp_board = Board::default(board_size);
        // 黒のアゲハマ。
        let mut _temp_black_age_hama = 0;
        // 白のアゲハマ。
        let mut _temp_white_age_hama = 0;
        // 黒の累計思考時間（秒）。
        let mut _temp_black_seconds = 0;
        // 白の累計思考時間（秒）。
        let mut _temp_white_seconds = 0;
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
                _temp_black_age_hama = numbers[0];
                _temp_white_age_hama = numbers[1];
                _temp_black_seconds = numbers[2];
                _temp_white_seconds = numbers[3];
                temp_ko = numbers[4];
                temp_turn = numbers[5];
            }
            // 以降の行は無視。
        }

        Position::default(temp_board, temp_ko as usize, temp_turn as i8, 0)
    }
}