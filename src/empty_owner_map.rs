/// 空連IDに占有者を紐づけるぜ☆（＾～＾）

use std;
use ren_address_map::*;
use address_ren_board::AddressRenBoard;

pub struct EmptyOwnerMap {
    /// 計算用。盤上に紐づく空連ID。
    pub address_ren_board: AddressRenBoard,

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
            address_ren_board: AddressRenBoard::new(),
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

    /// 目つぶしなら真。
    pub fn is_eye_filling(&self, color:i8, target:i16) -> bool {
        let ren_id = self.address_ren_board.get(target as usize);
        if ren_id == 0 {
            return false;
        }

        let owner = self.get(target as usize);
        if owner == 0 || owner == 3 {
            return false;
        }

        if owner as i8 != color {
            return false;
        }

        match self.space.get(ren_id as i16) {
            Some(n) => { 1==n.len() },
            None => { false },
        }
    }
}