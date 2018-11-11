// ランダムムーブ
extern crate rand;
use rand::Rng;

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

mod config;
use config::Config;

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
    let conf = Config::load("config.json");

    println!("Config comment: '{}'.", conf.comment);
    println!("Config conf_board_size: {}.", conf.board_size);


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

            // 何手目か。
            let mut ply = pos_v["ply"].as_i64().unwrap();
            println!("ply: '{}'.", ply);

            // 盤面作成。
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

            // 盤面表示☆（＾～＾）
            show_board(conf.board_size, board);

            // 手番の石の色☆（＾～＾） 1:黒, 2:白。
            let mut turn = match pos_v["turn"].as_str().unwrap() {
                "black" => 1,
                "white" => 2,
                _ => panic!("Undefined turn: [{}].", pos_v["turn"].as_str().unwrap())
            };
            println!("Turn: '{}'.", turn);

            // 読み取ったらファイル削除。
            fs::remove_file("position.json");

            // 盤番地を表示☆（＾～＾）
            println!("Cell address: ");
            i = 0;
            for _stone in board.iter() {
                if i == (conf.board_size+2) * (conf.board_size+2) {
                    break;
                }
                print!("{:3}, ", i);
                if i % (conf.board_size + 2) == (conf.board_size + 1) {
                    println!();
                }
                i += 1;
            }

            // 盤を表示☆（＾～＾）
            show_board_by_number(conf.board_size, board);

            // 連のIDを振る。
            let mut ren_id_board = [0; 21 * 21];
            let mut liberty_count_map = [0; 21*21];
            check_liberty(conf.board_size, board, &mut ren_id_board, &mut liberty_count_map);

            // 連のIDを表示☆（＾～＾）
            println!("Ren ID board: ");
            i = 0;
            for ren_id in ren_id_board.iter() {
                if i == (conf.board_size+2) * (conf.board_size+2) {
                    break;
                }
                print!("{:4}, ", ren_id);
                if i % (conf.board_size + 2) == (conf.board_size + 1) {
                    println!();
                }
                i += 1;
            }

            println!("Liberty count: ");
            for (ren_id, lib_cnt) in liberty_count_map.iter().enumerate() {
                if *lib_cnt != 0 {
                    println!("[{:3}] {:3}", ren_id, lib_cnt);
                }
            }

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
            let ko = 0;
            let forbidden = is_forbidden(convert_code_to_address(704, conf.board_size), turn, conf.board_size, board, ren_id_board, liberty_count_map, ko);
            println!("forbidden? {}", forbidden);
            let forbidden = is_forbidden(convert_code_to_address(401, conf.board_size), turn, conf.board_size, board, ren_id_board, liberty_count_map, ko);
            println!("forbidden? {}", forbidden);

            // ↓トライアウトの練習をする☆（＾～＾）

            // 相手がパスしていれば真。
            let mut opponent_passed = false;

            // ランダムムーブする☆（＾～＾） 上限は 400手でいいだろ☆（＾ｑ＾）
            for i_ply in ply..401 {
                let legal_moves = pick_move(turn, conf.board_size, board, ren_id_board, liberty_count_map, ko);
                // 合法手の表示☆（＾～＾）
                show_legal_moves(&legal_moves);
                // 合法手があれば、ランダムに１つ選ぶ。
                if do_random_move(turn, conf.board_size, &mut board, &legal_moves) {
                    // パスなら
                    if opponent_passed {
                        // TODO ゲーム終了☆（＾～＾）
                        println!("Pass: game end.");
                        break;
                    }
                    opponent_passed = true;
                }
                else {
                    // パスで無かったら。
                    opponent_passed = false;
                }

                // 盤を表示☆（＾～＾）
                println!("Ply: {}, Turn: {}.", i_ply, turn);
                show_board(conf.board_size, board);

                // 手番を反転する☆（＾～＾）
                turn = get_opponent(turn);
            }
            // 連続パス が起こったら終了☆（＾～＾）400手目を打ったところでも終了☆（＾～＾）

            // TODO コウをなんとかしろだぜ☆（*＾～＾*）
            println!("Finished.");
        }

        thread::sleep(Duration::from_millis(1));
    }
    // サーバーは、[Ctrl]+[C]キーで強制終了しろだぜ☆（＾～＾）
}

/// 盤の表示☆（＾～＾）
fn show_board(board_size:usize, board:[i8; 21 * 21]){
    println!("Board: ");
    for (i, stone) in board.iter().enumerate() {
        if i == (board_size+2) * (board_size+2) {
            break;
        }

        print!("{}", match stone {
            0 => ' ',
            1 => 'x',
            2 => 'o',
            _ => '+',
        });

        if i % (board_size + 2) == (board_size + 1) {
            println!();
        }
    }
}

