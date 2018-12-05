/// 連と呼吸点の計算☆（＾～＾）

use board::Board;
use ren_db::empty_ren::*;
use position::Position;
use ren_db::stone_ren::*;

/// 4方向の空点を数えるだけ☆（＾～＾）
pub fn count_liberty_at_point(target:usize, board:&Board) -> i8 {
    let top = target-(board.get_size()+2); // 上の番地。
    let right = target+1; // 右。
    let bottom = target+(board.get_size()+2); // 下。
    let left = target-1; // 左。

    let mut count = 0;
    if board.get_stone(top) == 0 {
        count += 1;
    }
    if board.get_stone(right) == 0 {
        count += 1;
    }
    if board.get_stone(bottom) == 0 {
        count += 1;
    }
    if board.get_stone(left) == 0 {
        count += 1;
    }

    count
}

/// 全部の交点に、連のIDを振る。
pub fn check_liberty_all_points(pos:&mut Position) {

    // TODO クリアーはしなくていいのか？
    // pos.get_territory().set(start, 0);

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

    for start in left_top ..= rigth_bottom { // 検索を開始するセルの番号。連のIDを決めるのにも使う。
        let color = pos.get_board().get_stone(start); // 開始地点にある石の色。この石と同じ色を探す。

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
