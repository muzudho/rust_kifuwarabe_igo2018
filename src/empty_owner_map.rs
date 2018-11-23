/// 空連IDに占有者を紐づけるぜ☆（＾～＾）

use std;
use ren_address_map::*;

pub struct EmptyOwnerMap {
    /// 空連の占有者は、以下のいずれか☆（＾～＾）
    /// 0. 未調査、または 隣接する石がない。
    /// 1. 黒石か枠のいずれかだけに隣接する。
    /// 2. 白石か枠のいずれかだけに隣接する。
    /// 3. 黒石と白石の両方に隣接する。
    pub owner: [usize; 21*21],

    /// 占有するスペース。連IDに、アドレスを紐づける。
    pub space: RenAddressMap,
}
impl EmptyOwnerMap {
    pub fn new() -> EmptyOwnerMap {
        EmptyOwnerMap {
            owner: [0; 21*21],
            space: RenAddressMap::new(),
        }
    }

    pub fn get(&self, index:usize) -> usize {
        self.owner[index]
    }

    pub fn set(&mut self, index:usize, empty_owner:usize) {
        self.owner[index] = empty_owner;
    }

    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.owner.iter()
    }

    /// キーを変更。
    pub fn change_key(&mut self, ren_id_before:i16, ren_id_after:i16){
        self.owner[ren_id_after as usize] = self.owner[ren_id_before as usize];
        self.owner[ren_id_before as usize] = 0;
    }

}