/// 盤の表示☆（＾～＾）
fn show_board_by_number(board_size:usize, board:[i8; 21 * 21]) {
    println!("Board: ");
    for (i, stone) in board.iter().enumerate() {
        if i == (board_size+2) * (board_size+2) {
            break;
        }
        print!("{}, ", stone);
        if i % (board_size + 2) == (board_size + 1) {
            println!();
        }
    }
}

/// 合法手の表示☆（＾～＾）
fn show_legal_moves(legal_moves:&[usize]){ // &Vec<usize>
    print!("Legal moves: ");
    for legal_move in legal_moves {
        print!("{}, ", legal_move);
    }
    println!(".");
}

/// 合法手の中からランダムに１つ選んで打つ☆（＾～＾） 無ければパス☆（＾～＾）
/// # Return.
/// - パスしたら真。
fn do_random_move(color:i8, board_size:usize, board:&mut[i8; 21 * 21], legal_moves:&[usize]) -> bool {
    let best_move = if (*legal_moves).is_empty() {0}else{*rand::thread_rng().choose(legal_moves).unwrap()};
    println!("Best move: {} {:04}.", best_move, convert_address_to_code(best_move, board_size));
    if best_move==0 {
        // パス
        return true;
    }

    // 石を置く。
    board[best_move] = color;
    false
}

// 符号を番地に変換。
//
// 例えば 3x3 の盤の中段右は x=3, y=2 と数えて、
//
//  x123
// y+++++   0  1  2  3  4
// 1+   +   5  6  7  8  9
// 2+  *+  10 11 12 13 14
// 3+   +  15 16 17 18 19
//  +++++  20 21 22 23 24
//
// 符号は 302、番地は 5 とする。
// 符号は人間が読み書きする用なので 入出力ファイルでのみ使用し、プログラム中では 番地 のみ使う。
fn convert_code_to_address(code:i16, board_size:usize) -> usize {
    // x と y に分解。
    // コードの算出。
    (code % 100i16 * (board_size as i16 + 2i16) + code / 100i16 % 100i16) as usize
}

/// 番地を 符号に変換する。
fn convert_address_to_code(address:usize, board_size:usize) -> i16 {
    // x を算出。
    let x = address as i16 % (board_size as i16 + 2i16);
    // y を算出。
    let y = address as i16 / (board_size as i16 + 2i16);
    // 符号にまとめる。
    x * 100 + y
}

/// 連の算出。
fn check_liberty(board_size:usize, board:[i8;21*21], ren_id_board:&mut [i16;21*21], liberty_count_map:&mut [i8;21*21]) {

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

    for start in left_top..rigth_bottom+1 { // 検索を開始するセルの番号。連のIDを決めるのにも使う。
        let color = board[start]; // 開始地点にある石の色。この石と同じ色を探す。
        if color==1 || color==2 { // 黒石か白石だけ探せばいい☆（＾～＾）
            walk_liberty(start as i16, color, board_size, board, ren_id_board, liberty_count_map, start); // まず開始地点から。
        }
    }
}

/// 連にIDを振り、連の呼吸点も数える。
/// # Arguments.
/// * `ren_id_board` - 1000以上はtemporaryな数。
fn walk_liberty(ren_id:i16, color:i8, board_size:usize, board:[i8;21*21], ren_id_board:&mut [i16;21*21], liberty_count_map:&mut [i8;21*21], target:usize){
    if board[target] == 0 && ren_id_board[target] != ren_id + 1000 { // 調べた先が空点で、まだ今回マークしていなければ。
        // println!("LIB: [{:3}] {:3}", ren_id, target);
        liberty_count_map[ren_id as usize] += 1;
    }
    
    if (ren_id_board[target] != 0 && ren_id_board[target] != ren_id + 1000) // 連IDが振られてたら終了。ただし「自分の連ID + 1000」を除く。0 は枠セル番号なんで、連IDに使わない。
        || // または、
        board[target] != color // 探している石でなければ終了。
    {
        if board[target] == 0 || 1000 <= ren_id_board[target] { // そこが空点か、1000以上の連IDなら「自分の連ID + 1000」を上書きでマークしておく。
            ren_id_board[target] = ren_id + 1000;
        }
        return;
    }

    // 探している色の石なら 連ID を付ける。検索を開始したセル番号でも振っとく。
    ren_id_board[target] = ren_id;

    // 隣を探す。（再帰）
    walk_liberty(ren_id, color, board_size, board, ren_id_board, liberty_count_map, target-(board_size+2));// 上。
    walk_liberty(ren_id, color, board_size, board, ren_id_board, liberty_count_map, target+1);// 右。
    walk_liberty(ren_id, color, board_size, board, ren_id_board, liberty_count_map, target+(board_size+2));// 下。
    walk_liberty(ren_id, color, board_size, board, ren_id_board, liberty_count_map, target-1);// 左。
}

/// 相手の石の色☆（＾～＾）
/// # Argumetns.
/// * `color` - 石の色。 1:黒, 2:白.
fn get_opponent(color:i8) -> i8 {
    (color+2)%2+1
}

