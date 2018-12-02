#![allow(dead_code)]

// ランダムムーブ
extern crate rand;
use rand::Rng;

/// 参考: https://github.com/serde-rs/json |シリアライズ、デシリアライズ。
extern crate serde_json;

/// このライブラリーに含まれる公開モジュール☆（＾～＾）
pub mod address_ren_board_searcher;
pub mod best_move;
pub mod board;
pub mod config_file;
pub mod empty_ren;
pub mod liberty;
pub mod out_file;
pub mod position_file;
pub mod position;
pub mod record;
pub mod ren_database;
pub mod stone_ren;
pub mod view;
pub mod zobrist_hash;

use address_ren_board_searcher::*;
use position::Position;
use empty_ren::*;
use liberty::*;
use record::*;
use ren_database::*;
use view::*;
// use zobrist_hash::*;

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

/// 連IDを塗り替えるぜ☆（＾～＾）
/// # Return.
/// - 新しい連ID。
pub fn refill_address_ren_board(target:usize, adjacent:usize, pos:&mut Position) -> i16 {
    let self_ren_id = target as i16;
    // 隣接する自分の連のID。 1000未満の数。
    let adjacent_ren_id = pos.get_ren_database().get_address_stone_ren_board().get(adjacent);

    // IDの数が 小さくない方 を、小さい方に塗り替える☆（＾～＾）
    if self_ren_id < adjacent_ren_id {
        println!("Do move: Self: {}, Adjacent: {}. 新しいIDの方が小さい。", self_ren_id, adjacent_ren_id);
        {
            let adjacent_ren_addr_vec = match pos.get_ren_database().get_stone_ren_map().get_ren(adjacent_ren_id) {
                Some(adjacent_ren_obj) => {
                    adjacent_ren_obj.to_addr_vec()
                },
                None => {panic!("Self: {}, Adjacent: {}.", self_ren_id, adjacent_ren_id)},
            };

            pos.get_mut_ren_database().get_mut_address_stone_ren_board().fill_by_vec(&adjacent_ren_addr_vec, self_ren_id);
        }

        // キー変更。
        pos.get_mut_ren_database().get_mut_stone_ren_map().change_key(adjacent_ren_id, self_ren_id);
        // pos.get_mut_ren_database().get_mut_stone_ren_map().change_key_liberty_count(adjacent_ren_id, self_ren_id);

        self_ren_id
    } else {
        println!("Do move: Self: {}, Adjacent: {}. 隣のIDの方が小さい。", self_ren_id, adjacent_ren_id);
        pos.get_mut_ren_database().get_mut_address_stone_ren_board().set(target, adjacent_ren_id);
        adjacent_ren_id
    }
}

/// 連を盤から除去するぜ☆（＾～＾）
/// # Arguments.
/// * `record` - 打ち上げた石の番地を覚えるのに使う。
pub fn peel_off_by_ren_id(adjacent:usize, pos:&mut Position, record:&mut Record) {
    // 除去される連ID。
    let adjacent_ren_id = pos.get_ren_database().get_address_stone_ren_board().get(adjacent);
    let adj_lib_cnt = pos.get_ren_database().get_stone_ren_map().get_ren(adjacent_ren_id).expect("peel_off_by_ren_id(1)").get_liberty_count();
    println!("Do move: 隣の連ID {}, 隣の呼吸点数 {}。", adjacent_ren_id, adj_lib_cnt);

    // 呼吸点
    if adj_lib_cnt==1 {
        // この連を盤から除去する。

        // 除去されるアドレス一覧。
        {
            let adjacent_ren_addr_vec = match pos.get_ren_database().get_stone_ren_map().get_ren(adjacent_ren_id) {
                Some(adjacent_ren_obj) => (*adjacent_ren_obj).to_addr_vec(),
                None => {panic!("Adjacent: {}.", adjacent_ren_id)},
            };

            pos.get_mut_board().fill_by_vec(&adjacent_ren_addr_vec, 0);

            pos.get_mut_ren_database().get_mut_address_stone_ren_board().fill_by_vec(&adjacent_ren_addr_vec, 0);

            record.add_current_agehama_by_vec(&adjacent_ren_addr_vec);
        }

        // キー削除。
        pos.get_mut_ren_database().get_mut_stone_ren_map().remove_ren(adjacent_ren_id);
        // pos.get_mut_ren_database().get_mut_stone_ren_map().get_mut_ren(adjacent_ren_id).expect("peel_off_by_ren_id(2)").set_liberty_count(0);
    }
}

