/// 石を置いている盤だぜ☆（＾～＾）

use std;

/// デフォルトで 19路盤☆（＾～＾）
pub struct Board {
    /// 石の色は i8 の -128～127 で足りる☆（＾～＾）
    /// サイズは 19x19に枠を付けたものを最大とする☆（＾～＾）
    pub value: [i8; 21 * 21],
    /// 何路盤か。
    size: usize,
}
impl Board {
    pub fn default(board_size:usize) -> Board {
        Board {
            value: [0; 21 * 21],
            size: board_size,
        }
    }

    pub fn get(&self, index:usize) -> i8 {
        self.value[index]
    }

    pub fn set(&mut self, index:usize, stone:i8) {
        self.value[index] = stone;
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
    pub fn fill(&mut self, addr_vec:&Vec<i16>, stone:i8) {
        for addr in addr_vec {
            self.value[*addr as usize] = stone;
        }
    }

}