/// 連IDに呼吸点の数を紐づけるぜ☆（＾～＾）

use std;

// 連の呼吸点の数。
pub struct LibertyCountMap {
    /// 呼吸点の数は、盤の交点の数より必ず少ない。が、 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    pub liberty_count: [i16; 21*21],
}
impl Default for LibertyCountMap {
    fn default() -> Self {
        Self::new()
    }
}
impl LibertyCountMap {
    pub fn new() -> LibertyCountMap {
        LibertyCountMap {
            liberty_count: [0; 21*21],
        }
    }

    pub fn add_liberty_count(&mut self, index:usize, liberty_count:i16) {
        self.liberty_count[index] += liberty_count;
    }

    pub fn get_liberty_count(&self, index:usize) -> i16 {
        self.liberty_count[index]
    }

    pub fn set_liberty_count(&mut self, index:usize, liberty_count:i16) {
        self.liberty_count[index] = liberty_count;
    }

    pub fn iter_liberty_count(&self) -> std::slice::Iter<i16> {
        self.liberty_count.iter()
    }

    /// キーを変更。
    pub fn change_key_liberty_count(&mut self, ren_id_before:i16, ren_id_after:i16){
        self.liberty_count[ren_id_after as usize] = self.liberty_count[ren_id_before as usize];
        self.liberty_count[ren_id_before as usize] = 0;
    }

}