/// 局面☆（＾▽＾）

pub struct Position {
    pub ply: usize,
    pub turn: i8,
    pub board: [i8; 21 * 21],
}
impl Position {
    pub fn default(ply_count:usize, turn_count:i8, board_stones:[i8; 21 * 21]) -> Position {
        Position {
            ply: ply_count,
            turn: turn_count,
            board: board_stones,
        }
    }
}