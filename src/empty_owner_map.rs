/// 空連IDに占有者を紐づけるぜ☆（＾～＾）

use address_ren_board::AddressRenBoard;
use empty_ren_territory::*;
use ren_address_map::*;

pub struct EmptyOwnerMap {
    /// 計算用。探索中のマーク。盤上に紐づく空連ID。
    pub address_ren_board: AddressRenBoard,

    /// 空連IDに紐づく占有者。番地でアクセスするので、ボード形式で持つ☆（＾～＾）
    /// 空連の占有者は、以下のいずれか☆（＾～＾）
    /// 0. 未調査、または 隣接する石がない。
    /// 1. 黒石か枠のいずれかだけに隣接する。
    /// 2. 白石か枠のいずれかだけに隣接する。
    /// 3. 黒石と白石の両方に隣接する。
    territory: EmptyRenTerritory,

    /// 占有するスペース。連IDに、アドレスを紐づける。
    pub space: RenAddressMap,
}
impl EmptyOwnerMap {
    pub fn new() -> EmptyOwnerMap {
        EmptyOwnerMap {
            address_ren_board: AddressRenBoard::new(),
            territory: EmptyRenTerritory::new(),
            space: RenAddressMap::new(),
        }
    }

    pub fn get_territory(&self) -> &EmptyRenTerritory {
        &self.territory
    }

    pub fn get_mut_territory(&mut self) -> &mut EmptyRenTerritory {
        &mut self.territory
    }

    /// 目つぶしなら真。
    pub fn is_eye_filling(&self, color:i8, target:i16) -> bool {
        let ren_id = self.address_ren_board.get(target as usize);
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

        match self.space.get(ren_id as i16) {
            Some(n) => { 1==n.len() },
            None => { false },
        }
    }
}