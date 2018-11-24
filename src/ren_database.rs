/// 連をデータベース化したい☆（＾～＾）

use std;
use std::collections::HashMap;
use address_ren_board::*;

/// 石連と、空連に大きく分かれる☆（＾～＾）
pub struct RenDatabase {
    // 石連ID に紐づくもの。 連IDは 番地から作られる。 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    stone_ren_map: RenMap,
    // 空連ID に紐づくもの。
    empty_ren_map: RenMap,

    /// 計算用。盤上に紐づく連ID。
    address_stone_ren_board: AddressRenBoard,

    /// 計算用。探索中のマーク。盤上に紐づく空連ID。
    address_empty_ren_board: AddressRenBoard,

    /// 空連IDに紐づく占有者。番地でアクセスするので、ボード形式で持つ☆（＾～＾）
    /// 空連の占有者は、以下のいずれか☆（＾～＾）
    /// 0. 未調査、または 隣接する石がない。
    /// 1. 黒石か枠のいずれかだけに隣接する。
    /// 2. 白石か枠のいずれかだけに隣接する。
    /// 3. 黒石と白石の両方に隣接する。
    empty_ren_territory: [usize; 21*21],
}
impl RenDatabase {
    pub fn new() -> RenDatabase {
        RenDatabase {
            stone_ren_map: RenMap::new(),
            empty_ren_map: RenMap::new(),
            address_stone_ren_board: AddressRenBoard::new(),
            address_empty_ren_board: AddressRenBoard::new(),
            empty_ren_territory: [0; 21*21],
        }
    }

    pub fn get_stone_ren_map(&self) -> &RenMap {
        &self.stone_ren_map
    }

    pub fn get_mut_stone_ren_map(&mut self) -> &mut RenMap {
        &mut self.stone_ren_map
    }

    pub fn get_empty_ren_map(&self) -> &RenMap {
        &self.empty_ren_map
    }

    pub fn get_mut_empty_ren_map(&mut self) -> &mut RenMap {
        &mut self.empty_ren_map
    }

    pub fn get_address_stone_ren_board(&self) -> &AddressRenBoard {
        &self.address_stone_ren_board
    }

    pub fn get_mut_address_stone_ren_board(&mut self) -> &mut AddressRenBoard {
        &mut self.address_stone_ren_board
    }

    pub fn get_address_empty_ren_board(&self) -> &AddressRenBoard {
        &self.address_empty_ren_board
    }

    pub fn get_mut_address_empty_ren_board(&mut self) -> &mut AddressRenBoard {
        &mut self.address_empty_ren_board
    }


    pub fn get_empty_ren_territory_board(&self) -> [usize; 21*21] {
        self.empty_ren_territory
    }

    pub fn get_empty_ren_territory(&self, index:usize) -> usize {
        self.empty_ren_territory[index]
    }

    pub fn set_empty_ren_territory(&mut self, index:usize, empty_ren_territory:usize) {
        self.empty_ren_territory[index] = empty_ren_territory;
    }

    /// 表示用など。
    pub fn iter_empty_ren_territory(&self) -> std::slice::Iter<usize> {
        self.empty_ren_territory.iter()
    }

    /// キーを変更。
    pub fn change_key_empty_ren_territory(&mut self, ren_id_before:i16, ren_id_after:i16){
        self.empty_ren_territory[ren_id_after as usize] = self.empty_ren_territory[ren_id_before as usize];
        self.empty_ren_territory[ren_id_before as usize] = 0;
    }

    /// 目つぶしなら真。
    pub fn is_eye_filling(&self, color:i8, target:i16) -> bool {
        let ren_id = self.get_address_empty_ren_board().get(target as usize);
        if ren_id == 0 {
            return false;
        }

        let owner = self.get_empty_ren_territory(target as usize);
        if owner == 0 || owner == 3 {
            return false;
        }

        if owner as i8 != color {
            return false;
        }

        match self.get_empty_ren_map().get_ren(ren_id as i16) {
            Some(ren_obj) => { 1 == ren_obj.len_addr() },
            None => { false },
        }
    }
}


/// 連の大分類リスト。
pub struct RenMap {
    /// 連ID と、 連オブジェクト が紐づく。
    map: HashMap<i16,RenObject>,
}
impl RenMap {
    pub fn new() -> RenMap {
        RenMap {
            map: HashMap::new(),
        }
    }

