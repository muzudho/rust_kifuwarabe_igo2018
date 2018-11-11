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

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;
use std::path::Path;
// use std::collections::HashMap;

use kifuwarabe_igo2018::*;
use kifuwarabe_igo2018::config_file::Config;
use kifuwarabe_igo2018::position_file::PositionFile;
use kifuwarabe_igo2018::position::Position;
use kifuwarabe_igo2018::liberty::*;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("position -- Test9Utiage.json", "position.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    if Path::new("position.json").exists() {

        // 局面ファイル読込。
        let pos = PositionFile::load("position.json");
        // 読み取ったらファイル削除。

        match fs::remove_file("position.json") {
            Ok(_o) => {}
            Err(e) => {panic!(e)}
        };

        // 盤面表示☆（＾～＾）
        show_board(conf.board_size, &pos.board);

        // 代入ではなく、コピーを作っている☆（*＾～＾*）
        let mut pos = Position::default(pos.ply, pos.turn, pos.board);

        // 盤番地を表示☆（＾～＾）
        show_board_address(conf.board_size);

        // 全部の交点に、連のIDを振る。
        check_liberty_all_points(conf.board_size, &pos.board, &mut pos.ren_id_board, &mut pos.liberty_count_map, &mut pos.ren_element_map);
        // 連のIDを表示☆（＾～＾）
        show_ren_id_board(conf.board_size, &pos.ren_id_board);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(pos.liberty_count_map);
        // 連の要素を表示☆（＾～＾）
        show_ren_element_map(&pos.ren_element_map);

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(102, conf.board_size), 1, conf.board_size, &mut pos.board, &mut pos.ren_id_board, &mut pos.ren_element_map);

        // 盤面表示☆（＾～＾）
        show_board(conf.board_size, &pos.board);
        // 連のIDを表示☆（＾～＾）
        show_ren_id_board(conf.board_size, &pos.ren_id_board);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(pos.liberty_count_map);
        // 連の要素を表示☆（＾～＾）
        show_ren_element_map(&pos.ren_element_map);
    }

    println!("Finished.");
}
