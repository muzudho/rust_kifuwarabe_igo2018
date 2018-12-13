use std::collections::HashSet;
use *;
use position::Position;
use record::Record;

/// AI竜星戦2018 仕様。
pub struct Air2018 {
    /// ボトルネックになっている番地。
    bottle_neck_addr: HashSet<i16>,

    /// 通った回数☆（＾～＾）
    count_board: [i8; 21 * 21],
}
impl Air2018 {
    pub fn new() -> Air2018 {
        Air2018 {
            bottle_neck_addr: HashSet::new(),
            count_board: [0; 21 * 21],
        }
    }

    /// 通った回数をクリアー。
    pub fn clear_count_board(&mut self, pos:&Position) {
        for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
            self.count_board[target] = 0;
        }
    }

    /// 2回以上通った所をボトルネックとする。
    pub fn count_bottle_neck(&mut self, pos:&Position) {
        for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
            if 1 < self.count_board[target] {
                self.bottle_neck_addr.insert(target as i16);
            }
        }
    }

    /// 指定局面での合法手生成。
    pub fn pick_move_air2018(&mut self, pos:&Position, record:&Record) -> Vec<usize> {
        // 相手の石の色。
        let opponent = get_opponent(pos.turn);

        // 相手の石について。
        for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
            if opponent == pos.get_board().get_stone(target as usize) {
                // 上
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_top_of(target) as i16);
                self.count_bottle_neck(pos);

                // 右上
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_top_right_of(target) as i16);
                self.count_bottle_neck(pos);

                // 右
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_right_of(target) as i16);
                self.count_bottle_neck(pos);

                // 右下
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_bottom_right_of(target) as i16);
                self.count_bottle_neck(pos);

                // 下
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_bottom_of(target) as i16);
                self.count_bottle_neck(pos);

                // 左下
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_bottom_left_of(target) as i16);
                self.count_bottle_neck(pos);

                // 左
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_left_of(target) as i16);
                self.count_bottle_neck(pos);

                // 左上
                self.clear_count_board(pos);
                self.search_by_right_hand(pos, 0, pos.get_board().get_top_left_of(target) as i16);
                self.count_bottle_neck(pos);
            }
        }

        let mut vec: Vec<usize> = Vec::new();

        // 着手禁止点以外は、全部合法手☆（*＾～＾*）
        for target in self.bottle_neck_addr.iter() {
            if !is_forbidden(*target as usize, pos, record) {
                vec.push(*target as usize);
            }
        }

        vec
    }

    /// 壁に右手を付けての探索。
    /// * `direction` - 顔の向き。 0:上、1:右、2:下、3:左。
    pub fn search_by_right_hand(&mut self, pos:&Position, direction:i8, target:i16) {
        println!("右手: {0}", target);
        match pos.get_board().get_stone(target as usize) {
            0 => {
                // 現在位置が空点なら、踏破扱い。
                self.count_board[target as usize] += 1;
            },
            _ => {
                // 現在位置が石、枠なら、終了。
                return;
            },
        }

        // 相手の石の色。
        let opponent = get_opponent(pos.turn);

        match direction {
            0 => {
                // 上を向いている。

                // 右に自分の色の石があるか？
                if pos.turn == pos.get_board().get_stone(pos.get_board().get_right_of(target as usize)) {
                    // 上に進む。
                    self.search_by_right_hand(pos, 0, pos.get_board().get_top_of(target as usize) as i16);
                }
            },
            1 => {
                // 右を向いている。

                // 下に自分の色の石があるか？
                if pos.turn == pos.get_board().get_stone(pos.get_board().get_bottom_of(target as usize)) {
                    // 右に進む。
                    self.search_by_right_hand(pos, 0, pos.get_board().get_right_of(target as usize) as i16);
                }
            },
            2 => {
                // 下を向いている。

                // 左に自分の色の石があるか？
                if pos.turn == pos.get_board().get_stone(pos.get_board().get_left_of(target as usize)) {
                    // 下に進む。
                    self.search_by_right_hand(pos, 0, pos.get_board().get_bottom_of(target as usize) as i16);
                }
            },
            _ => {
                // 左を向いている。

                // 上に自分の色の石があるか？
                if pos.turn == pos.get_board().get_stone(pos.get_board().get_top_of(target as usize)) {
                    // 左に進む。
                    self.search_by_right_hand(pos, 0, pos.get_board().get_left_of(target as usize) as i16);
                }
            }
        }
    }
}
