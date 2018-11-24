/// 空連の所有者☆（＾～＾） 空連IDと所有者を紐づける☆（＾～＾）
/// 所有者は、以下のいずれか☆（＾～＾）
/// 0. 未調査、または 隣接する石がない。
/// 1. 黒石か枠のいずれかだけに隣接する。
/// 2. 白石か枠のいずれかだけに隣接する。
/// 3. 黒石と白石の両方に隣接する。

use std;
use std::collections::HashMap;

pub struct EmptyRenTerritory {
    // 連ID に紐づく、所有者☆（＾～＾）
    pub value: HashMap<i16,i8>
}
impl EmptyRenTerritory {
    pub fn new()-> EmptyRenTerritory {
        EmptyRenTerritory {
            value: HashMap::new(),
        }
    }

    pub fn set(&mut self, ren_id:i16, owner:i8) {
        self.value.insert(ren_id, owner);
    }

    pub fn get(&self, ren_id:i16) -> Option<&i8> {
        self.value.get(&ren_id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<i16, i8> {
        self.value.iter()
    }

    pub fn contains_key(&self, ren_id:i16) -> bool {
        self.value.contains_key(&ren_id)
    }

    pub fn remove(&mut self, ren_id:i16) -> Option<i8> {
        self.value.remove(&ren_id)
    }

    /// キーを変更。
    pub fn change_key(&mut self, ren_id_before:i16, ren_id_after:i16){
        match self.remove(ren_id_before) {
            Some(owner) => {
                if self.contains_key(ren_id_after) {
                    panic!("キーを変更しようとしたら、既存だった。");
                } else {
                    // 無ければ、ベクターを丸ごと移動。
                    self.set(ren_id_after, owner);
                }
            },
            None => {panic!("ren_id_before: {}, ren_id_after: {}.", ren_id_before, ren_id_after);}
        };
    }
}