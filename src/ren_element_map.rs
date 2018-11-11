/// 連ID に紐づく、石の番地のリスト。
/// ハッシュマップの要素がベクターとかいう 複雑なもの☆（＾～＾）

use std;
use std::collections::HashMap;

pub struct RenElementMap {
    // 連ID に紐づく、石の番地のリスト。 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    pub value: HashMap<i16,Vec<i16>>
}
impl RenElementMap {
    pub fn new()-> RenElementMap {
        RenElementMap {
            value: HashMap::new(),
        }
    }

    /// 既存かチェックしないので、どんどん追加するぜ☆（＾～＾）
    pub fn add(&mut self, ren_id:i16, addr:i16) {
        match self.value.get_mut(&ren_id) {
            Some(vec) => {
                vec.push(addr);
            },
            None => {panic!("Ren id: {}, Addr: {}.", ren_id, addr)}
        };
    }

    pub fn get(&self, ren_id:i16) -> Option<&Vec<i16>> {
        self.value.get(&ren_id)
    }

    pub fn get_mut(&mut self, ren_id:i16) -> Option<&mut Vec<i16>> {
        self.value.get_mut(&ren_id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<i16, Vec<i16>> {
        self.value.iter()
    }

    pub fn contains_key(&self, ren_id:i16) -> bool {
        self.value.contains_key(&ren_id)
    }

    /// 新規ベクターを追加。
    pub fn insert(&mut self, ren_id:i16, vec:Vec<i16>) {
        self.value.insert(ren_id, vec);
    }

    /// 既存ベクターに追加。
    pub fn extend(&mut self, ren_id:i16, vec:Vec<i16>) {
        match self.value.get_mut(&ren_id) {
            Some(s) => {s.extend(vec.iter().cloned());},
            None => {panic!("Extend: ren_id: {}.", ren_id)},
        };
    }

    pub fn remove(&mut self, ren_id:i16) -> Option<Vec<i16>> {
        self.value.remove(&ren_id)
    }

    /// キーを変更。
    pub fn change_key(&mut self, ren_id_before:i16, ren_id_after:i16){
        match self.remove(ren_id_before) {
            Some(vec) => {
                if self.contains_key(ren_id_after) {
                    // 既存なら、既存ベクターに追加。
                    self.extend(ren_id_after, vec);
                } else {
                    // 無ければ、ベクターを丸ごと移動。
                    self.insert(ren_id_after, vec);
                }
            },
            None => {panic!("ren_id_before: {}, ren_id_after: {}.", ren_id_before, ren_id_after);}
        };
    }
}