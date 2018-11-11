#![allow(dead_code)]

// ランダムムーブ
extern crate rand;
use rand::Rng;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;

/// このライブラリーに含まれる公開モジュール☆（＾～＾）
pub mod config_file;
pub mod position_file;
pub mod position;
pub mod liberty;

use position::Position;
use std::collections::HashMap;

/// # 実行方法
/// [Windows]+[R], "cmd",
///
/// ```
/// ### コンパイル
/// cd  C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_igo2018
/// cargo clippy
///
/// ### 実行
/// cargo run --example main
/// ```
///

/// 盤の表示☆（＾～＾）
pub fn show_board(board_size:usize, board:[i8; 21 * 21]){
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

/// セル番地を表示☆（＾～＾）
pub fn show_board_address(board_size:usize) {
    println!("Cell address: ");
    let end = (board_size+2) * (board_size+2) + 1;

    for i in 0..end { // 検索を開始するセルの番号。連のIDを決めるのにも使う。
        if i == (board_size+2) * (board_size+2) {
            break;
        }
        print!("{:3}, ", i);
        if i % (board_size + 2) == (board_size + 1) {
            println!();
        }
    }
}

/// 盤の表示☆（＾～＾）
pub fn show_board_by_number(board_size:usize, board:[i8; 21 * 21]) {
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

/// 盤に振られた 連ID を表示だぜ☆（＾～＾）
pub fn show_ren_id_board(board_size:usize, ren_id_board:[i16; 21 * 21]) {
    println!("Ren ID board: ");
    for (i, ren_id) in ren_id_board.iter().enumerate() {
        if i == (board_size+2) * (board_size+2) {
            break;
        }
        print!("{:4}, ", ren_id);
        if i % (board_size + 2) == (board_size + 1) {
            println!();
        }
    }
}

/// 呼吸点の数を表示☆（＾～＾）
pub fn show_libarty_count(liberty_count_map:[i8; 21*21]) {
    println!("Liberty count: ");
    for (ren_id, lib_cnt) in liberty_count_map.iter().enumerate() {
        if *lib_cnt != 0 {
            println!("[{:3}] {:3}", ren_id, lib_cnt);
        }
    }
}

/// 連の要素を表示☆（＾～＾）
pub fn show_ren_element_map(ren_element_map:&HashMap<i8,Vec<i16>>) {
    println!("Ren element: ");
    for (ren_id, addr_vec) in ren_element_map {
        print!("[{:3}] ", ren_id);
        for addr in addr_vec.iter() {
            print!("{:3} ", addr);
        }
        println!(".");
    }
}

/// 合法手の表示☆（＾～＾）
pub fn show_legal_moves(legal_moves:&[usize]) {
    print!("Legal moves: ");
    for legal_move in legal_moves {
        print!("{}, ", legal_move);
    }
    println!(".");
}

/// 石を置くぜ☆（*＾～＾*）
/// 自殺手、コウの可能性は事前に除去しておくこと☆（＾～＾）
/// # Return.
/// - パスしたら真。
pub fn do_move(target:usize, color:i8, board_size:usize, board:&mut[i8; 21 * 21], ren_id_board:&mut [i16; 21 * 21],
    ren_element_map:&mut HashMap<i8, Vec<i16>>) -> bool {
    println!("Move: {} {:04}.", target, convert_address_to_code(target, board_size));

    if target == 0 {
        // パス
        return true;
    }

    board[target] = color;

    let top = target-(board_size+2); // 上の番地。
    let right = target+1; // 右。
    let bottom = target+(board_size+2); // 下。
    let left = target-1; // 左。

    // TODO 石が隣接していれば、連が変わる☆（＾～＾） 0～4つの連が隣接している☆（＾～＾）

    // 連がつながるか調べたいので、自分の色と比較☆（＾～＾）
    // TODO 上、右、下、左。
    if board[top] == color {
        // 隣接する自分の連のID。 1000未満の数。
        let ren_id = ren_id_board[top];
        // IDの数が 小さくない方 を、小さい方に塗り替える☆（＾～＾）
        if target < ren_id as usize {
            for addr in ren_element_map.get(&(ren_id as i8)) {
                ren_id_board[ren_id as usize] = top as i16;
            }
            // キー変更。
            match ren_element_map.remove(&(ren_id as i8)) {
                Some(s) => {ren_element_map.insert(top as i8, s)},
                None => {panic!("ren_id: {}.", ren_id)}
            };
        } else {
            ren_id_board[top] = ren_id;
        }
    }
    if board[right] == color {
        
    }
    if board[bottom] == color {
        
    }
    if board[left] == color {
        
    }

    // TODO 連ID から 紐づくすべての石を取得したい☆（＾～＾）

    // TODO 指定連ID を持つ石を、 べつの指定連ID に塗り替えたい☆（＾～＾）

    // TODO 今置いたばかりの石の連ID も、指定連ID にする☆（＾～＾）

    // TODO アンドゥを考えるなら、置き換える前の ID を覚えておきたい☆（＾～＾） 棋譜としてスタックに積み上げか☆（＾～＾）

    // TODO 隣接しているのが相手の石で、呼吸点が 1 なら、その連は取れる☆（＾～＾）

    // TODO アンドゥを考えるなら、取った連を 棋譜としてスタックに積み上げか☆（＾～＾）

    // TODO 純碁ルールなら アゲハマを覚えておかなくていい☆（＾～＾） 楽☆（＾～＾）

    // TODO 相手の石をウチアゲたなら、置いた石の番地を preKo として覚えておく☆（＾～＾） 前の preKo は ko へ退避しておく☆（＾～＾）
    // 相手がコウでなければ、当然取り返しに来る☆（＾～＾）preKo は ko へ、 preKo に置いた石の番地を入れる☆（＾～＾）

    false
}

/// 合法手の中からランダムに１つ選んで打つ☆（＾～＾） 無ければパス☆（＾～＾）
/// # Return.
/// - パスしたら真。
pub fn do_random_move(color:i8, board_size:usize, board:&mut[i8; 21 * 21], ren_id_board:&mut [i16;21*21],
    ren_element_map:&mut HashMap<i8, Vec<i16>>, legal_moves:&[usize]) -> bool {
    let best_move = if (*legal_moves).is_empty() {0}else{*rand::thread_rng().choose(legal_moves).unwrap()};

    // 石を置く。
    do_move(best_move, color, board_size, board, ren_id_board, ren_element_map)
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
pub fn convert_code_to_address(code:i16, board_size:usize) -> usize {
    // x と y に分解。
    // コードの算出。
    (code % 100i16 * (board_size as i16 + 2i16) + code / 100i16 % 100i16) as usize
}

/// 番地を 符号に変換する。
pub fn convert_address_to_code(address:usize, board_size:usize) -> i16 {
    // x を算出。
    let x = address as i16 % (board_size as i16 + 2i16);
    // y を算出。
    let y = address as i16 / (board_size as i16 + 2i16);
    // 符号にまとめる。
    x * 100 + y
}

/// 相手の石の色☆（＾～＾）
/// # Argumetns.
/// * `color` - 石の色。 1:黒, 2:白.
pub fn get_opponent(color:i8) -> i8 {
    (color+2)%2+1
}

/// 着手禁止点（自殺手またはコウ）なら真。
/// # Arguments.
/// * `target` - 石を置きたい空点の番地。
/// * `color` - 置く石の色。 1:黒, 2:白.
pub fn is_forbidden(target:usize, color:i8, board_size:usize, board:[i8;21*21], ren_id_board:[i16;21*21], liberty_count_map:[i8;21*21], ko:usize) -> bool {
    
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
pub fn pick_move(color:i8, board_size:usize, board:[i8;21*21], ren_id_board:[i16;21*21], liberty_count_map:[i8;21*21], ko:usize) -> Vec<usize> {
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
pub fn tryout(pos:&mut Position, board_size:usize, ren_id_board:&mut[i16;21*21], liberty_count_map:[i8;21*21],
    ren_element_map:&mut HashMap<i8, Vec<i16>>, ko:usize) {
    println!("Start tryout.");

    // 相手がパスしていれば真。
    let mut opponent_passed = false;

    // ランダムムーブする☆（＾～＾） 上限は 400手でいいだろ☆（＾ｑ＾）
    for i_ply in pos.ply..401 {
        let legal_moves = pick_move(pos.turn, board_size, pos.board, *ren_id_board, liberty_count_map, ko);
        // 合法手の表示☆（＾～＾）
        show_legal_moves(&legal_moves);
        // 合法手があれば、ランダムに１つ選ぶ。
        if do_random_move(pos.turn, board_size, &mut pos.board, ren_id_board, ren_element_map, &legal_moves) {
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
        println!("Ply: {}, Turn: {}.", i_ply, pos.turn);
        show_board(board_size, pos.board);

        // 手番を反転する☆（＾～＾）
        pos.turn = get_opponent(pos.turn);
    }
    // 連続パス が起こったら終了☆（＾～＾）400手目を打ったところでも終了☆（＾～＾）

    // TODO コウをなんとかしろだぜ☆（*＾～＾*）
    println!("Finished tryout.");
}

/// TODO 勝敗判定。
/// 純碁ルールでは、終局図では簡単で　単に盤上の石の数が多い方の勝ち。
/// 途中図で石の数を数えても　何の当てにもならない☆（＾～＾）
/// だから tryout(); してから呼び出せだぜ☆（＾～＾）
///
/// # Returns.
/// 黒番が勝ってれば 0, 白番が勝ってれば 1, 引き分けなら 2。
pub fn judge() -> i8 {
    0
}

/// TODO 次の１手を返す。
/// 書式は yyxx。 端には枠があるので、右上隅が 0101。左下隅が 1919。
pub fn think() -> i8 {
    // TODO tryout();
    judge();
    101
}
