/// コンフィグファイルを読み取る例☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy --example test
/// 
/// ### 実行。
/// cls
/// cargo run --example test
/// ```

extern crate kifuwarabe_igo2018;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use kifuwarabe_igo2018::*;
use config_file::Config;
use liberty::*;
use position_file::PositionFile;
use position::Position;
use record::*;
use view::*;

fn main() {
    // 設定ファイル読込。
    let conf = Config::load("config.json");

    loop {
        if Path::new("position.json").exists() {

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
            // let mut record = Record::new();

            // 表示☆（＾～＾）

            // 表示      : 盤番地を表示☆（＾～＾）
            show_board_address(conf.board_size);

            // 表示      : 盤を表示☆（＾～＾）
            show_board_by_number(&pos.get_board());

            // データ更新: 全部の交点に、連のIDを振る。
            check_liberty_all_points(&mut pos);

            // 表示      : 連のIDを表示☆（＾～＾）
            show_piece_distribution(&pos);

            // 表示      : 呼吸点の数を表示☆（＾～＾）
            show_libarty_count(&pos.get_piece_database().get_piece_mappings());

            // 表示      : 空連の占有者を表示☆（＾～＾）
            show_territory(&pos.get_piece_database().get_piece_mappings());
            show_ren_address_map(&pos.get_piece_database().get_piece_mappings());

            // TODO 連のIDに紐づく石の番地を表示☆（＾～＾）

            // 試し打ちをする☆（＾～＾）
            //
            // 例えば 3x3 の盤の中段右は x=3, y=2 と数えて、
            //
            // +++++
            // +   +
            // +  *+
            // +   +
            // +++++
            //
            // 符号は 302、番地は 5 とする。
            // 符号は人間が読み書きする用なので 入出力ファイルでのみ使用し、プログラム中では 番地 のみ使う。
            /*
            println!("Conv {} -> {}", 704, convert_code_to_address(704, board_size));
            println!("Conv {} -> {}", 101, convert_code_to_address(101, board_size));
            println!("Conv {} -> {}", 102, convert_code_to_address(102, board_size));
            println!("Conv {} -> {}", 908, convert_code_to_address(908, board_size));
            println!("Conv {} -> {}", 909, convert_code_to_address(909, board_size));
             */
            let forbidden = is_forbidden(convert_code_to_address(704, conf.board_size), &pos, &record);
            println!("forbidden? {}", forbidden);
            let forbidden = is_forbidden(convert_code_to_address(401, conf.board_size), &pos, &record);
            println!("forbidden? {}", forbidden);

        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}
