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

}