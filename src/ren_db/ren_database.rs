/// 連をデータベース化したい☆（＾～＾）

use std;
use std::collections::HashMap;
// use piece_distribution_searcher::*;

/// 石連と、空連に大きく分かれる☆（＾～＾）
#[derive(Default)]
pub struct RenDatabase {
    // 連ID に紐づくプロパティ。 連IDは 番地から作られる。 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    ren_mappings: PieceGraph,

    /// 算法の都合で２つに分けている。盤上に紐づく「石」の連ID。
    stone_piece_distribution: PieceDistribution,

    /// 算法の都合で２つに分けている。盤上に紐づく「空点」の連ID。
    empty_piece_distribution: PieceDistribution,
}
impl RenDatabase {
    pub fn new() -> RenDatabase {
        RenDatabase {
            ren_mappings: PieceGraph::new(),
            stone_piece_distribution: PieceDistribution::new(),
            empty_piece_distribution: PieceDistribution::new(),
        }
    }

    pub fn get_piece_mappings(&self) -> &PieceGraph {
        &self.ren_mappings
    }

    pub fn get_mut_ren_mappings(&mut self) -> &mut PieceGraph {
        &mut self.ren_mappings
    }

    pub fn get_stone_piece_distribution(&self) -> &PieceDistribution {
        &self.stone_piece_distribution
    }

    pub fn get_mut_stone_piece_distribution(&mut self) -> &mut PieceDistribution {
        &mut self.stone_piece_distribution
    }

    pub fn get_empty_piece_distribution(&self) -> &PieceDistribution {
        &self.empty_piece_distribution
    }

    pub fn get_mut_empty_piece_distribution(&mut self) -> &mut PieceDistribution {
        &mut self.empty_piece_distribution
    }
}


/// 連ID と、 連オブジェクト が紐づく。
#[derive(Default)]
pub struct PieceGraph {
    map: HashMap<i16,PieceObject>,
}
impl PieceGraph {
    pub fn new() -> PieceGraph {
        PieceGraph {
            map: HashMap::new(),
        }
    }

    /// TODO 接続。
    pub fn connect(&mut self, target:i16, stone:i8) {
        // if let piece = get_piece();
    }

    /// TODO 切断。
    pub fn cut(&mut self, target:i16, stone:i8) {

    }

    /// 連に番地を追加するぜ☆（＾～＾）
    pub fn add_addr(&mut self, piece_id:i16, addr:i16) {
        if let Some(ren_obj) = self.map.get_mut(&piece_id) {
            // 番地追加。
            ren_obj.add_addr(addr);
            return;
        };

        // 無い連なら、新規作成して追加。
        let ren_obj = PieceObject::default(piece_id, vec![addr], 0);
        self.map.insert(piece_id, ren_obj);
    }


    pub fn get_piece(&self, piece_id:i16) -> Option<&PieceObject> {
        self.map.get(&piece_id)
    }

    pub fn get_mut_ren(&mut self, piece_id:i16) -> Option<&mut PieceObject> {
        self.map.get_mut(&piece_id)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<i16, PieceObject> {
        self.map.iter()
    }
    /*
    pub fn iter_liberty_count(&self) -> std::slice::Iter<i16> {
        self.liberty_count.iter()
    }

    /// キーを変更。
    pub fn change_key_liberty_count(&mut self, piece_id_before:i16, piece_id_after:i16){
        self.liberty_count[piece_id_after as usize] = self.liberty_count[piece_id_before as usize];
        self.liberty_count[piece_id_before as usize] = 0;
    }
    */

    pub fn contains_key(&self, piece_id:i16) -> bool {
        self.map.contains_key(&piece_id)
    }

    /// 外部から連を追加。
    pub fn insert_ren(&mut self, piece_id:i16, ren_obj:PieceObject) {
        self.map.insert(piece_id, ren_obj);
    }

    /// 既存の連に、外部から連を結合。
    pub fn extend_ren(&mut self, piece_id:i16, other_ren_obj:&PieceObject) {
        match self.map.get_mut(&piece_id) {
            Some(ren_obj) => {ren_obj.extend(&other_ren_obj);},
            None => {panic!("Extend: piece_id: {}.", piece_id)},
        };
    }

    // 連を除外。
    pub fn remove_ren(&mut self, piece_id:i16) -> Option<PieceObject> {
        self.map.remove(&piece_id)
    }

    /// 指定した連から、指定した番地を除外する。
    pub fn remove_addr(&mut self, piece_id:i16, removing_addr:i16) {
        // println!("連{} の {}番地を除外。", piece_id, removing_addr);
        match self.get_mut_ren(piece_id) {
            Some(ren_obj) => {
                // println!("連{} はあった。 {}番地を除外。", piece_id, removing_addr);
                ren_obj.remove_addr(removing_addr);
            },
            None => {panic!("削除したかった番地がなかった。")},
        };
    }

    /// 連のIDを変更。
    pub fn change_key(&mut self, piece_id_before:i16, piece_id_after:i16){
        match self.map.remove(&piece_id_before) {
            Some(ren_obj) => {
                if self.contains_key(piece_id_after) {
                    // 既存なら、既存ベクターに追加。
                    self.extend_ren(piece_id_after, &ren_obj);
                } else {
                    // 無ければ、ベクターを丸ごと移動。
                    self.insert_ren(piece_id_after, ren_obj);
                }
            },
            None => {panic!("piece_id_before: {}, piece_id_after: {}.", piece_id_before, piece_id_after);}
        };
    }
}




/// 連（のプロパティ）。
pub struct PieceObject {
    /// 連ID。
    id: i16,

