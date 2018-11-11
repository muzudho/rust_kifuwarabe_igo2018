/// 局面☆（＾▽＾）

use std::collections::HashMap;

pub struct Position {
    pub ply: usize,
    pub turn: i8,
    pub board: [i8; 21 * 21],
    pub ren_id_board: [i16; 21 * 21],
    pub liberty_count_map: [i8; 21*21],
    // 19路盤は 361交点あるので、i8 の -128～127 では足りない☆（＾～＾）
    pub ren_element_map: HashMap<i8,Vec<i16>>
}
impl Position {
    pub fn default(ply_count:usize, turn_count:i8, board_stones:[i8; 21 * 21]) -> Position {
        Position {
            ply: ply_count,
            turn: turn_count,
            board: board_stones,
            ren_id_board: [0; 21 * 21],
            liberty_count_map: [0; 21*21],
            ren_element_map: HashMap::new(),
        }
    }
}