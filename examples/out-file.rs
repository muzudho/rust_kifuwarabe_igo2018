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
use out_file::OutFile;
use view::*;

fn main() {
    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("out -- Example19.txt", "out.txt") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 局面ファイル確認。
    let (pos, pre_move) = OutFile::load_out(19, "out.txt");

    // 盤面表示☆（＾～＾）
    show_board(&pos.get_board());

    println!("Ko: '{}'.", pos.ko);
    println!("Turn: '{}'.", pos.turn);
    println!("Pre move: '{}'.", pre_move);
    println!("Finished.");
}
