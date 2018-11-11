/// 石を置いている盤だぜ☆（＾～＾）

use std;

pub struct Board {
    /// 石は i8 の -128～127 で足りる☆（＾～＾）
    pub value: [i8; 21 * 21],
}
impl Board {
    pub fn new() -> Board {
        Board {
            value: [0; 21 * 21],
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

    /// 石を置く。石を除去したいときは stone を 0 にする。
    pub fn fill(&mut self, addr_vec:&Vec<i16>, stone:i8) {
        for addr in addr_vec {
            self.value[*addr as usize] = stone;
        }
    }

}