    /// 含む番地。
    addresses: Vec<i16>,

    /// 空連IDに紐づく占有者。番地でアクセスするので、ボード形式で持つ☆（＾～＾）
    /// 空連の占有者は、以下のいずれか☆（＾～＾）
    /// 0. 未調査、または 隣接する石がない。
    /// 1. 黒石か枠のいずれかだけに隣接する。
    /// 2. 白石か枠のいずれかだけに隣接する。
    /// 3. 黒石と白石の両方に隣接する。
    territory: i8,

    // 計算用。連に紐づく呼吸点の数。呼吸点の数は、盤の交点の数より必ず少ない。が、 19路盤は 361交点あるので、i16 にする。 i8 の -128～127 では足りない☆（＾～＾）
    liberty_count: i16,
}
impl PieceObject {
    /*
    pub fn new() -> PieceObject {
        PieceObject {
            id: 0,
            addresses: Vec::new(),
            territory: 0,
        }
    }
     */

    pub fn default(piece_id:i16, member_addresses:Vec<i16>, empty_territory:i8) -> PieceObject {
        PieceObject {
            id: piece_id,
            addresses: member_addresses,
            territory: empty_territory,
            liberty_count: 0,
        }
    }

    /// 番地を追加する。
    pub fn add_addr(&mut self, addr:i16) {
        self.addresses.push(addr);
    }

    /// 別の連の番地を追加する。
    pub fn extend(&mut self, other_ren_obj:&PieceObject) {
        self.addresses.extend(other_ren_obj.iter_addr().cloned());
    }

    pub fn get_id(&self) -> i16 {
        self.id
    }

    pub fn iter_addr(&self) -> std::slice::Iter<i16> {
        self.addresses.iter()
    }

    pub fn len_addr(&self) -> usize {
        self.addresses.len()
    }

    /// 指定の番地を除外する。
    pub fn remove_addr(&mut self, addr:i16) {
        let index = self.addresses.iter().position(|&r| r == addr).unwrap();
        self.addresses.remove(index);
    }

    pub fn to_addr_vec(&self) -> Vec<i16> {
        self.addresses.to_vec()
    }



    pub fn get_territory(&self) -> i8 {
        self.territory
    }

    pub fn set_territory(&mut self, territory:i8) {
        self.territory = territory;
    }

    /// 自分の目つぶしなら真。
    pub fn is_eye_filling(&self, color:i8) -> bool {
        if self.territory != color {
            return false;
        }

        1 == self.len_addr()
    }


    pub fn add_liberty_count(&mut self, liberty_count:i16) {
        self.liberty_count += liberty_count;
    }

    pub fn get_liberty_count(&self) -> i16 {
        self.liberty_count
    }

    pub fn set_liberty_count(&mut self, liberty_count:i16) {
        self.liberty_count = liberty_count;
    }
}




/// 連のIDが入った盤☆（＾～＾）
pub struct PieceDistribution {
    // 番地と連IDの紐づけ。
    pub value: [i16; 21 * 21],
}
impl Default for PieceDistribution {
    fn default() -> Self {
        Self::new()
    }
}
impl PieceDistribution {
    pub fn new() -> PieceDistribution {
        PieceDistribution {
            value: [0; 21 * 21]
        }
    }

    pub fn get(&self, addr:usize) -> i16 {
        self.value[addr]
    }

    pub fn set(&mut self, addr:usize, piece_id:i16) {
        self.value[addr] = piece_id;
    }

    pub fn iter(&self) -> std::slice::Iter<i16> {
        self.value.iter()
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは piece_id を 0 にする。
    pub fn fill_by_vec(&mut self, addr_vec:&Vec<i16>, piece_id:i16) {
        for addr in addr_vec {
            self.value[*addr as usize] = piece_id;
        }
    }

    /// 複数の指定アドレスを 連ID で埋める。石を除去したいときは piece_id を 0 にする。
    pub fn fill_by_ren(&mut self, ren_obj:&PieceObject, new_piece_id:i16) {
        for addr in ren_obj.iter_addr() {
            self.value[*addr as usize] = new_piece_id;
        }
    }
}
