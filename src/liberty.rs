/// 連と呼吸点の計算☆（＾～＾）

use board::Board;
use position::Position;

/// 4方向の空点を数えるだけ☆（＾～＾）
pub fn count_liberty_at_point(target:usize, board:&Board) -> i8 {
    let top = target-(board.get_size()+2); // 上の番地。
    let right = target+1; // 右。
    let bottom = target+(board.get_size()+2); // 下。
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
pub fn check_liberty_all_points(pos:&mut Position) {

    // TODO クリアーはしなくていいのか？
    // pos.empty_owner_map.set(start, 0);

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
    let left_top = pos.get_left_top_on_board();
    let rigth_bottom = pos.get_right_bottom_on_board();

    for start in left_top..rigth_bottom+1 { // 検索を開始するセルの番号。連のIDを決めるのにも使う。
        let color = pos.board.get(start); // 開始地点にある石の色。この石と同じ色を探す。

        // 黒石か白石は、呼吸点を探す。
        if color==1 || color==2 {
            // まず開始地点から。
            walk_liberty(start as i16, color, pos, start);
        } else if color==0 {
            // 空連の占有者は、以下のいずれか。
            // 0. 未調査、または 隣接する石がない。
            // 1. 黒石か枠のいずれかだけに隣接する。
            // 2. 白石か枠のいずれかだけに隣接する。
            // 3. 黒石と白石の両方に隣接する。
            walk_empty(start, pos, start);
        }
    }
}

/// 連にIDを振り、連の呼吸点も数える。
/// # Arguments.
/// * `address_ren_board` - 1000以上はtemporaryな数。
fn walk_liberty(ren_id:i16, color:i8, pos:&mut Position, target:usize){
    // 空点 かつ、まだ今回マークしていない --> 呼吸点+1。
    if pos.board.get(target) == 0 && pos.address_ren_board.get(target) != ren_id + 1000 {
        pos.liberty_count_map.add(ren_id as usize, 1);
    }
    
    if (
        // 連IDが振られてたら終了。ただし「自分の連ID + 1000」を除く。0 は枠セル番号なんで、連IDに使わない。
        pos.address_ren_board.get(target) != 0 && pos.address_ren_board.get(target) != ren_id + 1000)
        || // または、
        // 探している石でなければ終了。
        pos.board.get(target) != color
    {
        // 空点 --> 「自分の連ID + 1000」で ID上書き
        // 1000以上の連ID --> 同上。
        if pos.board.get(target) == 0 || 1000 <= pos.address_ren_board.get(target) {
            pos.address_ren_board.set(target, ren_id + 1000);
        }
        return;
    }

    // 探している色の石なら 連ID を付ける。検索を開始したセル番号でも振っとく。
    pos.address_ren_board.set(target, ren_id);
    if ren_id < 1000 && pos.ren_address_map.contains_key(ren_id) {
        match pos.ren_address_map.get_mut(ren_id) {
            Some(s) => {s.push(target as i16);}
            None => {panic!("walk_liberty");}
        }
    } else {
        let mut vec = Vec::new();
        vec.push(target as i16);
        pos.ren_address_map.insert(ren_id, vec);
    }

    // 隣を探す。（再帰）
    let top = pos.get_top_of(target);
    let bottom = pos.get_bottom_of(target);
    walk_liberty(ren_id, color, pos, top);// 上。
    walk_liberty(ren_id, color, pos, target+1);// 右。
    walk_liberty(ren_id, color, pos, bottom);// 下。
    walk_liberty(ren_id, color, pos, target-1);// 左。
}

/// 空連にIDを振り、空連の占有者も調べる。
fn walk_empty(ren_id:usize, pos:&mut Position, target:usize) {
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
