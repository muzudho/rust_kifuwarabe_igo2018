/// 大会参加用☆（＾～＾）
/// 
/// # 実行方法
/// [Windows]+[R], "cmd",
/// 
/// ```
/// ### コンパイル。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy --example main
/// 
/// ### 実行。
/// cls
/// cargo run --example main
/// ```

extern crate kifuwarabe_igo2018;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use air2018::*;
use ren_db::piece_distribution_searcher::*;
use kifuwarabe_igo2018::*;
use config_file::Config;
use out_file::OutFile;
use record::Record;
use liberty::*;
use best_move::BestMove;
use view::*;

fn main() {
    // ファイルをコピーするぜ☆（＾～＾）
    match fs::copy("config -- Air2018.json", "config.json") {
        Ok(_o) => {}
        Err(e) => {panic!(e)}
    };

    // 設定ファイル読込。
    let conf = Config::load("config.json");
    println!("Load: config.json");

    // 計算用。
    let mut piece_distribution_searcher = PieceDistributionSearcher::new();

    // 棋譜のクリアー。
    let mut record = Record::new();

    println!("CgfGoban(きふわらべ改造版)で対局を開始してください。 out.txt を読みに行きます。");

    loop {
        // 局面ファイルの有無確認。
        if Path::new(&conf.out_path).exists() {

            // 局面ファイル確認。
            let (mut pos, pre_move) = OutFile::load_out(conf.board_size, &conf.out_path);

            // 読み取ったらファイル削除。
            match fs::remove_file(&conf.out_path) {
                Ok(_o) => {}
                Err(e) => {panic!(e)}
            };

            // 表示: 盤面表示☆（＾～＾）
            show_board(&pos.get_board());
            println!("Turn: '{}'.", pos.turn);

            // 相手の指した手を棋譜に入れる。
            println!("Pre move: '{}'.", pre_move);
            record.count_up();
            record.set_current(convert_code_to_address(pre_move, pos.get_board().get_size()) as i16, pos.get_board().get_hash());
            // TODO 打ち上げた番地が分からん☆（＾～＾）アンドゥで困る☆（＾～＾）
            println!("Record size: '{}'.", record.len());

            // 全部の交点に、連のIDを振る。
            check_liberty_all_points(&mut pos);

            // 着手できる交点を取り出すぜ☆
            // TODO let legal_moves = pick_move_normal(&pos, &record);
            
            let mut air2018 = Air2018::new();
            let mut legal_moves = air2018.pick_move_air2018(&pos, &record);
            // 候補が無ければ、ランダム打ち。
            if legal_moves.is_empty() {
                legal_moves = pick_move_normal(&pos, &record);
                // ただし、端の手を省く。
                legal_moves = air2018.pick_move_air2018_filter2(&pos, &record, legal_moves);

                // それでも候補が無ければ、ランダム打ち。
                if legal_moves.is_empty() {
                    legal_moves = pick_move_normal(&pos, &record);
                }
            }

            // 試し打ちをする☆（＾～＾）
            let mut move_code = convert_address_to_code(do_random_move(&mut pos, &legal_moves, &mut record, &mut piece_distribution_searcher), pos.get_board().get_size());

            /* 棋譜データをクリアーしてないので止めで。☆（＾～＾）毎対局、再起動しろだぜ☆（＾～＾）
            if 2*410 < record.len() {
                // 2*410 手も超えていれば投了する☆（＾～＾）
                println!("410手ぐらいを超えていれば投了する☆（＾～＾） 自分の覚えている棋譜の長さ: '{}'.", record.len());
                move_code = 0;
            }
             */

            println!("BestMove: '{}'.", move_code);

            // in.txt ファイル出力。
            BestMove::save(&conf.in_path, move_code as usize);
        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}
