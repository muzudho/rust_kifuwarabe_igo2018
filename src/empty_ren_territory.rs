/// 空連の所有者☆（＾～＾） 空連IDと所有者を紐づける☆（＾～＾）
/// 所有者は、以下のいずれか☆（＾～＾）
/// 0. 未調査、または 隣接する石がない。
/// 1. 黒石か枠のいずれかだけに隣接する。
/// 2. 白石か枠のいずれかだけに隣接する。
/// 3. 黒石と白石の両方に隣接する。

use std;

pub struct EmptyRenTerritory {
    /// 空連IDに紐づく占有者。番地でアクセスするので、ボード形式で持つ☆（＾～＾）
    /// 空連の占有者は、以下のいずれか☆（＾～＾）
    /// 0. 未調査、または 隣接する石がない。
    /// 1. 黒石か枠のいずれかだけに隣接する。
    /// 2. 白石か枠のいずれかだけに隣接する。
    /// 3. 黒石と白石の両方に隣接する。
    owner: [usize; 21*21],
}
impl EmptyRenTerritory {
    pub fn new() -> EmptyRenTerritory {
        EmptyRenTerritory {
            owner: [0; 21*21],
        }
    }

    pub fn get_owner(&self, index:usize) -> usize {
        self.owner[index]
    }

    pub fn set_owner(&mut self, index:usize, empty_owner:usize) {
        self.owner[index] = empty_owner;
    }

    /// 表示用など。
    pub fn iter_owner(&self) -> std::slice::Iter<usize> {
        self.owner.iter()
    }

    /// キーを変更。
    pub fn change_key(&mut self, ren_id_before:i16, ren_id_after:i16){
        self.owner[ren_id_after as usize] = self.owner[ren_id_before as usize];
        self.owner[ren_id_before as usize] = 0;
    }
}