/// 空連IDに占有者を紐づけるぜ☆（＾～＾）

use std;

pub struct EmptyOwnerMap {
    // 空連の占有者は、以下のいずれか☆（＾～＾）
    // 0. 未調査、または 隣接する石がない。
    // 1. 黒石か枠のいずれかだけに隣接する。
    // 2. 白石か枠のいずれかだけに隣接する。
    // 3. 黒石と白石の両方に隣接する。
    pub value: [usize; 21*21],
}
impl EmptyOwnerMap {
    pub fn new() -> EmptyOwnerMap {
        EmptyOwnerMap {
            value: [0; 21*21],
        }
    }

    pub fn get(&self, index:usize) -> usize {
        self.value[index]
    }

    pub fn set(&mut self, index:usize, empty_owner:usize) {
        self.value[index] = empty_owner;
    }

    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.value.iter()
    }

    /// キーを変更。
    pub fn change_key(&mut self, ren_id_before:i16, ren_id_after:i16){
        self.value[ren_id_after as usize] = self.value[ren_id_before as usize];
        self.value[ren_id_before as usize] = 0;
    }

}