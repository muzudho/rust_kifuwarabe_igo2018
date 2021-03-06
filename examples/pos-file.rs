/// 局面ファイル読取テスト☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cargo clippy --example pos-file
/// 
/// ### 実行。
/// cls
/// cd C:\muzudho\projects_rust\rust-kifuwarabe-igo2018
/// cargo run --example pos-file
/// ```

extern crate kifuwarabe_igo2018;

use std::fs;
use kifuwarabe_igo2018::*;
use config_file::Config;
use position_file::PositionFile;
use view::*;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("position -- Test9Ren.json", "position.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 局面ファイル確認。
    let pos = PositionFile::load(conf.board_size, "position.json");
    println!("Comment: '{}'.", pos.comment);
    println!("Ply: '{}'.", pos.ply);
    println!("Turn: '{}'.", pos.turn);
    // 盤面表示☆（＾～＾）
    show_board(&pos.board);

    println!("Finished.");
}
