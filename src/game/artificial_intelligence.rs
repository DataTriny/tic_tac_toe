use super::{
    board::{Board, GameResult, PlayingPosition},
    players::Role
};

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
pub fn minimax(board: &mut Board, player: Role) -> Move {
    let available_spots = board.get_available_spots();
    if let GameResult::Winner(Role::O, _) = board.compute_result(Role::O) {
        return Move::with_score(-10);
    }
    if let GameResult::Winner(Role::X, _) = board.compute_result(Role::X) {
        return Move::with_score(10);
    }
    if available_spots.len() == 0 {
        return Move::with_score(0);
    }
    let mut moves = Vec::new();
    for spot in available_spots.iter() {
        board.set(spot.0, spot.1, player.clone());
        let m = Move::new(
            *spot,
            if player == Role::X {
                minimax(board, Role::O)
            } else {
                minimax(board, Role::X)
            }
            .score,
        );
        board.reset(spot.0, spot.1);
        moves.push(m);
    }
    let mut best_move = 0;
    if player == Role::X {
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

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::{
        minimax,
        super::{
            board::{Board, GameResult},
            players::Role
        }
    };

    #[test]
    fn minimax_role_x_pos_after_set_0_0_role_o() {
        let mut board = Board::new();
        let game_result = board.set(0, 0, Role::O);
        assert_eq!(GameResult::NotFinished, game_result);
        assert_eq!((1, 1), minimax(&mut board, Role::X).pos);
    }
}