/// 石を置くぜ☆（*＾～＾*）
/// 自殺手、コウの可能性は事前に除去しておくこと☆（＾～＾）
/// # Return.
/// - パスしたら真。
pub fn do_move(target:usize, pos:&mut Position, record:&mut Record, address_ren_board_searcher:&mut AddressRenBoardSearcher) -> bool {
    println!("Move: {} {:04}.", target, convert_address_to_code(target, pos.get_board().get_size()));

    // 手数のカーソルを１個進める。
    record.count_up();

    // パス
    if target == 0 {
        return true;
    }

    // 手番の石の色を ひっくり返す。
    {
        let turn = pos.turn;
        pos.get_mut_board().set_stone(target, turn);
    }
    record.set_current(target as i16, pos.get_board().get_hash());
    // 空連を切る。
    cut_empty_ren(pos, target, address_ren_board_searcher);

    let top = pos.get_board().get_top_of(target); // 上の番地。
    let right = pos.get_board().get_right_of(target); // 右。
    let bottom = pos.get_board().get_bottom_of(target); // 下。
    let left = pos.get_board().get_left_of(target); // 左。


    // TODO 石が隣接していれば、連が変わる☆（＾～＾） 0～4つの連が隣接している☆（＾～＾）

    // 連がつながるか調べたいので、自分の色と比較☆（＾～＾） 上、右、下、左。
    // - [v] 連のIDの更新。
    // TODO - [ ] 連の要素の更新。
    let mut small_id = target as i16;
    small_id = if pos.get_board().get_stone(top) == pos.turn {
        println!("Do move: 上とつながる。");
        // 置いた石と、隣の 連ID を見比べて、小さなID の方で塗りつぶす。
        refill_address_ren_board(target, top, pos)
    } else {small_id};
    small_id = if pos.get_board().get_stone(right) == pos.turn {
        println!("Do move: 右とつながる。");
        refill_address_ren_board(target, right, pos)
    } else {small_id};
    small_id = if pos.get_board().get_stone(bottom) == pos.turn {
        println!("Do move: 下とつながる。");
        refill_address_ren_board(target, bottom, pos)
    } else {small_id};
    small_id = if pos.get_board().get_stone(left) == pos.turn {
        println!("Do move: 左とつながる。");
        refill_address_ren_board(target, left, pos)
    } else {small_id};

    // [v] 連ID から 紐づくすべての石を取得したい☆（＾～＾） -> RenAddressMap を使う☆（＾～＾）

    // [v] 指定連ID を持つ石を、 べつの指定連ID に塗り替えたい☆（＾～＾） -> AddressRenBoard を使う☆（＾～＾）

    // [v] 今置いたばかりの石の連ID も、指定連ID にする☆（＾～＾） -> 連の要素一覧に 置いた石の番地を 追加。
    pos.get_mut_ren_database().get_mut_stone_ren_map().add_addr(small_id, target as i16);

    // TODO - [ ] 呼吸点の更新。 置いた石の呼吸点と、接続した連の呼吸点 を足して 1 引く☆（＾～＾）
    let target_liberty_count = count_liberty_at_point(target, &pos.get_board());
    println!("Do move: Target_liberty_count: {}.", target_liberty_count);
    pos.get_mut_ren_database().get_mut_stone_ren_map().get_mut_ren(small_id).expect("do_move(1)").add_liberty_count(i16::from(target_liberty_count - 1));


    // TODO アンドゥを考えるなら、置き換える前の ID を覚えておきたい☆（＾～＾） 棋譜としてスタックに積み上げか☆（＾～＾）

    // 今回のコウは消す。
    pos.ko = 0;

    // TODO 隣接しているのが相手の石で、呼吸点が 1 なら、その連は取れる☆（＾～＾）
    let opponent = get_opponent(pos.turn);
    if pos.get_board().get_stone(top) == opponent {
        println!("Do move: 上に相手の石。");
        peel_off_by_ren_id(top, pos, record);

        // コウ。
        if 1 == record.get_current().agehama_addrs.len() {
            pos.ko = top as i16;
        }
    }
    if pos.get_board().get_stone(right) == opponent {
        println!("Do move: 右に相手の石。");
        peel_off_by_ren_id(right, pos, record);

        // コウ。
        if 1 == record.get_current().agehama_addrs.len() {
            pos.ko = right as i16;
        }
    }
    if pos.get_board().get_stone(bottom) == opponent {
        println!("Do move: 下に相手の石。");
        peel_off_by_ren_id(bottom, pos, record);

        // コウ。
        if 1 == record.get_current().agehama_addrs.len() {
            pos.ko = bottom as i16;
        }
    }
    if pos.get_board().get_stone(left) == opponent {
        println!("Do move: 左に相手の石。");
        peel_off_by_ren_id(left, pos, record);

        // コウ。
        if 1 == record.get_current().agehama_addrs.len() {
            pos.ko = left as i16;
        }
    }

    // TODO アンドゥを考えるなら、取った連を 棋譜としてスタックに積み上げか☆（＾～＾）

    // TODO 純碁ルールなら アゲハマを覚えておかなくていい☆（＾～＾） 楽☆（＾～＾）

    // TODO 相手の石をウチアゲたなら、置いた石の番地を preKo として覚えておく☆（＾～＾） 前の preKo は ko へ退避しておく☆（＾～＾）
    // 相手がコウでなければ、当然取り返しに来る☆（＾～＾）preKo は ko へ、 preKo に置いた石の番地を入れる☆（＾～＾）

    // 手番をひっくり返す。
    pos.turn = opponent;

    false
}

