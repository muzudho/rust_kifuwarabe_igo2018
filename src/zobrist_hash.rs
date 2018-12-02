// TODO スーパーコウ ルールのためのゾブリスト ハッシュだぜ☆（＾～＾）

// ランダムムーブ
extern crate rand;
use rand::Rng;

// でかい囲碁盤で、まともに ゾブリスト ハッシュ を作ると 時間がかかってしまう。
// スーパーコウ に使うので、盤面の情報だけでいいのか☆（＾～＾）？
pub struct ZobristHash {
    // 枠の内側の、黒石の、盤面の361マス のランダム値。
    pub black: [u64; 21*21],

    // 枠の内側の、白石の、盤面の361マス のランダム値。
    pub white: [u64; 21*21],

    // // コウの番地 361 マスのランダム値。

    // // 手番2つのランダム値。
}
impl Default for ZobristHash {
    fn default() -> Self {
        Self::new()
    }
}
impl ZobristHash {
    pub fn new() -> ZobristHash {
        let mut zob = ZobristHash {
            black: [0; 21*21],
            white: [0; 21*21],
        };

        // めんどくさいんで、枠にもランダムな数を入れておく。それなら 何路盤でも対応できる☆（＾～＾）
        for i in 0..441 {
            zob.black[i] = rand::thread_rng().next_u64();
            zob.white[i] = rand::thread_rng().next_u64();
        }

        zob
    }

    /// 石が現れた／消えた。
    pub fn switch(&self, hash:u64, color:i8, target:i16) -> u64 {
        match color {
            1 => {hash ^ self.black[target as usize]},
            2 => {hash ^ self.white[target as usize]},
            _ => {hash},
        }        
    }
}