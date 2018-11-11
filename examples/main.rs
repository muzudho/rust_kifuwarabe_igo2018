/// コンフィグファイルを読み取る例☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cargo clippy --example main
/// 
/// ### 実行。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo run --example main
/// ```

extern crate kifuwarabe_igo2018;

// ランダムムーブ
extern crate rand;
use rand::Rng;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;

use std::path::Path;

use std::thread;
use std::time::Duration;

use kifuwarabe_igo2018::*;
use kifuwarabe_igo2018::config_file::Config;
use kifuwarabe_igo2018::position_file::PositionFile;
use kifuwarabe_igo2018::position::Position;
use kifuwarabe_igo2018::liberty::*;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    println!("Config comment: '{}'.", conf.comment);
    println!("Config conf_board_size: {}.", conf.board_size);


    loop {
        if Path::new("position.json").exists() {

            // 局面ファイル確認。
            let pos = PositionFile::load("position.json");
            println!("Pos comment: '{}'.", pos.comment);
            println!("ply: '{}'.", pos.ply);
            println!("Turn: '{}'.", pos.turn);
            // 盤面表示☆（＾～＾）
            show_board(conf.board_size, pos.board);

            // 読み取ったらファイル削除。
            fs::remove_file("position.json");


            // 代入ではなく、コピーを作っている☆（*＾～＾*）
            let mut pos = Position::default(pos.ply, pos.turn, pos.board);

            // 盤番地を表示☆（＾～＾）
            show_board_address(conf.board_size);

            // 盤を表示☆（＾～＾）
            show_board_by_number(conf.board_size, pos.board);

            // 全部の交点に、連のIDを振る。
            let mut ren_id_board = [0; 21 * 21];
            let mut liberty_count_map = [0; 21*21];
            check_liberty_all_points(conf.board_size, pos.board, &mut ren_id_board, &mut liberty_count_map);

            // 連のIDを表示☆（＾～＾）
            show_ren_id_board(conf.board_size, ren_id_board);

            // 呼吸点の数を表示☆（＾～＾）
            show_libarty_count(liberty_count_map);

            // 試し打ちをする☆（＾～＾）
            //
            // 例えば 3x3 の盤の中段右は x=3, y=2 と数えて、
            //
            // +++++
            // +   +
            // +  *+
            // +   +
            // +++++
            //
            // 符号は 302、番地は 5 とする。
            // 符号は人間が読み書きする用なので 入出力ファイルでのみ使用し、プログラム中では 番地 のみ使う。
            /*
            println!("Conv {} -> {}", 704, convert_code_to_address(704, board_size));
            println!("Conv {} -> {}", 101, convert_code_to_address(101, board_size));
            println!("Conv {} -> {}", 102, convert_code_to_address(102, board_size));
            println!("Conv {} -> {}", 908, convert_code_to_address(908, board_size));
            println!("Conv {} -> {}", 909, convert_code_to_address(909, board_size));
             */
            let ko = 0;
            let forbidden = is_forbidden(convert_code_to_address(704, conf.board_size), pos.turn, conf.board_size, pos.board, ren_id_board, liberty_count_map, ko);
            println!("forbidden? {}", forbidden);
            let forbidden = is_forbidden(convert_code_to_address(401, conf.board_size), pos.turn, conf.board_size, pos.board, ren_id_board, liberty_count_map, ko);
            println!("forbidden? {}", forbidden);

        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}
