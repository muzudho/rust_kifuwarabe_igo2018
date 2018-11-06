/// 参考:
/// https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;
use serde_json::Value;

use std::fs;
use std::fs::File;
use std::io::Read;

use std::path::Path;

use std::thread;
use std::time::Duration;

/// # 実行方法
/// [Windows]+[R], "cmd",
///
/// ```
/// ### コンパイル
/// cd  C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy
///
/// ### 実行
/// cargo run --release
/// ```
///
fn main() {
    // 設定ファイル読込。
    let mut conf_file = match File::open("config.json") {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    let mut conf_contents = String::new();
    match conf_file.read_to_string(&mut conf_contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // https://docs.serde.rs/serde_json/value/enum.Value.html
    let conf_v: Value = match serde_json::from_str(&conf_contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // コメント取得。
    let conf_comment = conf_v["comment"].as_str().expect("comment.").to_string();
    println!("Config comment: '{}'.", conf_comment);

    // 何路盤。
    let board_size: usize = conf_v["boardSize"].as_i64().expect("boardSize.") as usize; // FIXME 変換方法が分からん☆（＾～＾）
    println!("Config conf_board_size: {}.", board_size);

    loop {
        // ファイル確認。
        // println!("{}", Path::new("position.json").exists());
        if Path::new("position.json").exists() {
            // 指し手ファイル読込。
            let mut pos_file = match File::open("position.json") {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

            let mut pos_contents = String::new();
            match pos_file.read_to_string(&mut pos_contents) {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

            let pos_v: Value = match serde_json::from_str(&pos_contents) {
                Ok(n) => n,
                Err(err) => panic!("File open error. {:?}", err),
            };

            // コメント取得。
            let pos_comment = pos_v["comment"].as_str().unwrap().to_string();
            println!("Pos comment: '{}'.", pos_comment);

            // 盤面取得。
            let mut i = 0;
            let mut board = [9; 21 * 21]; // 19路盤枠ありが入るサイズを確保しておく。使ってない数字で埋める☆（＾～＾）
            for line in pos_v["board"].as_array().unwrap().iter() {
                let chars = line.as_str().unwrap().chars().collect::<Vec<char>>();
                for ch in &chars {
                    board[i] = match ch {
                        'x' => 1, // 黒。
                        'o' => 2, // 白。
                        '+' => 3, // 枠。
                        _ => 0,   // スペース。
                    };
                    i += 1;
                }
                println!("Line: '{}'.", line);
            }

            let turn = pos_v["turn"].as_str().unwrap().to_string();
            println!("Turn: '{}'.", turn);

            // 読み取ったらファイル削除。
            fs::remove_file("position.json");

            // 盤を表示☆（＾～＾）
            println!("Board: ");
            i = 0;
            for num in board.iter() {
                if i == (board_size+2) * (board_size+2) {
                    break;
                }
                print!("{}, ", num);
                if i % (board_size + 2) == (board_size + 1) {
                    println!();
                }
                i += 1;
            }

            // 連のIDを振る。
            let mut ren_id_board = [0; 21 * 21];
            check_liberty(board_size, board, &mut ren_id_board);

            // 連のIDを表示☆（＾～＾）
            println!("Ren ID board: ");
            i = 0;
            for ren_id in ren_id_board.iter() {
                print!("{:3}, ", ren_id);
                if i == (board_size+2) * (board_size+2) {
                    break;
                } else if i % (board_size + 2) == (board_size + 1) {
                    println!();
                }
                i += 1;
            }
        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

/// 連の算出。
fn check_liberty(board_size:usize, board:[i8;21*21], ren_id_board:&mut [i8;21*21]) {

    // 枠の中の左上隅から右下隅まで検索☆（＾～＾）
    // 小さい盤で数えてみろだぜ☆（＾～＾）
    //
    // ++++
    // +  +
    // +  +
    // ++++
    //
    // は board_size 2 で、セル番号は
    //
    //  0, 1, 2, 3,
    //  4, 5, 6, 7,
    //  8, 9,10,11,
    // 12,13,14,15
    //
    // だから、枠の中の 左上隅は 5、右下隅は 10 で、算出方法は以下の通り☆
    let left_top = (board_size+2) + 1;
    let rigth_bottom = (board_size+2) * board_size + board_size;

    for start in left_top..rigth_bottom { // 検索を開始するセルの番号。連のIDを決めるのにも使う。
        let color = board[start]; // 開始地点にある石の色。この石と同じ色を探す。
        if color==1 || color==2 { // 黒石か白石だけ探せばいい☆（＾～＾）
            let opponent = (color+2)%2+1;// 相手の石の色。
            walk_liberty(start as i8, color, opponent, board_size, board, ren_id_board, start); // まず開始地点から。
        }
    }
}

/// 連の計算中。
/// # Parameters.
/// * `dir` - 0: 上, 1: 右, 2: 下, 3: 左.
fn walk_liberty(ren_id:i8, color:i8, opponent:i8, board_size:usize, board:[i8;21*21], ren_id_board:&mut [i8;21*21], target:usize){
    // 連IDが振られてたら終了。ただし相手の石の色を除く。
    if ren_id_board[target] != 0 && ren_id_board[target] != opponent { // 0 は枠セル番号なんで、連IDに使わない。
        return;
    }
    // 探している石でなければ、自分の石の色をマークして終了。
    if board[target] != color {
        ren_id_board[target] = color; // 1とか2 という数は枠セル番号なんで、連IDに使わない。
        return;
    }

    // 探している色の石なら 連ID を付ける。検索を開始したセル番号でも振っとく。
    ren_id_board[target] = ren_id;

    // 隣を探す。（再帰）
    walk_liberty(ren_id, color, opponent, board_size, board, ren_id_board, target-(board_size+2));// 上 。
    walk_liberty(ren_id, color, opponent, board_size, board, ren_id_board, target+1);// 右。
    walk_liberty(ren_id, color, opponent, board_size, board, ren_id_board, target+(board_size+2));// 下。
    walk_liberty(ren_id, color, opponent, board_size, board, ren_id_board, target-1);// 左。
}

/// TODO トライアウト。
/// 盤上に適当に石を置き続けて終局図に持っていくこと。どちらも石を置けなくなったら終了。
fn tryout() {}

/// TODO 勝敗判定。
/// 純碁ルールでは、終局図では簡単で　単に盤上の石の数が多い方の勝ち。
/// 途中図で石の数を数えても　何の当てにもならない☆（＾～＾）
/// だから tryout(); してから呼び出せだぜ☆（＾～＾）
///
/// # Returns.
/// 黒番が勝ってれば 0, 白番が勝ってれば 1, 引き分けなら 2。
fn judge() -> i8 {
    0
}

/// TODO 次の１手を返す。
/// 書式は yyxx。 端には枠があるので、右上隅が 0101。左下隅が 1919。
fn think() -> i8 {
    tryout();
    judge();
    101
}
