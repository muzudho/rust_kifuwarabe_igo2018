/// 連をデータベース化したい☆（＾～＾）

use std;
use std::collections::HashMap;

pub struct RenDatabase {
    // 連ID に紐づくもの。 連IDは 番地から作られる。 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    stone_ren: HashMap<i16,StoneRenRecord>,
    // 空連ID に紐づくもの。
    empty_ren: HashMap<i16,EmptyRenRecord>,
}
impl RenDatabase {
    pub fn new() -> RenDatabase {
        RenDatabase {
            stone_ren: HashMap::new(),
            empty_ren: HashMap::new(),
        }
    }

    pub fn get_stone_ren(&self) -> &StoneRenRecord {
        &self.stone_ren
    }

    pub fn get_mut_stone_ren(&mut self) -> &mut StoneRenRecord {
        &mut self.stone_ren
    }

    pub fn get_empty_ren(&self) -> &EmptyRenRecord {
        &self.empty_ren
    }

    pub fn get_mut_empty_ren(&mut self) -> &mut EmptyRenRecord {
        &mut self.empty_ren
    }

}



pub struct StoneRenRecord {

}
impl StoneRenRecord {
    pub fn new() -> StoneRenRecord {
        StoneRenRecord {

        }
    }
}

pub struct EmptyRenRecord {

}
impl EmptyRenRecord {
    pub fn new() -> EmptyRenRecord {
        EmptyRenRecord {

        }
    }
}