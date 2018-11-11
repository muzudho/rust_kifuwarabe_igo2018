/// コンフィグファイルを読み取る例☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### 実行。
/// cls
/// cd  C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo run --example conf
/// ```

extern crate kifuwarabe_igo2018;

// mod config_file;
// use config_file::Config;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    println!("Config comment: '{}'.", conf.comment);
    println!("Config conf_board_size: {}.", conf.board_size);
}