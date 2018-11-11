/// 連と呼吸点の計算☆（＾～＾）

use board::Board;
use ren_element_map::RenElementMap;
use ren_id_board::RenIDBoard;

/// 4方向の空点を数えるだけ☆（＾～＾）
pub fn count_liberty_at_point(target:usize, board_size:usize, board:&Board) -> i8 {
    let top = target-(board_size+2); // 上の番地。
    let right = target+1; // 右。
    let bottom = target+(board_size+2); // 下。
    let left = target-1; // 左。

    let mut count = 0;
    if board.get(top) == 0 {
        count += 1;
    }
    if board.get(right) == 0 {
        count += 1;
    }
    if board.get(bottom) == 0 {
        count += 1;
    }
    if board.get(left) == 0 {
        count += 1;
    }

    count
}

/// 全部の交点に、連のIDを振る。
pub fn check_liberty_all_points(board_size:usize, board:&Board, ren_id_board:&mut RenIDBoard,
    liberty_count_map:&mut [i8;21*21], ren_element_map:&mut RenElementMap) {

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
        let color = board.get(start); // 開始地点にある石の色。この石と同じ色を探す。
        if color==1 || color==2 { // 黒石か白石だけ探せばいい☆（＾～＾）
            walk_liberty(start as i16, color, board_size, &board, ren_id_board, liberty_count_map, ren_element_map, start); // まず開始地点から。
        }
    }
}

/// 連にIDを振り、連の呼吸点も数える。
/// # Arguments.
/// * `ren_id_board` - 1000以上はtemporaryな数。
fn walk_liberty(ren_id:i16, color:i8, board_size:usize, board:&Board, ren_id_board:&mut RenIDBoard,
    liberty_count_map:&mut [i8;21*21], ren_element_map:&mut RenElementMap, target:usize){
    if board.get(target) == 0 && ren_id_board.get(target) != ren_id + 1000 { // 調べた先が空点で、まだ今回マークしていなければ。
        // println!("LIB: [{:3}] {:3}", ren_id, target);
        liberty_count_map[ren_id as usize] += 1;
    }
    
    if (ren_id_board.get(target) != 0 && ren_id_board.get(target) != ren_id + 1000) // 連IDが振られてたら終了。ただし「自分の連ID + 1000」を除く。0 は枠セル番号なんで、連IDに使わない。
        || // または、
        board.get(target) != color // 探している石でなければ終了。
    {
        if board.get(target) == 0 || 1000 <= ren_id_board.get(target) { // そこが空点か、1000以上の連IDなら「自分の連ID + 1000」を上書きでマークしておく。
            ren_id_board.set(target, ren_id + 1000);
        }
        return;
    }

    // 探している色の石なら 連ID を付ける。検索を開始したセル番号でも振っとく。
    ren_id_board.set(target, ren_id);
    if ren_id < 1000 && ren_element_map.contains_key(ren_id) {
        match ren_element_map.get_mut(ren_id) {
            Some(s) => {s.push(target as i16);}
            None => {panic!("walk_liberty");}
        }
    } else {
        let mut vec = Vec::new();
        vec.push(target as i16);
        ren_element_map.insert(ren_id, vec);
    }

    // 隣を探す。（再帰）
    walk_liberty(ren_id, color, board_size, &board, ren_id_board, liberty_count_map, ren_element_map, target-(board_size+2));// 上。
    walk_liberty(ren_id, color, board_size, &board, ren_id_board, liberty_count_map, ren_element_map, target+1);// 右。
    walk_liberty(ren_id, color, board_size, &board, ren_id_board, liberty_count_map, ren_element_map, target+(board_size+2));// 下。
    walk_liberty(ren_id, color, board_size, &board, ren_id_board, liberty_count_map, ren_element_map, target-1);// 左。
}
