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

use address_ren_board_searcher::*;
use kifuwarabe_igo2018::*;
use config_file::Config;
use position_file::PositionFile;
use position::Position;
use ren_database::*;
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

    let mut address_ren_board_searcher = AddressRenBoardSearcher::new();
    // 代入ではなく、コピーを作っている☆（*＾～＾*）
    let mut pos = Position::default(pos.board, 0, pos.turn);

    // 盤番地を表示☆（＾～＾）
    show_board_address(conf.board_size);

    // 全部の交点に、連のIDを振る。
    check_liberty_all_points(&mut pos);

    // 連のIDを表示☆（＾～＾）
    show_address_ren_board(&pos);

    // 呼吸点の数を表示☆（＾～＾）
    show_libarty_count(&pos.get_ren_database().get_ren_mappings());

    // 空連の占有者を表示☆（＾～＾）
    show_territory(&pos.get_ren_database().get_ren_mappings());
    show_ren_address_map(&pos.get_ren_database().get_ren_mappings());

    {
        // 空連12 の上の 0102 点に石を置く☆（＾～＾）
        let ren_id = 12;
        let stone_addr = convert_code_to_address(102, pos.get_board().get_size());
        println!("board size: {}.", pos.get_board().get_size());
        println!("stone_addr: {}.", stone_addr);
        pos.get_mut_ren_database().get_mut_ren_mappings().remove_addr(ren_id, stone_addr as i16);

        address_ren_board_searcher.count_up_mark();
        let mut shrink: Vec<i16> = Vec::new();

        {
            // 空連12 の 0102 点から上を探索。
            let start = pos.get_board().get_top_of(stone_addr);
            let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), ren_id, start, stone_addr);
            print!("空連{} の {:04} 点から上を探索。", ren_id, convert_address_to_code(start, pos.get_board().get_size()));
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&address_ren_board_searcher.get_found_addr());
            }
        }

        {
            // 空連12 の 0102 点から右を探索。
            let start = pos.get_board().get_right_of(stone_addr);
            let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), ren_id, start, stone_addr);
            print!("空連{} の {:04} 点から右を探索。", ren_id, convert_address_to_code(start, pos.get_board().get_size()));        
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&address_ren_board_searcher.get_found_addr());
            }
        }

        {
            // 空連12 の 0102 点から下を探索。
            let start = pos.get_board().get_bottom_of(stone_addr);
            let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), ren_id, start, stone_addr);
            print!("空連{} の {:04} 点から下を探索。", ren_id, convert_address_to_code(start, pos.get_board().get_size()));        
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&address_ren_board_searcher.get_found_addr());
            }
        }

        {
            // 空連12 の 0102 点から左を探索。
            let start = pos.get_board().get_left_of(stone_addr);
            let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), ren_id, start, stone_addr);
            print!("空連{} の {:04} 点から左を探索。", ren_id, convert_address_to_code(start, pos.get_board().get_size()));        
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&address_ren_board_searcher.get_found_addr());
            }
        }

        if !shrink.is_empty() {
            let old_territory = match pos.get_ren_database().get_empty_ren_map().get_ren(ren_id) {
                Some(ren_obj) => ren_obj.get_territory(),
                None => {println!("テスト テリトリーの取得失敗。連ID: {}.", ren_id); 0},
            };

            pos.get_mut_ren_database().get_mut_empty_ren_map().remove_ren(ren_id);
            pos.get_mut_ren_database().get_mut_empty_ren_map().insert_ren(ren_id, PieceObject::default(ren_id, shrink, old_territory));

            print!("縮まった空連の作り直し。番地: ");
            match &pos.get_ren_database().get_empty_ren_map().get_ren(ren_id) {
                Some(ren_obj) => show_ren_addr(ren_obj),
                None => {},
            };
            println!(".");
        }
    }
}
