use chess::{Board, BoardStatus, Color, MoveGen, Piece};

const PIECE_VALUES: [(Piece, i32); 6] = [
    (Piece::Pawn, 1),
    (Piece::Knight, 3),
    (Piece::Bishop, 3),
    (Piece::Rook, 5),
    (Piece::Queen, 9),
    (Piece::King, 1000),
];

fn evaluation(board: &Board) -> i32 {
    let mut score = 0;

    for (piece, value) in PIECE_VALUES.iter() {
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
    
    let mut eval = if maximize { i32::MIN } else { i32::MAX };

    if maximize {
        for m in MoveGen::new_legal(&board) {
            let neighbour = board.make_move_new(m);
            eval = eval.max(minmax(&neighbour, depth - 1, false));
        }

    } else {
        for m in MoveGen::new_legal(&board) {
            let neighbour = board.make_move_new(m);
            eval = eval.min(minmax(&neighbour, depth - 1, true));
        }
    }

    eval
}

fn main() {
    let board = Board::default();
    let movegen = MoveGen::new_legal(&board);
    assert_eq!(movegen.len(), 20);
    println!("{}", minmax(&board, 4, true))
}