/// 着手禁止点（自殺手またはコウ）なら真。
/// # Arguments.
/// * `target` - 石を置きたい空点の番地。
/// * `color` - 置く石の色。 1:黒, 2:白.
fn is_forbidden(target:usize, color:i8, board_size:usize, board:[i8;21*21], ren_id_board:[i16;21*21], liberty_count_map:[i8;21*21], ko:usize) -> bool {
    
    let top = target-(board_size+2); // 上の番地。
    let right = target+1; // 右。
    let bottom = target+(board_size+2); // 下。
    let left = target-1; // 左。
    let top_ren_id = ren_id_board[top] as usize; // 上の連のID。
    let right_ren_id = ren_id_board[right] as usize; // 上の連のID。
    let bottom_ren_id = ren_id_board[bottom] as usize; // 上の連のID。
    let left_ren_id = ren_id_board[left] as usize; // 上の連のID。
    let opponent = get_opponent(color); // 相手の石の色。

    /*
    println!("forbidden? board[target] != 0 -> {}", board[target] != 0);
    println!("forbidden? target == ko -> {}", target == ko);
    println!("forbidden? board[top] == 0 -> {}", board[top] == 0);
    println!("forbidden? board[right] == 0 -> {}", board[right] == 0);
    println!("forbidden? board[bottom] == 0 -> {}", board[bottom] == 0);
    println!("forbidden? board[left] == 0 -> {}", board[left] == 0);
    println!("forbidden? board[top] == color && 1<liberty_count_map[top_ren_id] -> {}", board[top] == color && 1<liberty_count_map[top_ren_id]);
    println!("forbidden? board[right] == color && 1<liberty_count_map[right_ren_id] -> {}", board[right] == color && 1<liberty_count_map[right_ren_id]);
    println!("forbidden? board[bottom] == color && 1<liberty_count_map[bottom_ren_id] -> {}", board[bottom] == color && 1<liberty_count_map[bottom_ren_id]);
    println!("forbidden? board[left] == color && 1<liberty_count_map[left_ren_id] -> {}", board[left] == color && 1<liberty_count_map[left_ren_id]);
    println!("forbidden? board[top] == opponent && liberty_count_map[top_ren_id] < 2 -> {}", board[top] == opponent && liberty_count_map[top_ren_id] < 2);
    println!("forbidden? board[right] == opponent && liberty_count_map[right_ren_id] < 2 -> {}", board[right] == opponent && liberty_count_map[right_ren_id] < 2);
    println!("forbidden? board[bottom] == opponent && liberty_count_map[bottom_ren_id] < 2 -> {}", board[bottom] == opponent && liberty_count_map[bottom_ren_id] < 2);
    println!("forbidden? board[left] == opponent && liberty_count_map[left_ren_id] < 2 -> {}", board[left] == opponent && liberty_count_map[left_ren_id] < 2);
    */

    if
        // 空点以外は、着手禁止点。
        board[target] != 0
        // コウ（前にアゲるところに石を打ったばかりの番地）なら、着手禁止点。
        || target == ko
    {
        return true;
    }

    if
        // 隣に空点があれば、自殺手ではない。
        board[top] == 0 || board[right] == 0 || board[bottom] == 0 || board[left] == 0
        // 隣に呼吸点が 2つ以上ある自分の色の連が1つでもあれば、自殺手ではない。
        || (board[top] == color && top_ren_id < 1000 && 1<liberty_count_map[top_ren_id])
        || (board[right] == color && right_ren_id < 1000 && 1<liberty_count_map[right_ren_id])
        || (board[bottom] == color && bottom_ren_id < 1000 && 1<liberty_count_map[bottom_ren_id])
        || (board[left] == color && left_ren_id < 1000 && 1<liberty_count_map[left_ren_id])
        // 隣に呼吸点が 1つ以下の相手の色の連が1つでもあれば、自殺手ではない。
        || (board[top] == opponent && top_ren_id < 1000 && liberty_count_map[top_ren_id] < 2)
        || (board[right] == opponent && right_ren_id < 1000 && liberty_count_map[right_ren_id] < 2)
        || (board[bottom] == opponent && bottom_ren_id < 1000 && liberty_count_map[bottom_ren_id] < 2)
        || (board[left] == opponent && left_ren_id < 1000 && liberty_count_map[left_ren_id] < 2)
    {
        return false;
    }

    // それ以外なら 着手禁止点。
    true
}

/// 合法手生成。
fn pick_move(color:i8, board_size:usize, board:[i8;21*21], ren_id_board:[i16;21*21], liberty_count_map:[i8;21*21], ko:usize) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();

    let left_top = (board_size+2) + 1;
    let rigth_bottom = (board_size+2) * board_size + board_size;

    for target in left_top..rigth_bottom+1 {
        if !is_forbidden(target, color, board_size, board, ren_id_board, liberty_count_map, ko) {
            vec.push(target);
        }
    }

    vec
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
