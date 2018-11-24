/// 局面☆（＾▽＾）

use board::Board;
use empty_owner_map::EmptyOwnerMap;
use liberty_count_map::LibertyCountMap;
use ren_address_map::RenAddressMap;
use address_ren_board::AddressRenBoard;

/// ゲーム中にインスタンスが１つだけ存在する☆（＾～＾） グローバル変数みたいな便利さで使っている☆（＾～＾）
pub struct Position {
    /// 枠付きの盤面。
    pub board: Board,
    /// コウの番地。
    pub ko: i16,
    /// 手番。1:黒、2:白。
    pub turn: i8,
    /// 計算用。盤上に紐づく連ID。
    pub address_ren_board: AddressRenBoard,
    /// 計算用。連に紐づく呼吸点の数。
    pub liberty_count_map: LibertyCountMap,
    /// 計算用。連に紐づく番地のリスト。
    pub ren_address_map: RenAddressMap,
    /// 計算用。空連の占有者。 0:未調査、1:黒、2:白、3:黒白両方。
    pub empty_owner_map: EmptyOwnerMap,
}
impl Position {
    pub fn default(board_stones:Board, ko_address:i16, turn_count:i8) -> Position {
        Position {
            board: board_stones,
            ko: ko_address,
            turn: turn_count,
            address_ren_board: AddressRenBoard::new(),
            liberty_count_map: LibertyCountMap::new(),
            ren_address_map: RenAddressMap::new(),
            empty_owner_map: EmptyOwnerMap::new(),
        }
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