// 連IDが入った盤を探索するやつだぜ☆（＾～＾）
use std::cmp;
use board::*;
use ren_db::ren_database::*;

// アプリケーション１つに、インスタンスは１個だけ存在するぜ☆（＾～＾）
pub struct AddressRenBoardSearcher {
    // 盤面を全クリアーする時間がもったいないから、マークの方を変えるぜ☆（＾～＾）
    pub mark: i64,
    pub mark_board: [i64; 21 * 21],

    // 指定の連IDが入っているところだけを探す☆（＾ｑ＾）
    filter_ren: i16,

    // 探索して、通った番地☆（＾ｑ＾）
    found_addr: Vec<i16>,
}
impl AddressRenBoardSearcher {
    pub fn new() -> AddressRenBoardSearcher {
        AddressRenBoardSearcher {
            mark: 0,
            mark_board: [0; 21*21],
            filter_ren: 0,
            found_addr: Vec::new(),
        }
    }

    /// 探索する前に、新しいマークにしておくこと。
    pub fn count_up_mark(&mut self) {
        self.mark += 1;
    }

    /// 先に count_up_mark() を使うこと。
    /// 指定の 連ID が入っているところだけを探す。
    /// 
    /// 探索した番地のなかで、一番小さい番地 を返す。探索に失敗したら 0 を返す。
    /// # Arguments.
    /// * `first_mark_target` - 上下左右のうち、探索してほしくない方を先にマークしておくのに使う。
    pub fn get_min_address(&mut self, board:&Board, address_ren_board:&AddressRenBoard, ren_id:i16, start:usize, first_mark_target:usize) -> i16 {

        // 初期化。
        self.found_addr.clear();

        if ren_id != address_ren_board.get(start) {
            // 起点が、指定の連でなければ、探索失敗。
            return 0;
        }
        println!("address_ren_board.get({}): {}.", start, address_ren_board.get(start));

        self.filter_ren = ren_id;

        // 最初にマークしておくところ。
        self.mark_board[first_mark_target] = self.mark;

        // 今回マークしておくところ。
        self.mark_board[start] = self.mark;
        self.found_addr.push(start as i16);

        let mut min_addr = start as i16;

        // 上。
        min_addr = self.search_min_address(min_addr, board, address_ren_board, board.get_top_of(start));
        // 右。
        min_addr = self.search_min_address(min_addr, board, address_ren_board, board.get_right_of(start));
        // 下。
        min_addr = self.search_min_address(min_addr, board, address_ren_board, board.get_bottom_of(start));
        // 左。
        min_addr = self.search_min_address(min_addr, board, address_ren_board, board.get_left_of(start));

        if min_addr == i16::max_value() {
            // 一致なし。
            return 0;
        }

        min_addr
    }

    fn search_min_address(&mut self, min_addr:i16, board:&Board, address_ren_board:&AddressRenBoard, start:usize) -> i16 {
        if
            // 起点が、指定の連でなければ、探索失敗。
            self.filter_ren != address_ren_board.get(start)
            // または
            ||
            // 既に探索済なら無視。
            self.mark == self.mark_board[start]
        {
            return min_addr;
        }

        // 今回マークしておくところ。
        self.mark_board[start] = self.mark;
        self.found_addr.push(start as i16);

        let mut temp_min_addr = cmp::min(min_addr, start as i16);

        // 再帰。
        // 上。
        temp_min_addr = self.search_min_address(temp_min_addr, board, address_ren_board, board.get_top_of(start));
        // 右。
        temp_min_addr = self.search_min_address(temp_min_addr, board, address_ren_board, board.get_right_of(start));
        // 下。
        temp_min_addr = self.search_min_address(temp_min_addr, board, address_ren_board, board.get_bottom_of(start));
        // 左。
        self.search_min_address(temp_min_addr, board, address_ren_board, board.get_left_of(start))
    }

    pub fn get_found_addr(&self) -> &Vec<i16> {
        &self.found_addr
    }
}