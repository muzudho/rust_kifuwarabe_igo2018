/// 連ID に紐づく、石の番地のリスト。
/// ハッシュマップの要素がベクターとかいう 複雑なもの☆（＾～＾）

use std;
use std::collections::HashMap;

pub struct RenElementMap {
    // 連ID に紐づく、石の番地のリスト。 19路盤は 361交点あるので、i8 の -128～127 では足りない☆（＾～＾）
    pub value: HashMap<i8,Vec<i16>>
}
impl RenElementMap {
    pub fn new()-> RenElementMap {
        RenElementMap {
            value: HashMap::new(),
        }
    }

    pub fn get(&self, ren_id:i8) -> Option<&Vec<i16>> {
        self.value.get(&ren_id)
    }

    pub fn get_mut(&mut self, ren_id:i8) -> Option<&mut Vec<i16>> {
        self.value.get_mut(&ren_id)
    }

    pub fn contains_key(&self, ren_id:i8) -> bool {
        self.value.contains_key(&ren_id)
    }

    // 新規ベクターを追加。
    pub fn insert(&mut self, ren_id:i8, vec:Vec<i16>) {
        self.value.insert(ren_id, vec);
    }

    // 既存ベクターに追加。
    pub fn extend(&mut self, ren_id:i8, vec:Vec<i16>) {
        match self.value.get_mut(&(ren_id as i8)) {
            Some(s) => {s.extend(vec.iter().cloned());},
            None => {panic!("Extend: ren_id: {}.", ren_id)},
        };
    }

    pub fn remove(&mut self, ren_id:i8) -> Option<Vec<i16>> {
        self.value.remove(&(ren_id as i8))
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<i8, Vec<i16>> {
        self.value.iter()
    }
}