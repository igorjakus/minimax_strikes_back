use std::time::Instant;
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


fn minmax(board: &Board, depth: i32, alpha: i32, beta: i32, maximize: bool) -> i32 {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return evaluation(board)
    }
    
    let mut alpha = alpha;
    let mut beta = beta;

    let mut eval = if maximize { i32::MIN } else { i32::MAX };

    if maximize {
        for m in MoveGen::new_legal(&board) {
            let neighbour = board.make_move_new(m);
            eval = eval.max(minmax(&neighbour, depth - 1, alpha, beta, false));
            alpha = alpha.max(eval);

            if beta <= alpha {
                break;
            }
        }

    } else {
        for m in MoveGen::new_legal(&board) {
            let neighbour = board.make_move_new(m);
            eval = eval.min(minmax(&neighbour, depth - 1, alpha, beta, true));
            beta = beta.min(eval);
            
            if beta <= alpha {
                break;
            }
        }
    }

    eval
}

fn main() {
    let board = Board::default();

    let start = Instant::now();
    
    let eval = minmax(&board, 8, i32::MIN, i32::MAX, true);
    println!("Minmax result: {}", eval);
    
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
