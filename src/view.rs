use board::Board;
use position::Position;
use record::*;
use ren_db::ren_database::*;

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
pub fn show_piece_distribution(pos:&Position) {
    println!("Ren ID board: ");
    for (i, piece_id) in pos.get_piece_database().get_stone_piece_distribution().iter().enumerate() {
        if i == (pos.get_board().get_size()+2) * (pos.get_board().get_size()+2) {
            break;
        }
        print!("{:4}, ", piece_id);
        if i % (pos.get_board().get_size() + 2) == (pos.get_board().get_size() + 1) {
            println!();
        }
    }

    println!("Empty ren ID board: ");
    for (i, piece_id) in pos.get_piece_database().get_empty_piece_distribution().iter().enumerate() {
        if i == (pos.get_board().get_size()+2) * (pos.get_board().get_size()+2) {
            break;
        }
        print!("{:4}, ", piece_id);
        if i % (pos.get_board().get_size() + 2) == (pos.get_board().get_size() + 1) {
            println!();
        }
    }
}

/// 呼吸点の数を表示☆（＾～＾）
pub fn show_libarty_count(ren_map:&PieceGraph) {
    println!("Liberty count: ");
    for (piece_id, ren_obj) in ren_map.iter() {
        if (*ren_obj).get_liberty_count() != 0 {
            println!("[{:3}] {:3}", piece_id, (*ren_obj).get_liberty_count());
        }
    }
}

/// 空連の占有者を表示☆（＾～＾）
pub fn show_territory(ren_map:&PieceGraph) {
    println!("Territory: ");
    for (piece_id, ren_obj) in ren_map.iter() {
        let territory = ren_obj.get_territory();
        if territory != 0 && territory != 3 {
            println!("[{:3}] {:3}", piece_id, territory);
        }
    }
}

/// すべての連の、すべての番地を表示☆（＾～＾）
pub fn show_ren_address_map(ren_map:&PieceGraph) {
    println!("Ren element: ");
    for (piece_id, ren_obj) in ren_map.iter() {
        print!("[{:3}] ", piece_id);
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
pub fn show_ren_addr(ren_obj:&PieceObject) {
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
