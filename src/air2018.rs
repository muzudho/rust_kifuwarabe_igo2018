use *;
use position::Position;
use record::Record;

/// AI竜星戦2018 仕様。
pub struct Air2018 {
    /// ボトルネックになっている番地。
    bottle_neck_addr_vec: Vec<i16>,

    /// 通った回数☆（＾～＾）
    count_board: [i8; 21 * 21],
}
impl Air2018 {
    pub fn new() -> Air2018 {
        Air2018 {
            bottle_neck_addr_vec: Vec::new(),
            count_board: [0; 21 * 21],
        }
    }

    /// 指定局面での合法手生成。
    pub fn pick_move_air2018(&mut self, pos:&Position, record:&Record) -> Vec<usize> {
        let mut vec: Vec<usize> = Vec::new();

        // 着手禁止点以外は、全部合法手☆（*＾～＾*）
        for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
            if !is_forbidden(target, pos, record) {
                vec.push(target);
            }
        }

        // 相手の石の色。
        let opponent = get_opponent(pos.turn);

        // 相手の石について。
        for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
            if opponent == pos.get_board().get_stone(target as usize) {
                self.search_by_right_hand(target as i16);
            }
        }

        vec
    }

    /// 壁に右手を付けての探索。
    pub fn search_by_right_hand(&mut self, start:i16) {
        
    }
}
