/// 「ウチアゲ」のテストだぜ☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cargo clippy --example utiage
/// 
/// ### 実行。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo run --example utiage
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

    // ファイルをコピーするぜ☆（＾～＾）
    fs::copy("position -- Test9Utiage.json", "position.json");

    if Path::new("position.json").exists() {

        // 局面ファイル読込。
        let pos = PositionFile::load("position.json");
        // 読み取ったらファイル削除。
        fs::remove_file("position.json");

        // 盤面表示☆（＾～＾）
        show_board(conf.board_size, pos.board);
        // 代入ではなく、コピーを作っている☆（*＾～＾*）
        let mut pos = Position::default(pos.ply, pos.turn, pos.board);

        // 全部の交点に、連のIDを振る。
        let mut ren_id_board = [0; 21 * 21];
        let mut liberty_count_map = [0; 21*21];
        check_liberty_all_points(conf.board_size, pos.board, &mut ren_id_board, &mut liberty_count_map);
        // 連のIDを表示☆（＾～＾）
        show_ren_id_board(conf.board_size, ren_id_board);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(liberty_count_map);
        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(102, conf.board_size), 1, conf.board_size, &mut pos.board);

        // 盤面表示☆（＾～＾）
        show_board(conf.board_size, pos.board);
        // 連のIDを表示☆（＾～＾）
        show_ren_id_board(conf.board_size, ren_id_board);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(liberty_count_map);
    }

    println!("Finished.");
}
