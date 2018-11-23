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
use position::Position;

fn main() {
    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("out -- Example19.txt", "out.txt") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 局面ファイル確認。
    let pos = Position::load_out(19, "out.txt");

    // 盤面表示☆（＾～＾）
    show_board(&pos.board);

    println!("ko: '{}'.", pos.ko);
    println!("turn: '{}'.", pos.turn);
    println!("Finished.");
}
