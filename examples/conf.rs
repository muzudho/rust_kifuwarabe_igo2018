/// 設定ファイル読取テスト☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cargo clippy --example conf
/// 
/// ### 実行。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo run --example conf
/// ```

extern crate kifuwarabe_igo2018;

use kifuwarabe_igo2018::config_file::Config;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    println!("Comment: '{}'.", conf.comment);
    println!("Board size: {}.", conf.board_size);

    println!("Finished.");
}
