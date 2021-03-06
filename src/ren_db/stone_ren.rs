/// 石の連に関するものだぜ☆（＾～＾）

use position::Position;
use ren_db::ren_database::*;

/// 連にIDを振り、連の呼吸点も数える。
/// # Arguments.
/// * `piece_distribution` - 1000以上はtemporaryな数。
pub fn walk_liberty(piece_id:i16, color:i8, pos:&mut Position, target:usize){
    // 空点 かつ、まだ今回マークしていない --> 呼吸点+1。
    if pos.get_board().get_stone(target) == 0 && pos.get_piece_database().get_stone_piece_distribution().get(target) != piece_id + 1000 {
        pos.get_mut_ren_database().get_mut_ren_mappings().get_mut_ren(piece_id).expect("walk_liberty(1)").add_liberty_count(1);
    }
    
    if (
        // 連IDが振られてたら終了。ただし「自分の連ID + 1000」を除く。0 は枠セル番号なんで、連IDに使わない。
        pos.get_piece_database().get_stone_piece_distribution().get(target) != 0 && pos.get_piece_database().get_stone_piece_distribution().get(target) != piece_id + 1000)
        || // または、
        // 探している石でなければ終了。
        pos.get_board().get_stone(target) != color
    {
        // 空点 --> 「自分の連ID + 1000」で ID上書き
        // 1000以上の連ID --> 同上。
        if pos.get_board().get_stone(target) == 0 || 1000 <= pos.get_piece_database().get_stone_piece_distribution().get(target) {
            pos.get_mut_ren_database().get_mut_stone_piece_distribution().set(target, piece_id + 1000);
        }
        return;
    }

    // 探している色の石なら
    // 番地に 連ID を紐づける。検索を開始したセル番号でも振っとく。
    pos.get_mut_ren_database().get_mut_stone_piece_distribution().set(target, piece_id);

    if piece_id < 1000 && pos.get_piece_database().get_piece_mappings().contains_key(piece_id) {
        match pos.get_mut_ren_database().get_mut_ren_mappings().get_mut_ren(piece_id) {
            Some(ren_obj) => {ren_obj.add_addr(target as i16);}
            None => {panic!("walk_liberty");}
        }
    } else {
        let old_territory = match pos.get_piece_database().get_piece_mappings().get_piece(piece_id) {
            Some(ren_obj) => ren_obj.get_territory(),
            None => {
                // TODO ここはよく通るが何だろうか？
                println!("石連テリトリーの取得失敗。連ID: {}.", piece_id);
                0
            },
        };

        pos.get_mut_ren_database().get_mut_ren_mappings().insert_ren(piece_id, PieceObject::default(piece_id, vec![target as i16], old_territory));
    }

    // 隣を探す。（再帰）
    let top = pos.get_top_of(target);
    let bottom = pos.get_bottom_of(target);
    walk_liberty(piece_id, color, pos, top);// 上。
    walk_liberty(piece_id, color, pos, target+1);// 右。
    walk_liberty(piece_id, color, pos, bottom);// 下。
    walk_liberty(piece_id, color, pos, target-1);// 左。
}

/// TODO 石の連の接続☆（＾～＾）
/// --> 新しい連が1つ増える。
/// --> 既存の1～4つの連に番地が1つ増える。
/// # Arguments.
/// * 'connecting_addr' - 石を置いて、連がつながるところ。
pub fn connect_stone_ren(pos:&mut Position, color:i8, connecting_addr:usize) {
    // 石の連ID。

    // 最小のIDの方に統合する。

    // 上隣の同色の連ID。
    {
        let adjacent = pos.get_top_of(connecting_addr);
        // pos.get_board().
    }

    // 右隣の同色の連ID。
    {
        let adjacent = pos.get_right_of(connecting_addr);

    }

    // 下隣の同色の連ID。
    {
        let adjacent = pos.get_bottom_of(connecting_addr);

    }

    // 左隣の同色の連ID。
    {
        let adjacent = pos.get_left_of(connecting_addr);

    }

    // 呼吸点の数え直し。
}