/// 空連IDに占有者を紐づけるぜ☆（＾～＾）

use address_ren_board::AddressRenBoard;
use empty_ren_territory::*;
use ren_database::*;

pub struct EmptyOwnerMap {
    /// 空連IDに紐づく占有者。番地でアクセスするので、ボード形式で持つ☆（＾～＾）
    /// 空連の占有者は、以下のいずれか☆（＾～＾）
    /// 0. 未調査、または 隣接する石がない。
    /// 1. 黒石か枠のいずれかだけに隣接する。
    /// 2. 白石か枠のいずれかだけに隣接する。
    /// 3. 黒石と白石の両方に隣接する。
    territory: EmptyRenTerritory,
}
impl EmptyOwnerMap {
    pub fn new() -> EmptyOwnerMap {
        EmptyOwnerMap {
            territory: EmptyRenTerritory::new(),
        }
    }

    pub fn get_territory(&self) -> &EmptyRenTerritory {
        &self.territory
    }

    pub fn get_mut_territory(&mut self) -> &mut EmptyRenTerritory {
        &mut self.territory
    }

    /// 目つぶしなら真。
    pub fn is_eye_filling(&self, color:i8, target:i16, ren_database:&RenDatabase) -> bool {
        let ren_id = ren_database.get_address_empty_ren_board().get(target as usize);
        if ren_id == 0 {
            return false;
        }

        let owner = self.territory.get_owner(target as usize);
        if owner == 0 || owner == 3 {
            return false;
        }

        if owner as i8 != color {
            return false;
        }

        match ren_database.get_empty_ren_map().get_ren(ren_id as i16) {
            Some(ren_obj) => { 1 == ren_obj.len_addr() },
            None => { false },
        }
    }
}