/// TODO 一手戻すぜ☆（＾～＾）
pub fn undo_move(pos:&mut Position, record:&mut Record) {
    let last = match record.count_down() {
        Some(n) => n,
        None => {
            println!("Undo fail.");
            return;
        },
    };
    println!("Undo move: {:04}.", convert_address_to_code(last.move_addr as usize, pos.get_board().get_size()));

    // 置いた石を取り除く。
    pos.get_mut_board().set_stone(last.move_addr as usize, 0);

    // TODO ウチアゲた石も戻したい。
}

/// 合法手の中からランダムに１つ選んで打つ☆（＾～＾） 無ければパス☆（＾～＾）
/// # Return.
/// - 石を打った番地。
pub fn do_random_move(pos:&mut Position, legal_moves:&[usize], record:&mut Record, address_ren_board_searcher:&mut AddressRenBoardSearcher) -> usize {

    // - 置ける場所がなければパス。
    // - 置ける場所からランダムに１つ選ぶ。
    let best_move = if (*legal_moves).is_empty() {
        0
    } else {
        *rand::thread_rng().choose(legal_moves).unwrap()
    };

    // 石を置く。
    do_move(best_move, pos, record, address_ren_board_searcher);

    best_move
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
/// * `pos` - 局面。
/// * `record` - スーパーコウの判定に使う予定。
pub fn is_forbidden(target:usize, pos:&Position, record:&Record) -> bool {
    
    // 上、右、下、左の番地。
    let top = pos.get_top_of(target);
    let right = pos.get_right_of(target);
    let bottom = pos.get_bottom_of(target);
    let left = pos.get_left_of(target);

    // 上、右、下、左の連のID。
    let top_ren_id = pos.get_ren_database().get_address_stone_ren_board().get(top) as usize;
    let right_ren_id = pos.get_ren_database().get_address_stone_ren_board().get(right) as usize;
    let bottom_ren_id = pos.get_ren_database().get_address_stone_ren_board().get(bottom) as usize;
    let left_ren_id = pos.get_ren_database().get_address_stone_ren_board().get(left) as usize;

    // 相手の石の色。
    let opponent = get_opponent(pos.turn);

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

    // 着手禁止と すぐ分かるところなら、さっさと真を返すぜ☆（＾～＾）

    // 空点と、コウ☆（＾～＾）
    if pos.get_board().get_stone(target) != 0
       ||
       target as i16 == pos.ko
    {
        return true;
    }

    // FIXME 目つぶしは、着手禁止点扱いにする。連をつなぐ有効な手の場合もあるが。
    if let Some(ren_obj) = pos.get_ren_database().get_empty_ren_map().get_ren(target as i16) {
        if ren_obj.is_eye_filling(pos.turn) {
            return true;
        }
    }

    // 着手可能と すぐ分かるところなら、さっさと偽を返すぜ☆（＾～＾）
    // - 盤。
    // - 連の呼吸点。
    if
        // 隣に空点があれば、自殺手ではない。
        pos.get_board().get_stone(top) == 0 || pos.get_board().get_stone(right) == 0 || pos.get_board().get_stone(bottom) == 0 || pos.get_board().get_stone(left) == 0
        // 隣に呼吸点が 2つ以上ある自分の色の連が1つでもあれば、自殺手ではない。
        || (pos.get_board().get_stone(top) == pos.turn && top_ren_id < 1000 && 1<pos.get_ren_database().get_stone_ren_map().get_ren(top_ren_id as i16).expect("is_forbidden(1)").get_liberty_count())
        || (pos.get_board().get_stone(right) == pos.turn && right_ren_id < 1000 && 1<pos.get_ren_database().get_stone_ren_map().get_ren(right_ren_id as i16).expect("is_forbidden(2)").get_liberty_count())
        || (pos.get_board().get_stone(bottom) == pos.turn && bottom_ren_id < 1000 && 1<pos.get_ren_database().get_stone_ren_map().get_ren(bottom_ren_id as i16).expect("is_forbidden(3)").get_liberty_count())
        || (pos.get_board().get_stone(left) == pos.turn && left_ren_id < 1000 && 1<pos.get_ren_database().get_stone_ren_map().get_ren(left_ren_id as i16).expect("is_forbidden(4)").get_liberty_count())
        // 隣に呼吸点が 1つ以下の相手の色の連が1つでもあれば、自殺手ではない。
        || (pos.get_board().get_stone(top) == opponent && top_ren_id < 1000 && pos.get_ren_database().get_stone_ren_map().get_ren(top_ren_id as i16).expect("is_forbidden(5)").get_liberty_count() < 2)
        || (pos.get_board().get_stone(right) == opponent && right_ren_id < 1000 && pos.get_ren_database().get_stone_ren_map().get_ren(right_ren_id as i16).expect("is_forbidden(6)").get_liberty_count() < 2)
        || (pos.get_board().get_stone(bottom) == opponent && bottom_ren_id < 1000 && pos.get_ren_database().get_stone_ren_map().get_ren(bottom_ren_id as i16).expect("is_forbidden(7)").get_liberty_count() < 2)
        || (pos.get_board().get_stone(left) == opponent && left_ren_id < 1000 && pos.get_ren_database().get_stone_ren_map().get_ren(left_ren_id as i16).expect("is_forbidden(8)").get_liberty_count() < 2)
    {
        return false;
    }

    // それ以外なら 着手禁止点。
    true
}

/// 指定局面での合法手生成。
pub fn pick_move(pos:&Position, record:&Record) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();

    // 着手禁止点以外は、全部合法手☆（*＾～＾*）
    for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
        if !is_forbidden(target, pos, record) {
            vec.push(target);
        }
    }

    vec
}

/// TODO トライアウト。
/// 盤上に適当に石を置き続けて終局図に持っていくこと。どちらも石を置けなくなったら終了。
pub fn tryout(pos:&mut Position, record:&mut Record, address_ren_board_searcher:&mut AddressRenBoardSearcher) {
    println!("Start tryout.");

    // 相手がパスしていれば真。
    let mut opponent_passed = false;

    // ランダムムーブする☆（＾～＾） 上限は 2000手でいいだろ☆（＾ｑ＾）
    for i_time in 0 .. 2001 {
        let legal_moves = pick_move(&pos, record);
        // 合法手の表示☆（＾～＾）
        show_legal_moves(&legal_moves);
        // 合法手があれば、ランダムに１つ選ぶ。
        if do_random_move(pos, &legal_moves, record, address_ren_board_searcher) == 0 {
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
        println!("Time: {}, Turn: {}.", i_time, pos.turn);
        show_board(&pos.get_board());

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
