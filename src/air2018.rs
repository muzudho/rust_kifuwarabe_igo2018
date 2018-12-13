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

    /// エンドポイントを通った回数☆（＾～＾）スタート地点が 1、ゴールが 2 だぜ☆（＾～＾）
    count_end_point: i8,
}
impl Air2018 {
    pub fn new() -> Air2018 {
        Air2018 {
            bottle_neck_addr: HashSet::new(),
            count_board: [0; 21 * 21],
            count_end_point: 0,
        }
    }

    /// 通った回数をクリアー。
    pub fn clear_count_board(&mut self, pos:&Position) {
        for target in pos.get_left_top_on_board() ..= pos.get_right_bottom_on_board() {
            self.count_board[target] = 0;
        }

        self.count_end_point = 0;
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

                // 石の上からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_top_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の右上からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_top_right_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の右からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_right_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の右下からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_bottom_right_of(target) as i16;
                    self.search_by_right_hand(pos, end_point, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の下からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_bottom_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の左下からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_bottom_left_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の左からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_left_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }

                // 石の左上からスタート。
                for direction in 0..=3 {
                    self.clear_count_board(pos);
                    let end_point = pos.get_board().get_top_left_of(target) as i16;
                    self.search_by_right_hand(pos, end_point as i16, direction, end_point);
                    // if self.count_end_point != 2 {
                    // }
                    self.count_bottle_neck(pos);
                }
            }
        }

        let mut vec: Vec<usize> = Vec::new();

        // 着手禁止点以外は、全部合法手☆（*＾～＾*）
        println!("ボトルネック {0} 箇所。", self.bottle_neck_addr.len());
        for target in self.bottle_neck_addr.iter() {
            if !is_forbidden(*target as usize, pos, record) {
                vec.push(*target as usize);
            }
        }

        vec
    }

    /// 壁に右手を付けての探索。
    /// * `direction` - 顔の向き。 0:上、1:右、2:下、3:左。
    pub fn search_by_right_hand(&mut self, pos:&Position, end_point:i16, direction:i8, target:i16) {

        // 現在地の石の色。
        let current_stone = pos.get_board().get_stone(target as usize);

        if current_stone == 3 {
            // 現在地が枠なら、即終了。
            return;
        }

        // 壁となる石の色。（相手の石の色）
        let wall_stone = get_opponent(pos.turn);

        // 通り抜けれる石の色。（自分の石の色）
        let floor_stone = pos.turn;

        // 上を向いている。
        let top_direction = 0;

        // 右を向いている。
        let right_direction = 1;

        // 下を向いている。
        let bottom_direction = 2;

        // 左を向いている。
        let left_direction = 3;

        // 上の石の色。
        let top_stone = pos.get_board().get_stone(pos.get_board().get_top_of(target as usize));

        // 右上の石の色。
        let top_right_stone = pos.get_board().get_stone(pos.get_board().get_top_right_of(target as usize));

        // 右の石の色。
        let right_stone = pos.get_board().get_stone(pos.get_board().get_right_of(target as usize));

        // 右下の石の色。
        let bottom_right_stone = pos.get_board().get_stone(pos.get_board().get_bottom_right_of(target as usize));

        // 下の石の色。
        let bottom_stone = pos.get_board().get_stone(pos.get_board().get_bottom_of(target as usize));

        // 左下の石の色。
        let bottom_left_stone = pos.get_board().get_stone(pos.get_board().get_bottom_left_of(target as usize));

        // 左の石の色。
        let left_stone = pos.get_board().get_stone(pos.get_board().get_left_of(target as usize));

        // 左上の石の色。
        let top_left_stone = pos.get_board().get_stone(pos.get_board().get_top_left_of(target as usize));



        // println!("右手: {0}, 終点: {1}", target, end_point);
        if current_stone == 0 || current_stone == floor_stone {
            if self.count_end_point == 0 {
                // 始点は、区間の開とするので、無視する。
            }
            else
            {
                // 現在位置が空点 または 通れる石 なら、踏破扱い。
                self.count_board[target as usize] += 1;
            }
        }
        else if current_stone == wall_stone
        {
            // 現在位置が壁なら、終了。
            return;
        }

        if end_point == target {
            if self.count_end_point == 0 {
                // println!("始点: {0}", target);
                self.count_end_point += 1;
            } else {
                // println!("終点: {0}", target);
                self.count_end_point += 1;
                return;
            }
        }

        // 上、右、下、左 の順。
        match direction {
            0 => {
                // 上を向いている。
                // print!("（＾～＾）↑");

                // 右に壁があるか？
                if wall_stone == right_stone {
                    if top_stone == 0 || top_stone == floor_stone {
                        // 上が空いてるか、通れる石なら、上に進む。
                        // println!("足↑");
                        self.search_by_right_hand(pos, end_point as i16, top_direction, pos.get_board().get_top_of(target as usize) as i16);
                    }
                    else if left_stone == 0 || left_stone == floor_stone {
                        // 左が空いてるか、通れる石なら、左に進む。
                        // println!("足←");
                        self.search_by_right_hand(pos, end_point as i16, left_direction, pos.get_board().get_left_of(target as usize) as i16);
                    }
                    else
                    {
                        // そうでなければ、逆走。
                        // println!("◇↓逆走");
                        self.search_by_right_hand(pos, end_point as i16, bottom_direction, pos.get_board().get_bottom_of(target as usize) as i16);
                    }
                }
                else if bottom_right_stone == wall_stone && (right_stone == 0 || right_stone == floor_stone) {
                    // 右下が壁で、右が空いてるか、通れる石なら、右に進む。
                    // println!("手空。足→");
                    self.search_by_right_hand(pos, end_point as i16, right_direction, pos.get_board().get_right_of(target as usize) as i16);
                }
                else
                {
                    // println!("（／＿＼）");
                }
            },
            1 => {
                // 右を向いている。
                // print!("（＾～＾）→");

                // 下に壁があるか？
                if wall_stone == bottom_stone {
                    if right_stone == 0 || right_stone == floor_stone {
                        // 右が空いてるか、通れる石なら、右に進む。
                        // println!("足→");
                        self.search_by_right_hand(pos, end_point as i16, right_direction, pos.get_board().get_right_of(target as usize) as i16);
                    }
                    else if top_stone == 0 || top_stone == floor_stone {
                        // 上が空いてるか、通れる石なら、上に進む。
                        // println!("足↑");
                        self.search_by_right_hand(pos, end_point as i16, top_direction, pos.get_board().get_top_of(target as usize) as i16);
                    }
                    else
                    {
                        // そうでなければ、逆走。
                        // println!("◇←逆走");
                        self.search_by_right_hand(pos, end_point as i16, left_direction, pos.get_board().get_left_of(target as usize) as i16);
                    }
                }
                else if bottom_left_stone == wall_stone && (bottom_stone == 0 || bottom_stone == floor_stone) {
                    // 左下が壁で、下が空いてるか、通れる石なら、下に進む。
                    // println!("手空。足↓");
                    self.search_by_right_hand(pos, end_point as i16, bottom_direction, pos.get_board().get_bottom_of(target as usize) as i16);
                }
                else
                {
                    // println!("（／＿＼）");
                }
            },
            2 => {
                // 下を向いている。
                // print!("（＾～＾）↓");

                // 左に壁があるか？
                if wall_stone == left_stone {
                    if bottom_stone == 0 || bottom_stone == floor_stone {
                        // 下が空いてるか、通れる石なら、下に進む。
                        // println!("足↓");
                        self.search_by_right_hand(pos, end_point as i16, bottom_direction, pos.get_board().get_bottom_of(target as usize) as i16);
                    }
                    else if right_stone == 0 || right_stone == floor_stone {
                        // 右が空いてるか、通れる石なら、右に進む。
                        // println!("足→");
                        self.search_by_right_hand(pos, end_point as i16, right_direction, pos.get_board().get_right_of(target as usize) as i16);
                    }
                    else
                    {
                        // そうでなければ、逆走。
                        // println!("◇↑逆走");
                        self.search_by_right_hand(pos, end_point as i16, top_direction, pos.get_board().get_top_of(target as usize) as i16);
                    }
                }
                else if top_left_stone == wall_stone && (left_stone == 0 || left_stone == floor_stone) {
                    // 左上が壁で、左が空いてるか、通れる石なら、左に進む。
                    // println!("手空。足←");
                    self.search_by_right_hand(pos, end_point as i16, left_direction, pos.get_board().get_left_of(target as usize) as i16);
                }
                else
                {
                    // println!("（／＿＼）");
                }
            },
            _ => {
                // 左を向いている。
                // print!("（＾～＾）←");

                // 上に壁があるか？
                if wall_stone == top_stone {
                    if left_stone == 0 || left_stone == floor_stone {
                        // 左が空いてるか、通れる石なら、左に進む。
                        // println!("足←");
                        self.search_by_right_hand(pos, end_point as i16, left_direction, pos.get_board().get_left_of(target as usize) as i16);
                    }
                    else if bottom_stone == 0 || bottom_stone == floor_stone {
                        // 下が空いてるか、通れる石なら、下に進む。
                        // println!("足↓");
                        self.search_by_right_hand(pos, end_point as i16, bottom_direction, pos.get_board().get_bottom_of(target as usize) as i16);
                    }
                    else
                    {
                        // そうでなければ、逆走。
                        // println!("◇→逆走");
                        self.search_by_right_hand(pos, end_point as i16, right_direction, pos.get_board().get_right_of(target as usize) as i16);
                    }
                }
                else if top_right_stone == wall_stone && (top_stone == 0 || top_stone == floor_stone) {
                    // 右上が壁で、上が空いてるか、通れる石なら、上に進む。
                    // println!("手空。足↑");
                    self.search_by_right_hand(pos, end_point as i16, top_direction, pos.get_board().get_top_of(target as usize) as i16);
                }
                else
                {
                    // println!("（／＿＼）");
                }
            }
        }
    }
}
