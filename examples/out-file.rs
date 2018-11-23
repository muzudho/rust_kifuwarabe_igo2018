/// 大会用 out.txt 局面ファイル読取テスト☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy --example out-file
/// 
/// ### 実行。
/// cls
/// cargo run --example out-file
/// ```

extern crate kifuwarabe_igo2018;

use std::fs;
use kifuwarabe_igo2018::*;
use kifuwarabe_igo2018::out_file::OutFile;

fn main() {
    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("out -- Example19.txt", "out.txt") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 局面ファイル確認。
    let pos = OutFile::load("out.txt");

    // 盤面表示☆（＾～＾）
    show_board(19, &pos.board);

    println!("blackAgeHama: '{}'.", pos.black_age_hama);
    println!("whiteAgeHama: '{}'.", pos.white_age_hama);
    println!("blackSeconds: '{}'.", pos.black_seconds);
    println!("whiteSeconds: '{}'.", pos.white_seconds);
    println!("ko: '{}'.", pos.ko);
    println!("turn: '{}'.", pos.turn);
    println!("Finished.");
}
