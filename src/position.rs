/// 局面☆（＾▽＾）

pub struct Position {
    pub ply: usize,
    pub turn: i8,
    pub board: [i8; 21 * 21],
}
impl Position {
    pub fn default(ply:usize, turn:i8, board:[i8; 21 * 21]) -> Position {
        Position {
            ply: ply,
            turn: turn,
            board: board,
        }
    }
}