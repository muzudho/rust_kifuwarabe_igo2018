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
/// cargo clippy --example ren-board-search
/// 
/// ### 実行。
/// cls
/// cargo run --example ren-board-search
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

    // 全部の交点に、連のIDを振る。
    check_liberty_all_points(&mut pos);

    // 連のIDを表示☆（＾～＾）
    show_address_ren_board(&pos);

    // 呼吸点の数を表示☆（＾～＾）
    show_libarty_count(&pos.liberty_count_map);

    // 空連の占有者を表示☆（＾～＾）
    show_empty_owner(&pos.empty_owner_map);
    show_ren_address_map(&pos.empty_owner_map.space);

    {
        // 空連12 の 0102 点から探索。
        let ren_id = 12;
        let mark = convert_code_to_address(102, pos.board.get_size());
        println!("board size: {}.", pos.board.get_size());
        println!("mark: {}.", mark);

        {
            // 空連12 の 0102 点から上を探索。
            let start = pos.board.get_top_of(mark);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, ren_id, start, mark);
            println!("空連{} の {:04} 点から上を探索。 min_addr: {}.", ren_id, convert_address_to_code(start, pos.board.get_size()), min_addr);
        }

        {
            // 空連12 の 0102 点から右を探索。
            let start = pos.board.get_right_of(mark);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, ren_id, start, mark);
            println!("空連{} の {:04} 点から右を探索。 min_addr: {}.", ren_id, convert_address_to_code(start, pos.board.get_size()), min_addr);        
        }

        {
            // 空連12 の 0102 点から下を探索。
            let start = pos.board.get_bottom_of(mark);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, ren_id, start, mark);
            println!("空連{} の {:04} 点から下を探索。 min_addr: {}.", ren_id, convert_address_to_code(start, pos.board.get_size()), min_addr);        
        }

        {
            // 空連12 の 0102 点から左を探索。
            let start = pos.board.get_left_of(mark);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, ren_id, start, mark);
            println!("空連{} の {:04} 点から左を探索。 min_addr: {}.", ren_id, convert_address_to_code(start, pos.board.get_size()), min_addr);        
        }
    }
}
