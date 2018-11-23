// 棋譜☆（＾～＾）

pub struct RecordItem {
    // 指し手の番地を入れてくだけ☆（＾～＾）
    pub move_addr: i16,

    // 打ち上げた石の番地を覚えるのに使う。
    pub agehama_addrs: Vec<i16>,
}
impl RecordItem {
    pub fn new() -> RecordItem {
        RecordItem {
            move_addr: 0,
            agehama_addrs: Vec::new(),
        }
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは ren_id を 0 にする。
    pub fn add_agehama(&mut self, agehama:&Vec<i16>) {
        for addr in agehama {
            self.agehama_addrs.push(*addr);
        }
    }
}

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

    pub fn countUp(&mut self) {
        self.items.push(RecordItem::new());
    }

    pub fn get_mut_current(&mut self) -> &mut RecordItem {
        let index = self.items.len()-1;
        &mut self.items[index]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}