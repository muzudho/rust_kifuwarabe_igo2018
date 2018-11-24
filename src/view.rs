use address_ren_board::*;
use board::Board;
use empty_owner_map::EmptyOwnerMap;
use liberty_count_map::LibertyCountMap;
use position::Position;
use record::*;
use ren_database::*;

/// 盤の表示☆（＾～＾）
pub fn show_board(board:&Board){
    println!("Board: ");
    for (i, stone) in board.iter().enumerate() {
        if i == (board.get_size()+2) * (board.get_size()+2) {
            break;
        }

        print!("{}", match stone {
            0 => ' ',
            1 => 'x',
            2 => 'o',
            _ => '+',
        });

        if i % (board.get_size() + 2) == (board.get_size() + 1) {
            println!();
        }
    }
    println!("Hash: '{}'.", board.get_hash());
}

/// スーパーコウかどうか表示☆（＾～＾）
pub fn show_super_ko(record:&Record) {
    if record.is_super_ko() {
        println!("Super ko.");
    }
}

/// セル番地を表示☆（＾～＾）
pub fn show_board_address(board_size:usize) {
    println!("Cell address: ");
    let end = (board_size+2) * (board_size+2) + 1;

    for i in 0..end { // 検索を開始するセルの番号。連のIDを決めるのにも使う。
        if i == (board_size+2) * (board_size+2) {
            break;
        }
        print!("{:3}, ", i);
        if i % (board_size + 2) == (board_size + 1) {
            println!();
        }
    }
}

/// 盤の表示☆（＾～＾）
pub fn show_board_by_number(board:&Board) {
    println!("Board: ");
    for (i, stone) in board.iter().enumerate() {
        if i == (board.get_size()+2) * (board.get_size()+2) {
            break;
        }
        print!("{}, ", stone);
        if i % (board.get_size() + 2) == (board.get_size() + 1) {
            println!();
        }
    }
}

/// 盤に振られた 連ID を表示だぜ☆（＾～＾）
pub fn show_address_ren_board(pos:&Position) {
    println!("Ren ID board: ");
    for (i, ren_id) in pos.get_ren_database().get_address_stone_ren_board().iter().enumerate() {
        if i == (pos.board.get_size()+2) * (pos.board.get_size()+2) {
            break;
        }
        print!("{:4}, ", ren_id);
        if i % (pos.board.get_size() + 2) == (pos.board.get_size() + 1) {
            println!();
        }
    }

    println!("Empty ren ID board: ");
    for (i, ren_id) in pos.empty_owner_map.address_empty_ren_board.iter().enumerate() {
        if i == (pos.board.get_size()+2) * (pos.board.get_size()+2) {
            break;
        }
        print!("{:4}, ", ren_id);
        if i % (pos.board.get_size() + 2) == (pos.board.get_size() + 1) {
            println!();
        }
    }
}

/// 呼吸点の数を表示☆（＾～＾）
pub fn show_libarty_count(liberty_count_map:&LibertyCountMap) {
    println!("Liberty count: ");
    for (ren_id, lib_cnt) in liberty_count_map.iter().enumerate() {
        if *lib_cnt != 0 {
            println!("[{:3}] {:3}", ren_id, lib_cnt);
        }
    }
}

/// 空連の占有者を表示☆（＾～＾）
pub fn show_empty_owner(empty_owner_map:&EmptyOwnerMap) {
    println!("Empty owner: ");
    for (ren_id, owner) in empty_owner_map.get_territory().iter_owner().enumerate() {
        if *owner != 0 && *owner != 3 {
            println!("[{:3}] {:3}", ren_id, owner);
        }
    }
}

/// すべての連の、すべての番地を表示☆（＾～＾）
pub fn show_ren_address_map(ren_map:&RenMap) {
    println!("Ren element: ");
    for (ren_id, ren_obj) in ren_map.iter() {
        print!("[{:3}] ", ren_id);
        for addr in ren_obj.iter_addr() {
            print!("{:3} ", addr);
        }
        println!(".");
    }
}

/// 合法手の表示☆（＾～＾）
pub fn show_legal_moves(legal_moves:&[usize]) {
    print!("Legal moves: ");
    for legal_move in legal_moves {
        print!("{}, ", legal_move);
    }
    println!(".");
}

/// 連が占有する番地を表示☆（＾～＾）
pub fn show_ren_addr(ren_obj:&RenObject) {
    for addr in ren_obj.iter_addr() {
        print!("{:3}, ", addr);
    }
    println!(".");
}

/// ベクターを表示☆（＾～＾）
pub fn show_vector_i16(vec:&Vec<i16>) {
    for value in vec.iter() {
        print!("{:3}, ", value);
    }
    println!(".");
}