    /// 連に番地を追加するぜ☆（＾～＾）
    pub fn add_addr(&mut self, ren_id:i16, addr:i16) {
        match self.map.get_mut(&ren_id) {
            Some(ren_obj) => {
                // 番地追加。
                ren_obj.add_addr(addr);
                return;
            },
            None => {},
        };

        // 無い連なら、新規作成。
        let mut ren_obj = RenObject::new();
        // 番地追加。
        ren_obj.add_addr(addr);
        self.map.insert(ren_id, ren_obj);
    }


    pub fn get_ren(&self, ren_id:i16) -> Option<&RenObject> {
        self.map.get(&ren_id)
    }

    pub fn get_mut_ren(&mut self, ren_id:i16) -> Option<&mut RenObject> {
        self.map.get_mut(&ren_id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<i16, RenObject> {
        self.map.iter()
    }

    pub fn contains_key(&self, ren_id:i16) -> bool {
        self.map.contains_key(&ren_id)
    }

    /// 外部から連を追加。
    pub fn insert_ren(&mut self, ren_id:i16, ren_obj:RenObject) {
        self.map.insert(ren_id, ren_obj);
    }

    /// 既存の連に、外部から連を結合。
    pub fn extend_ren(&mut self, ren_id:i16, other_ren_obj:RenObject) {
        match self.map.get_mut(&ren_id) {
            Some(ren_obj) => {ren_obj.extend(other_ren_obj);},
            None => {panic!("Extend: ren_id: {}.", ren_id)},
        };
    }

    // 連を除外。
    pub fn remove_ren(&mut self, ren_id:i16) -> Option<RenObject> {
        self.map.remove(&ren_id)
    }

    /// 指定した連から、指定した番地を除外する。
    pub fn remove_addr(&mut self, ren_id:i16, removing_addr:i16) {
        // println!("連{} の {}番地を除外。", ren_id, removing_addr);
        match self.get_mut_ren(ren_id) {
            Some(ren_obj) => {
                // println!("連{} はあった。 {}番地を除外。", ren_id, removing_addr);
                ren_obj.remove_addr(removing_addr);
            },
            None => {panic!("削除したかった番地がなかった。")},
        };
    }

    /// 連のIDを変更。
    pub fn change_key(&mut self, ren_id_before:i16, ren_id_after:i16){
        match self.map.remove(&ren_id_before) {
            Some(ren_obj) => {
                if self.contains_key(ren_id_after) {
                    // 既存なら、既存ベクターに追加。
                    self.extend_ren(ren_id_after, ren_obj);
                } else {
                    // 無ければ、ベクターを丸ごと移動。
                    self.insert_ren(ren_id_after, ren_obj);
                }
            },
            None => {panic!("ren_id_before: {}, ren_id_after: {}.", ren_id_before, ren_id_after);}
        };
    }
}




/// 連。
pub struct RenObject {
    /// 連ID。
    id: i16,

    /// 含む番地。
    addresses: Vec<i16>,
}
impl RenObject {
    pub fn new() -> RenObject {
        RenObject {
            id: 0,
            addresses: Vec::new(),
        }
    }

    pub fn default(ren_id:i16, member_addresses:Vec<i16>) -> RenObject {
        RenObject {
            id: ren_id,
            addresses: member_addresses,
        }
    }

    /// 番地を追加する。
    pub fn add_addr(&mut self, addr:i16) {
        self.addresses.push(addr);
    }

    /// 別の連の番地を追加する。
    pub fn extend(&mut self, other_ren_obj:RenObject) {
        self.addresses.extend(other_ren_obj.iter_addr().cloned());
    }

    pub fn get_id(&self) -> i16 {
        self.id
    }

    pub fn iter_addr(&self) -> std::slice::Iter<i16> {
        self.addresses.iter()
    }

    pub fn len_addr(&self) -> usize {
        self.addresses.len()
    }

    /// 指定の番地を除外する。
    pub fn remove_addr(&mut self, addr:i16) {
        let index = self.addresses.iter().position(|&r| r == addr).unwrap();
        self.addresses.remove(index);
    }

    pub fn to_addr_vec(&self) -> Vec<i16> {
        self.addresses.to_vec()
    }
}




