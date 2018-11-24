/// 石の連に関するものだぜ☆（＾～＾）

use position::Position;

/// 連にIDを振り、連の呼吸点も数える。
/// # Arguments.
/// * `address_ren_board` - 1000以上はtemporaryな数。
pub fn walk_liberty(ren_id:i16, color:i8, pos:&mut Position, target:usize){
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