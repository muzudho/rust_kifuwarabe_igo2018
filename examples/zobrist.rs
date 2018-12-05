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
use ren_db::address_ren_board_searcher::*;
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

    let mut address_ren_board_searcher = AddressRenBoardSearcher::new();
    let mut record = Record::new();
    // 代入ではなく、コピーを作っている☆（*＾～＾*）
    let mut pos = Position::default(pos.board, 0, pos.turn);

    // 全部の交点に、連のIDを振る。
    check_liberty_all_points(&mut pos);

    {
        // 試し打ち☆（＾～＾）
        do_move(convert_code_to_address(303, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());

        // アンドゥ☆（＾～＾）
        undo_move(&mut pos, &mut record);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
    }

    {
        // 試し打ち☆（＾～＾）
        do_move(convert_code_to_address(603, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());

        // 試し打ち☆（＾～＾）
        do_move(convert_code_to_address(306, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());

        // 試し打ち☆（＾～＾）
        do_move(convert_code_to_address(606, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());

        // アンドゥ☆（＾～＾）
        undo_move(&mut pos, &mut record);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());

        // アンドゥ☆（＾～＾）
        undo_move(&mut pos, &mut record);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());

        // アンドゥ☆（＾～＾）
        undo_move(&mut pos, &mut record);
        // 盤面表示☆（＾～＾）
        show_board(&pos.get_board());
    }

    println!("３コウのテスト。");
    {
        // +++++++++
        // +       +
        // +       +
        // +       +
        // +       +
        // +++++++++

        do_move(convert_code_to_address(103, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // +       +
        // +       +
        // +x      +
        // +       +
        // +++++++++

        do_move(convert_code_to_address(102, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // +       +
        // +o      +
        // +x      +
        // +       +
        // +++++++++

        do_move(convert_code_to_address(204, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // +       +
        // +o      +
        // +x      +
        // + x     +
        // +++++++++

        do_move(convert_code_to_address(201, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o     +
        // +o      +
        // +x      +
        // + x     +
        // +++++++++

        do_move(convert_code_to_address(303, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o     +
        // +o      +
        // +x x    +
        // + x     +
        // +++++++++

        do_move(convert_code_to_address(302, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o     +
        // +o o    +
        // +x x    +
        // + x     +
        // +++++++++

        do_move(convert_code_to_address(404, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o     +
        // +o o    +
        // +x x    +
        // + x x   +
        // +++++++++

        do_move(convert_code_to_address(401, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o   +
        // +o o    +
        // +x x    +
        // + x x   +
        // +++++++++

        do_move(convert_code_to_address(503, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o   +
        // +o o    +
        // +x x x  +
        // + x x   +
        // +++++++++

        do_move(convert_code_to_address(502, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o   +
        // +o o o  +
        // +x x x  +
        // + x x   +
        // +++++++++

        do_move(convert_code_to_address(604, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o   +
        // +o o o  +
        // +x x x  +
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(601, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o o o  +
        // +x x x  +
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(703, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o o o  +
        // +x x x x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(702, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o o o o+
        // +x x x x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(202, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +oxo o o+
        // +x x x x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(403, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +oxo o o+
        // +x xox x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(602, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +oxo oxo+
        // +x xox x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(203, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o o oxo+
        // +xoxox x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(402, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o oxoxo+
        // +xox x x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(603, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o oxo o+
        // +xox xox+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(202, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +oxoxo o+
        // +x x xox+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(403, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +oxo o o+
        // +x xoxox+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(602, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +oxo oxo+
        // +x xox x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(203, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o o oxo+
        // +xoxox x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(402, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o oxoxo+
        // +xox x x+
        // + x x x +
        // +++++++++

        do_move(convert_code_to_address(603, pos.get_board().get_size()), &mut pos, &mut record, &mut address_ren_board_searcher);
        show_board(&pos.get_board());
        show_super_ko(&record);
        // +++++++++
        // + o o o +
        // +o oxo o+
        // +xox xox+
        // + x x x +
        // +++++++++
    }
}
