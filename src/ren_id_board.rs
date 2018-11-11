// 連のIDが入った盤☆（＾～＾）

use std;

pub struct RenIDBoard {
    pub value: [i16; 21 * 21],
}
impl RenIDBoard {
    pub fn new() -> RenIDBoard {
        RenIDBoard {
            value: [0; 21 * 21]
        }
    }

    pub fn get(&self, index:usize) -> i16 {
        self.value[index]
    }

    pub fn set(&mut self, index:usize, id:i16) {
        self.value[index] = id;
    }

    pub fn iter(&self) -> std::slice::Iter<i16> {
        self.value.iter()
    }

}