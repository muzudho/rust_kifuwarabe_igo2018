/// 局面☆（＾▽＾）

use board::Board;
use liberty_count_map::LibertyCountMap;
use ren_element_map::RenElementMap;
use ren_id_board::RenIDBoard;

pub struct Position {
    pub ply: usize,
    pub turn: i8,
    pub board: Board,
    pub ren_id_board: RenIDBoard,
    pub liberty_count_map: LibertyCountMap,
    // 連に紐づく番地のリスト。
    pub ren_element_map: RenElementMap,
}
impl Position {
    pub fn default(ply_count:usize, turn_count:i8, board_stones:Board) -> Position {
        Position {
            ply: ply_count,
            turn: turn_count,
            board: board_stones,
            ren_id_board: RenIDBoard::new(),
            liberty_count_map: LibertyCountMap::new(),
            ren_element_map: RenElementMap::new(),
        }
    }
}