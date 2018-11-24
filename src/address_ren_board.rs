// 連のIDが入った盤☆（＾～＾）

use std;
use ren_database::*;

pub struct AddressRenBoard {
    // 番地と連IDの紐づけ。
    pub value: [i16; 21 * 21],
}
impl AddressRenBoard {
    pub fn new() -> AddressRenBoard {
        AddressRenBoard {
            value: [0; 21 * 21]
        }
    }

    pub fn get(&self, addr:usize) -> i16 {
        self.value[addr]
    }

    pub fn set(&mut self, addr:usize, ren_id:i16) {
        self.value[addr] = ren_id;
    }

    pub fn iter(&self) -> std::slice::Iter<i16> {
        self.value.iter()
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは ren_id を 0 にする。
    pub fn fill_by_vec(&mut self, addr_vec:&Vec<i16>, ren_id:i16) {
        for addr in addr_vec {
            self.value[*addr as usize] = ren_id;
        }
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは ren_id を 0 にする。
    pub fn fill_by_ren(&mut self, ren_obj:&RenObject, new_ren_id:i16) {
        for addr in ren_obj.iter_addr() {
            self.value[*addr as usize] = new_ren_id;
        }
    }
}