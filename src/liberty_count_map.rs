/// 連IDに呼吸点の数を紐づけるぜ☆（＾～＾）

use std;

pub struct LibertyCountMap {
    /// 呼吸点の数は、盤の交点の数より必ず少ない。が、 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    pub value: [i16; 21*21],
}
impl LibertyCountMap {
    pub fn new() -> LibertyCountMap {
        LibertyCountMap {
            value: [0; 21*21],
        }
    }

    pub fn add(&mut self, index:usize, liberty_count:i16) {
        self.value[index] += liberty_count;
    }

    pub fn get(&self, index:usize) -> i16 {
        self.value[index]
    }

    pub fn set(&mut self, index:usize, liberty_count:i16) {
        self.value[index] = liberty_count;
    }

    pub fn iter(&self) -> std::slice::Iter<i16> {
        self.value.iter()
    }

}