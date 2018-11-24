/// 局面☆（＾▽＾）

use board::Board;
use liberty_count_map::LibertyCountMap;
use ren_database::*;
use address_ren_board::AddressRenBoard;
use address_ren_board_searcher::*;
use empty_ren_territory::*;

/// ゲーム中にインスタンスが１つだけ存在する☆（＾～＾） グローバル変数みたいな便利さで使っている☆（＾～＾）
pub struct Position {
    /// 枠付きの盤面。
    pub board: Board,
    /// コウの番地。
    pub ko: i16,
    /// 手番。1:黒、2:白。
    pub turn: i8,
    /// 計算用。連に紐づく呼吸点の数。
    pub liberty_count_map: LibertyCountMap,
    
    /// 連のデータベース☆（＾～＾）
    ren_database: RenDatabase,
}
impl Position {
    pub fn default(board_stones:Board, ko_address:i16, turn_count:i8) -> Position {
        Position {
            board: board_stones,
            ko: ko_address,
            turn: turn_count,
            liberty_count_map: LibertyCountMap::new(),
            ren_database: RenDatabase::new(),
        }
    }
    
    pub fn get_ren_database(&self) -> &RenDatabase {
        &self.ren_database
    }
    
    pub fn get_mut_ren_database(&mut self) -> &mut RenDatabase {
        &mut self.ren_database
    }

    /// 枠の中の左上隅から右下隅まで検索☆（＾～＾）
    /// 小さい盤で数えてみろだぜ☆（＾～＾）
    ///
    /// ++++
    /// +  +
    /// +  +
    /// ++++
    ///
    /// は board_size 2 で、セル番号は
    ///
    ///  0, 1, 2, 3,
    ///  4, 5, 6, 7,
    ///  8, 9,10,11,
    /// 12,13,14,15
    ///
    /// だから、枠の中の 左上隅は 5、右下隅は 10 で、算出方法は以下の通り☆
    pub fn get_left_top_on_board(&self) -> usize {
        (self.board.get_size()+2) + 1
    }

    /// 枠の中の右下隅の番地。
    pub fn get_right_bottom_on_board(&self) -> usize {
        (self.board.get_size()+2) * self.board.get_size() + self.board.get_size()
    }

    /// 上隣の番地。
    pub fn get_top_of(&self, target:usize) -> usize {
        target-(self.board.get_size()+2)
    }

    /// 下隣の番地。
    pub fn get_bottom_of(&self, target:usize) -> usize {
        target+(self.board.get_size()+2)
    }
}