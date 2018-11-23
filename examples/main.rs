/// 大会参加用☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy --example main
/// 
/// ### 実行。
/// cls
/// cargo run --example main
/// ```

extern crate kifuwarabe_igo2018;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use kifuwarabe_igo2018::*;
use config_file::Config;
use out_file::OutFile;
use position::Position;
use liberty::*;
use best_move::BestMove;
use view::*;

fn main() {
    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("config -- Air2018.json", "config.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 設定ファイル読込。
    let conf = Config::load("config.json");

    loop {
        // 局面ファイルの有無確認。
        if Path::new(&conf.out_path).exists() {

            // 局面ファイル確認。
            let (mut pos, pre_move) = OutFile::load_out(conf.board_size, &conf.out_path);

            // 読み取ったらファイル削除。
            match fs::remove_file(&conf.out_path) {
                Ok(_o) => {}
                Err(e) => {panic!(e)}
            };

            // 盤面表示☆（＾～＾）
            show_board(&pos.board);
            println!("Turn: '{}'.", pos.turn);
            println!("ply: '{}'.", pos.ply);
            println!("Pre move: '{}'.", pre_move);

            // 全部の交点に、連のIDを振る。
            check_liberty_all_points(&mut pos);

            // 試し打ちをする☆（＾～＾）
            let legal_moves = pick_move(&pos);
            let move_code = convert_address_to_code(do_random_move(&mut pos, &legal_moves), pos.board.get_size());
            println!("BestMove: '{}'.", move_code);

            // in.txt ファイル出力。
            BestMove::save(&conf.in_path, move_code as usize);
        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}
