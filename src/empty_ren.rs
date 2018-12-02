// 空連に関するもの☆（＾～＾）

use address_ren_board_searcher::*;
use position::Position;
use ren_database::*;
use view::*;


/// 空連にIDを振り、空連の占有者も調べる。
pub fn walk_empty(ren_id:usize, pos:&mut Position, target:usize) {
    match pos.get_board().get_stone(target) {
        0 => {
            // 空点。
            if 0 < pos.get_ren_database().get_address_empty_ren_board().get(target) {
                // しかし、既に連IDが振ってある。調査済みだ。 --> 隣は調べない。
                return;
            }

            // 空点 かつ、連IDが振ってない --> 連IDを振る。隣も調べる。

            // 番地に 空連ID を紐づける。検索を開始したセル番号でも振っとく。
            pos.get_mut_ren_database().get_mut_address_empty_ren_board().set(target, ren_id as i16);

            // 連IDに 番地を追加する。
            pos.get_mut_ren_database().get_mut_ren_mappings().add_addr(ren_id as i16, target as i16);
        },

        1 => {
            match pos.get_mut_ren_database().get_mut_ren_mappings().get_mut_ren(ren_id as i16) {
                Some(ren_obj) => {
                    if ren_obj.get_territory() == 3 {
                        // 占有者は白黒両方の場合、更新なし。
                    } else if ren_obj.get_territory() == 2 {
                        // 黒石 かつ、占有者が 黒、白の両方になる。
                        ren_obj.set_territory(3);
                    } else {
                        // 占有者は黒石か。
                        ren_obj.set_territory(1);
                    }
                },
                None => {}
            };

            // 隣は調べない。
            return;
        },

        2 => {
            match pos.get_mut_ren_database().get_mut_ren_mappings().get_mut_ren(ren_id as i16) {
                Some(ren_obj) => {
                    if ren_obj.get_territory() == 3 {
                        // 占有者は白黒両方の場合、更新なし。
                    } else if ren_obj.get_territory() == 1 {
                        // 白石 かつ、占有者が 黒、白の両方になる。
                        ren_obj.set_territory(3);
                    } else {
                        // 占有者は白石か。
                        ren_obj.set_territory(2);
                    }
                },
                None => {}
            };

            // 隣は調べない。
            return;
        },

        3 => {
            // 枠 --> 隣は調べない。
            return;
        },

        _ => {panic!("想定しない石の種類。 '{}'", pos.get_board().get_stone(target))},
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
pub fn cut_empty_ren(pos:&mut Position, cutting_addr:usize, address_ren_board_searcher:&mut AddressRenBoardSearcher) {
    // 空連ID。
    let empty_ren_id = pos.get_ren_database().get_address_empty_ren_board().get(cutting_addr);

    // 石を置いた交点から探索。
    println!("board size: {}.", pos.get_board().get_size());
    println!("cutting_addr: {}.", cutting_addr);
    pos.get_mut_ren_database().get_mut_ren_mappings().remove_addr(empty_ren_id, cutting_addr as i16);

    address_ren_board_searcher.count_up_mark();
    let mut shrink: Vec<i16> = Vec::new();

    {
        // 上側の空連を起点に探索し、一番小さい番地。
        let start = pos.get_board().get_top_of(cutting_addr);

        let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), empty_ren_id, start, cutting_addr);

        println!("空連{} の上を探索。", empty_ren_id);
        if min_addr == 0 {
            println!("空連なし。");
        } else if min_addr == empty_ren_id {
            // 連ID が被っている。
            print!("縮まった空連: {}, 番地: ", min_addr);
            shrink = address_ren_board_searcher.get_found_addr().to_vec();
        } else {
            print!("分かれた空連: {}, 番地: ", min_addr);
            show_vector_i16(&address_ren_board_searcher.get_found_addr());
        }
    }

    {
        // 右側の空連を起点に探索し、一番小さい番地。
        let start = pos.get_board().get_right_of(cutting_addr);
        let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), empty_ren_id, start, cutting_addr);
        println!("空連{} の右を探索。", empty_ren_id);        
        if min_addr == 0 {
            println!("空連なし。");
        } else if min_addr == empty_ren_id {
            // 連ID が被っている。
            print!("縮まった空連: {}, 番地: ", min_addr);
            shrink = address_ren_board_searcher.get_found_addr().to_vec();
        } else {
            print!("分かれた空連: {}, 番地: ", min_addr);
            show_vector_i16(&address_ren_board_searcher.get_found_addr());
        }
    }

    {
        // 下側の空連を起点に探索し、一番小さい番地。
        let start = pos.get_board().get_bottom_of(cutting_addr);
        let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), empty_ren_id, start, cutting_addr);
        println!("空連{} の下を探索。", empty_ren_id);        
        if min_addr == 0 {
            println!("空連なし。");
        } else if min_addr == empty_ren_id {
            // 連ID が被っている。
            print!("縮まった空連: {}, 番地: ", min_addr);
            shrink = address_ren_board_searcher.get_found_addr().to_vec();
        } else {
            print!("分かれた空連: {}, 番地: ", min_addr);
            show_vector_i16(&address_ren_board_searcher.get_found_addr());
        }
    }

    {
        // 左側の空連を起点に探索し、一番小さい番地。
        let start = pos.get_board().get_left_of(cutting_addr);
        let min_addr = address_ren_board_searcher.get_min_address(&pos.get_board(), &pos.get_ren_database().get_address_empty_ren_board(), empty_ren_id, start, cutting_addr);
        println!("空連{} の左を探索。", empty_ren_id);
        if min_addr == 0 {
            println!("空連なし。");
        } else if min_addr == empty_ren_id {
            // 連ID が被っている。
            print!("縮まった空連: {}, 番地: ", min_addr);
            shrink = address_ren_board_searcher.get_found_addr().to_vec();
        } else {
            print!("分かれた空連: {}, 番地: ", min_addr);
            show_vector_i16(&address_ren_board_searcher.get_found_addr());
        }
    }

    if !shrink.is_empty() {
        let old_territory = match pos.get_ren_database().get_ren_mappings().get_ren(empty_ren_id) {
            Some(ren_obj) => ren_obj.get_territory(),
            None => {println!("空連テリトリーの取得失敗。連ID: {}.", empty_ren_id); 0},
        };

        pos.get_mut_ren_database().get_mut_ren_mappings().remove_ren(empty_ren_id);
        pos.get_mut_ren_database().get_mut_ren_mappings().insert_ren(empty_ren_id, RenObject::default(empty_ren_id, shrink, old_territory));

        print!("縮まった空連の作り直し。番地: ");
        match &pos.get_mut_ren_database().get_mut_ren_mappings().get_ren(empty_ren_id) {
            Some(ren_obj) => show_ren_addr(ren_obj),
            None => {},
        };
        println!(".");
    }

    // TODO 空点の所有者の更新。
}