/// 局面☆（＾▽＾）

use board::Board;
use liberty_count_map::LibertyCountMap;
use ren_element_map::RenElementMap;
use ren_id_board::RenIDBoard;

pub struct Position {
    /// 枠付きの盤面。
    pub board: Board,
    /// コウの番地。
    pub ko: usize,
    /// 手番。1:黒、2:白。
    pub turn: i8,
    /// 何手目か。
    pub ply: usize,
    /// 計算用。盤上に紐づく連ID。
    pub ren_id_board: RenIDBoard,
    /// 計算用。連に紐づく呼吸点の数。
    pub liberty_count_map: LibertyCountMap,
    /// 計算用。連に紐づく番地のリスト。
    pub ren_element_map: RenElementMap,
}
impl Position {
    pub fn default(board_stones:Board, ko_address:usize, turn_count:i8, ply_count:usize) -> Position {
        Position {
            board: board_stones,
            ko: ko_address,
            turn: turn_count,
            ply: ply_count,
            ren_id_board: RenIDBoard::new(),
            liberty_count_map: LibertyCountMap::new(),
            ren_element_map: RenElementMap::new(),
        }
    }
}