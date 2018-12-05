// 棋譜☆（＾～＾）

use ren_db::ren_database::*;

#[derive(Default)]
pub struct RecordItem {
    // 指し手の番地を入れてくだけ☆（＾～＾）
    pub move_addr: i16,

    // 打ち上げた石の番地を覚えるのに使う。
    pub agehama_addrs: Vec<i16>,

    // 盤面ハッシュ。スーパーコウ判定に使う☆（＾～＾）
    pub board_hash: u64,
}
impl RecordItem {
    pub fn new() -> RecordItem {
        RecordItem {
            move_addr: 0,
            agehama_addrs: Vec::new(),
            board_hash: 0,
        }
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは ren_id を 0 にする。
    pub fn add_agehama_by_vec(&mut self, agehama:&Vec<i16>) {
        for addr in agehama {
            self.agehama_addrs.push(*addr);
        }
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは ren_id を 0 にする。
    pub fn add_agehama_by_ren(&mut self, agehama_ren_obj:&PieceObject) {
        for addr in agehama_ren_obj.iter_addr() {
            self.agehama_addrs.push(*addr);
        }
    }
}

#[derive(Default)]
pub struct Record {
    // 1手打つごとに増えていく☆（＾～＾）
    pub items: Vec<RecordItem>,
}
impl Record {
    pub fn new() -> Record {
        Record {
            items: Vec::new(),
        }
    }

    pub fn count_up(&mut self) {
        self.items.push(RecordItem::new());
    }

    pub fn count_down(&mut self) -> Option<RecordItem> {
        self.items.pop()
    }

    pub fn get_current(&self) -> &RecordItem {
        let index = self.items.len()-1;
        &self.items[index]
    }

    pub fn set_current(&mut self, target:i16, board_hash:u64) {
        let index = self.items.len()-1;
        self.items[index].move_addr = target;
        self.items[index].board_hash = board_hash;
    }

    pub fn add_current_agehama_by_vec(&mut self, agehama:&Vec<i16>){
        let index = self.items.len()-1;
        self.items[index].add_agehama_by_vec(agehama);
    }

    pub fn add_current_agehama_by_ren(&mut self, agehama_ren_obj:&PieceObject){
        let index = self.items.len()-1;
        self.items[index].add_agehama_by_ren(agehama_ren_obj);
    }

    /*
    pub fn get_mut_current(&mut self) -> &mut RecordItem {
        let index = self.items.len()-1;
        &mut self.items[index]
    }
     */

    pub fn is_empty(&self) -> bool {
        0 == self.items.len()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// スーパー コウのとき、真☆（＾～＾）
    pub fn is_super_ko(&self) -> bool {
        let last = self.get_current();

        for i in 0..self.len()-1 {
            if last.board_hash == self.items[i].board_hash {
                return true;
            }
        }
        false
    }
}