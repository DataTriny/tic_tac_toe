use super::board::{Board, GameResult, PlayingPosition, Tile};

/// Represents a move that can be performed by the computer.
#[derive(Clone, Copy)]
pub struct Move {
    /// The coordinates of this move.
    pub pos: PlayingPosition,
    /// The score associated with this move.
    pub score: i32,
}

impl Move {
    fn new(pos: PlayingPosition, score: i32) -> Self {
        Move { pos, score }
    }

    fn with_score(score: i32) -> Self {
        Move::new((0, 0), score)
    }
}

/// A minimax algorithm that performs on a tic-tac-toe board. Returns the best move found.
pub fn minimax(board: &mut Board, player: Tile) -> Move {
    let available_spots = board.get_available_spots();
    if available_spots.len() == 0 {
        return Move::with_score(match board.compute_result() {
            GameResult::Winner(Tile::O, _) => -10,
            GameResult::Winner(Tile::X, _) => 10,
            _ => 0,
        });
    }
    let mut moves = Vec::new();
    for spot in available_spots.iter() {
        board.set(spot.0, spot.1, player.clone());
        let m = Move::new(
            *spot,
            if player == Tile::X {
                minimax(board, Tile::O)
            } else {
                minimax(board, Tile::X)
            }
            .score,
        );
        board.set(spot.0, spot.1, Tile::Empty);
        moves.push(m);
    }
    let mut best_move = 0;
    if player == Tile::X {
        let mut best_score = -10000;
        for (i, m) in moves.iter().enumerate() {
            if m.score > best_score {
                best_score = m.score;
                best_move = i;
            }
        }
    } else {
        let mut best_score = 10000;
        for (i, m) in moves.iter().enumerate() {
            if m.score < best_score {
                best_score = m.score;
                best_move = i;
            }
        }
    }
    moves[best_move]
}
