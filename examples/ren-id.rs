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

use kifuwarabe_igo2018::*;
use config_file::Config;
use position_file::PositionFile;
use position::Position;
use liberty::*;
use view::*;

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
    let mut pos = Position::default(pos.board, 0, pos.turn);

    // 盤番地を表示☆（＾～＾）
    show_board_address(conf.board_size);

    // 盤を表示☆（＾～＾）
    show_board_by_number(&pos.board);

    // 全部の交点に、連のIDを振る。
    check_liberty_all_points(&mut pos);

    // 連のIDを表示☆（＾～＾）
    show_address_ren_board(&pos);

    // 呼吸点の数を表示☆（＾～＾）
    show_libarty_count(&pos.liberty_count_map);

    // 空連の占有者を表示☆（＾～＾）
    show_empty_ren_territory(&pos.get_territory());
    show_ren_address_map(&pos.get_territory().space);

    // 目つぶしの確認☆（＾～＾）
    println!("eye_fill: 0401x {}", pos.get_territory().is_eye_filling(1, convert_code_to_address(401, pos.board.get_size()) as i16));
    println!("eye_fill: 0704o {}", pos.get_territory().is_eye_filling(2, convert_code_to_address(704, pos.board.get_size()) as i16));
    println!("eye_fill: 0404x {}", pos.get_territory().is_eye_filling(1, convert_code_to_address(404, pos.board.get_size()) as i16));
    println!("eye_fill: 0909o {}", pos.get_territory().is_eye_filling(2, convert_code_to_address(909, pos.board.get_size()) as i16));
}
