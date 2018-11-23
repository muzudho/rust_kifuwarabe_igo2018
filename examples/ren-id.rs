/// 連IDのテスト☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### 環境のアップデート
/// cargo update
/// 
/// ### コンパイル。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy --example ren-id
/// 
/// ### 実行。
/// cls
/// cargo run --example ren-id
/// ```

extern crate kifuwarabe_igo2018;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;
use std::path::Path;

use kifuwarabe_igo2018::*;
use kifuwarabe_igo2018::config_file::Config;
use kifuwarabe_igo2018::position_file::PositionFile;
use kifuwarabe_igo2018::position::Position;
use kifuwarabe_igo2018::liberty::*;


fn main() {
    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("config -- Example9.json", "config.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 設定ファイル読込。
    let conf = Config::load("config.json");

    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("position -- Test9Ren.json", "position.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 局面ファイル確認。
    let pos = PositionFile::load(conf.board_size, "position.json");
    // 読み取ったらファイル削除。

    match fs::remove_file("position.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    println!("Pos comment: '{}'.", pos.comment);
    println!("ply: '{}'.", pos.ply);
    println!("Turn: '{}'.", pos.turn);
    // 盤面表示☆（＾～＾）
    show_board(&pos.board);

    // 代入ではなく、コピーを作っている☆（*＾～＾*）
    let mut pos = Position::default(pos.board, 0, pos.turn, pos.ply);

    // 盤番地を表示☆（＾～＾）
    show_board_address(conf.board_size);

    // 盤を表示☆（＾～＾）
    show_board_by_number(&pos.board);

    // 全部の交点に、連のIDを振る。
    check_liberty_all_points(conf.board_size, &pos.board, &mut pos.ren_id_board, &mut pos.liberty_count_map, &mut pos.ren_element_map);

    // 連のIDを表示☆（＾～＾）
    show_ren_id_board(conf.board_size, &pos.ren_id_board);

    // 呼吸点の数を表示☆（＾～＾）
    show_libarty_count(&pos.liberty_count_map);
}
