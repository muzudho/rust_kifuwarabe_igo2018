// 空連に関するもの☆（＾～＾）

use position::Position;
use view::*;


/// 空連にIDを振り、空連の占有者も調べる。
pub fn walk_empty(ren_id:usize, pos:&mut Position, target:usize) {
    match pos.board.get(target) {
        0 => {
            // 空点。
            if 0 < pos.empty_owner_map.address_ren_board.get(target) {
                // 空点 しかし、既に連IDが振ってある。調査済みだ。 --> 隣は調べない。
                return;
            }

            // 空点 かつ、連IDが振ってない --> 連IDを振る。隣も調べる。
            pos.empty_owner_map.address_ren_board.set(target, ren_id as i16);
            pos.empty_owner_map.space.add(ren_id as i16, target as i16);
        },

        1 => {
            if pos.empty_owner_map.get(ren_id) == 3 {
                // 占有者は白黒両方の場合、更新なし。
            } else if pos.empty_owner_map.get(ren_id) == 2 {
                // 黒石 かつ、占有者が 黒、白の両方になる。
                pos.empty_owner_map.set(ren_id, 3);
            } else {
                // 占有者は黒石か。
                pos.empty_owner_map.set(ren_id, 1);
            }

            // 隣は調べない。
            return;
        },

        2 => {
            if pos.empty_owner_map.get(ren_id) == 3 {
                // 占有者は白黒両方の場合、更新なし。
            } else if pos.empty_owner_map.get(ren_id) == 1 {
                // 白石 かつ、占有者が 黒、白の両方になる。
                pos.empty_owner_map.set(ren_id, 3);
            } else {
                // 占有者は白石か。
                pos.empty_owner_map.set(ren_id, 2);
            }

            // 隣は調べない。
            return;
        },

        3 => {
            // 枠 --> 隣は調べない。
            return;
        },

        _ => {panic!("想定しない石の種類。 '{}'", pos.board.get(target))},
    };

    // 隣を探す。（再帰）
    let top = pos.get_top_of(target);
    let right = target+1;
    let bottom = pos.get_bottom_of(target);
    let left = target-1;
    // 上。
    walk_empty(ren_id, pos, top);
    // 右。
    walk_empty(ren_id, pos, right);
    // 下。
    walk_empty(ren_id, pos, bottom);
    // 左。
    walk_empty(ren_id, pos, left);
}

/// 連の一点を指定し、そこを切断する。その結果、次のいずれかが起こる。
/// --> 連から１つのアドレスが減る。
/// --> 2～4つの連に分割する。
/// # Arguments.
/// * 'cutting_addr' - 石を置いて、消えるところ。
pub fn cut_empty_ren(pos:&mut Position, cutting_addr:usize) {

    // その番地のリスト。

    // 指定の連IDを探索し、一番小さい番地を返す関数。

    {
        // 空連ID。
        let empty_ren_id = pos.empty_owner_map.address_ren_board.get(cutting_addr);

        // 石を置いた交点から探索。
        println!("board size: {}.", pos.board.get_size());
        println!("cutting_addr: {}.", cutting_addr);

        pos.address_ren_board_searcher.count_up_mark();
        let mut shrink: Vec<i16> = Vec::new();

        {
            // 上側の空連を起点に探索し、一番小さい番地。
            let start = pos.board.get_top_of(cutting_addr);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, empty_ren_id, start, cutting_addr);
            println!("空連{} の上を探索。", empty_ren_id);
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == empty_ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = pos.address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&pos.address_ren_board_searcher.get_found_addr());
            }
        }

        {
            // 右側の空連を起点に探索し、一番小さい番地。
            let start = pos.board.get_right_of(cutting_addr);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, empty_ren_id, start, cutting_addr);
            println!("空連{} の右を探索。", empty_ren_id);        
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == empty_ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = pos.address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&pos.address_ren_board_searcher.get_found_addr());
            }
        }

        {
            // 下側の空連を起点に探索し、一番小さい番地。
            let start = pos.board.get_bottom_of(cutting_addr);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, empty_ren_id, start, cutting_addr);
            println!("空連{} の下を探索。", empty_ren_id);        
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == empty_ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = pos.address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&pos.address_ren_board_searcher.get_found_addr());
            }
        }

        {
            // 左側の空連を起点に探索し、一番小さい番地。
            let start = pos.board.get_left_of(cutting_addr);
            let min_addr = pos.address_ren_board_searcher.get_min_address(&pos.board, &pos.empty_owner_map.address_ren_board, empty_ren_id, start, cutting_addr);
            println!("空連{} の左を探索。", empty_ren_id);
            if min_addr == 0 {
                println!("空連なし。");
            } else if min_addr == empty_ren_id {
                // 連ID が被っている。
                print!("縮まった空連: {}, 番地: ", min_addr);
                shrink = pos.address_ren_board_searcher.get_found_addr().to_vec();
            } else {
                print!("分かれた空連: {}, 番地: ", min_addr);
                show_vector_i16(&pos.address_ren_board_searcher.get_found_addr());
            }
        }

        if 0 < shrink.len() {
            pos.empty_owner_map.space.remove(empty_ren_id);
            pos.empty_owner_map.space.insert(empty_ren_id, shrink);

            print!("縮まった空連の作り直し。番地: ");
            match &pos.empty_owner_map.space.get(empty_ren_id) {
                Some(s) => show_vector_i16(s),
                None => {},
            };
            println!(".");
        }
    }

    // pos.address_ren_board_searcher.

    // 探索前の空連は破棄。

    // 新しい空連を追加。
}