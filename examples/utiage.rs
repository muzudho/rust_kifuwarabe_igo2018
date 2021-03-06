/// 「ウチアゲ」のテストだぜ☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cargo clippy --example utiage
/// 
/// ### 実行。
/// cls
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo run --example utiage
/// ```

extern crate kifuwarabe_igo2018;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;
use std::path::Path;
// use std::collections::HashMap;

use kifuwarabe_igo2018::*;
use ren_db::piece_distribution_searcher::*;
use config_file::Config;
use liberty::*;
use position_file::PositionFile;
use position::Position;
use record::*;
use view::*;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("position -- Test9Utiage.json", "position.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    if Path::new("position.json").exists() {

        // 局面ファイル読込。
        let pos = PositionFile::load(conf.board_size, "position.json");
        // 読み取ったらファイル削除。

        match fs::remove_file("position.json") {
            Ok(_o) => {}
            Err(e) => {panic!(e)}
        };

        // 盤面表示☆（＾～＾）
        show_board(&pos.board);

        let mut piece_distribution_searcher = PieceDistributionSearcher::new();
        // 代入ではなく、コピーを作っている☆（*＾～＾*）
        let mut pos = Position::default(pos.board, 0, pos.turn);
        let mut record = Record::new();

        // 盤番地を表示☆（＾～＾）
        show_board_address(conf.board_size);

        // 全部の交点に、連のIDを振る。
        check_liberty_all_points(&mut pos);
        // 連のIDを表示☆（＾～＾）
        show_piece_distribution(&pos);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(&pos.get_piece_database().get_piece_mappings());
        // 空連の占有者を表示☆（＾～＾）
        show_territory(&pos.get_piece_database().get_piece_mappings());
        // 連の要素を表示☆（＾～＾）
        show_ren_address_map(&pos.get_piece_database().get_stone_ren_map());

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(102, conf.board_size), &mut pos, &mut record, &mut piece_distribution_searcher);

        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
        // 連のIDを表示☆（＾～＾）
        show_piece_distribution(&pos);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(&pos.get_piece_database().get_piece_mappings());
        // 空連の占有者を表示☆（＾～＾）
        show_territory(&pos.get_piece_database().get_piece_mappings());
        // 連の要素を表示☆（＾～＾）
        show_ren_address_map(&pos.get_piece_database().get_piece_mappings());

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(401, conf.board_size), &mut pos, &mut record, &mut piece_distribution_searcher);

        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
        // 連のIDを表示☆（＾～＾）
        show_piece_distribution(&pos);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(&pos.get_piece_database().get_piece_mappings());
        // 空連の占有者を表示☆（＾～＾）
        show_territory(&pos.get_piece_database().get_piece_mappings());
        // 連の要素を表示☆（＾～＾）
        show_ren_address_map(&pos.get_piece_database().get_piece_mappings());

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(901, conf.board_size), &mut pos, &mut record, &mut piece_distribution_searcher);

        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
        // 連のIDを表示☆（＾～＾）
        show_piece_distribution(&pos);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(&pos.get_piece_database().get_piece_mappings());
        // 空連の占有者を表示☆（＾～＾）
        show_territory(&pos.get_piece_database().get_piece_mappings());
        // 連の要素を表示☆（＾～＾）
        show_ren_address_map(&pos.get_piece_database().get_piece_mappings());

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(109, conf.board_size), &mut pos, &mut record, &mut piece_distribution_searcher);

        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
        // 連のIDを表示☆（＾～＾）
        show_piece_distribution(&pos);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(&pos.get_piece_database().get_piece_mappings());
        // 空連の占有者を表示☆（＾～＾）
        show_territory(&pos.get_piece_database().get_piece_mappings());
        // 連の要素を表示☆（＾～＾）
        show_ren_address_map(&pos.get_piece_database().get_piece_mappings());

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(409, conf.board_size), &mut pos, &mut record, &mut piece_distribution_searcher);

        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
        // 連のIDを表示☆（＾～＾）
        show_piece_distribution(&pos);
        // 呼吸点の数を表示☆（＾～＾）
        show_libarty_count(&pos.get_piece_database().get_piece_mappings());
        // 空連の占有者を表示☆（＾～＾）
        show_territory(&pos.get_piece_database().get_piece_mappings());
        // 連の要素を表示☆（＾～＾）
        show_ren_address_map(&pos.get_piece_database().get_piece_mappings());

        // 試し打ちをする☆（＾～＾）
        do_move(convert_code_to_address(909, conf.board_size), &mut pos, &mut record, &mut piece_distribution_searcher);
    }

    println!("Finished.");
}
