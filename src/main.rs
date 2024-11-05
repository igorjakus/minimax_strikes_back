use chess::{Board, BoardStatus, Color, MoveGen, Piece};

fn evaluation(board: &Board) -> i32 {
    let mut score = 0;

    let piece_values = [
        (Piece::Pawn, 1),
        (Piece::Knight, 3),
        (Piece::Bishop, 3),
        (Piece::Rook, 5),
        (Piece::Queen, 9),
        (Piece::King, 1000),
    ];

    for (piece, value) in piece_values.iter() {
        let white_pieces = board.pieces(*piece) & board.color_combined(Color::White);
        score += white_pieces.popcnt() as i32 * value;
        
        let black_pieces = board.pieces(*piece) & board.color_combined(Color::Black);
        score -= black_pieces.popcnt() as i32 * value;
    }

    score
}


fn minmax(board: &Board, depth: i32, maximize: bool) -> i32 {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return evaluation(board)
    }
    
    if maximize {
        let mut eval = i32::MIN;
        eval = eval.max(minmax(board, depth - 1, false));
        return eval
    } else {
        let mut eval = i32::MAX;
        eval = eval.min(minmax(board, depth - 1, false));
        return eval
    }
}

fn main() {
    let board = Board::default();
    let movegen = MoveGen::new_legal(&board);
    assert_eq!(movegen.len(), 20);
    println!("{}", minmax(&board, 0, true))
}
