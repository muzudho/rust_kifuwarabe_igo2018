/// ゾブリストハッシュのテスト☆（＾～＾）
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
/// cargo clippy --example zobrist
/// 
/// ### 実行。
/// cls
/// cargo run --example zobrist
/// ```

extern crate kifuwarabe_igo2018;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;

use kifuwarabe_igo2018::*;
use config_file::Config;
use liberty::*;
use position_file::PositionFile;
use position::Position;
use record::*;
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
    match fs::copy("position -- Example9.json", "position.json") {
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

    let mut record = Record::new();
    // 代入ではなく、コピーを作っている☆（*＾～＾*）
    let mut pos = Position::default(pos.board, 0, pos.turn);

    // 全部の交点に、連のIDを振る。
    check_liberty_all_points(&mut pos);

    // 試し打ち☆（＾～＾）
    do_move(convert_code_to_address(303, pos.board.get_size()), &mut pos, &mut record);
    // 盤面表示☆（＾～＾）
    show_board(&pos.board);

    // TODO アンドゥしたい☆（＾～＾）

    // 試し打ち☆（＾～＾）
    do_move(convert_code_to_address(603, pos.board.get_size()), &mut pos, &mut record);
    // 盤面表示☆（＾～＾）
    show_board(&pos.board);

    // 試し打ち☆（＾～＾）
    do_move(convert_code_to_address(306, pos.board.get_size()), &mut pos, &mut record);
    // 盤面表示☆（＾～＾）
    show_board(&pos.board);

    // 試し打ち☆（＾～＾）
    do_move(convert_code_to_address(606, pos.board.get_size()), &mut pos, &mut record);
    // 盤面表示☆（＾～＾）
    show_board(&pos.board);
}
