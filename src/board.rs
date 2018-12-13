/// 石を置いている盤だぜ☆（＾～＾）

use std;
use ren_db::ren_database::*;
use zobrist_hash::*;

/// プログラム中にインスタンスが１つだけ存在する☆（＾～＾） グローバル変数のような便利さで使っている☆（＾～＾）
/// デフォルトで 19路盤☆（＾～＾）
pub struct Board {
    /// 石の色は i8 の -128～127 で足りる☆（＾～＾）
    /// サイズは 19x19に枠を付けたものを最大とする☆（＾～＾）
    value: [i8; 21 * 21],
    /// 何路盤か。
    size: usize,
    /// ゾブリスト ハッシュ生成器。
    zobrist_hash: ZobristHash,
    /// ハッシュ。
    hash: u64,
}
impl Board {
    pub fn default(board_size:usize) -> Board {
        Board {
            value: [0; 21 * 21],
            size: board_size,
            zobrist_hash: ZobristHash::new(),
            hash: 0,
        }
    }

    pub fn get_stone(&self, index:usize) -> i8 {
        self.value[index]
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    /// 石を置く。
    /// - ゾブリストハッシュを再計算。
    pub fn set_stone(&mut self, index:usize, stone:i8) {
        self.hash = self.zobrist_hash.switch(self.hash, self.value[index], index as i16);
        self.value[index] = stone;
        self.hash = self.zobrist_hash.switch(self.hash, stone, index as i16);
    }

    pub fn iter(&self) -> std::slice::Iter<i8> {
        self.value.iter()
    }

    pub fn set_size(&mut self, size:usize){
        self.size = size;
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    /// 石を置く。石を除去したいときは stone を 0 にする。
    pub fn fill_by_vec(&mut self, addr_vec:&Vec<i16>, stone:i8) {
        for addr in addr_vec {
            self.set_stone(*addr as usize, stone);
        }
    }

    /// 石を置く。石を除去したいときは stone を 0 にする。
    pub fn fill_by_ren(&mut self, ren_obj:&PieceObject, stone:i8) {
        for addr in ren_obj.iter_addr() {
            self.set_stone(*addr as usize, stone);
        }
    }

    // 上隣の番地。
    pub fn get_top_of(&self, target:usize) -> usize {
        target-(self.size+2)
    }

    // 右上隣の番地。
    pub fn get_top_right_of(&self, target:usize) -> usize {
        target-(self.size+2)+1
    }

    // 右隣の番地。
    pub fn get_right_of(&self, target:usize) -> usize {
        target+1
    }

    // 右下隣の番地。
    pub fn get_bottom_right_of(&self, target:usize) -> usize {
        target+(self.size+2)+1
    }

    // 下隣の番地。
    pub fn get_bottom_of(&self, target:usize) -> usize {
        target+(self.size+2)
    }

    // 左下隣の番地。
    pub fn get_bottom_left_of(&self, target:usize) -> usize {
        target+(self.size+2)-1
    }

    // 左隣の番地。
    pub fn get_left_of(&self, target:usize) -> usize {
        target-1
    }

    // 左上隣の番地。
    pub fn get_top_left_of(&self, target:usize) -> usize {
        target-(self.size+2)-1
    }
}