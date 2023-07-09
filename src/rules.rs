use crate::PieceType;
use crate::Color;
use crate::Piece;
use crate::Board;

// Check if the move is valid for the piece
pub fn is_valid_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    match piece.piece_type {
        PieceType::Pawn => is_valid_pawn_move(board, piece, start, end),
        PieceType::Rook => is_valid_rook_move(board, piece, start, end),
        PieceType::Knight => is_valid_knight_move(board, piece, start, end),
        PieceType::Bishop => is_valid_bishop_move(board, piece, start, end),
        PieceType::Queen => is_valid_queen_move(board, piece, start, end),
        PieceType::King => is_valid_king_move(board, piece, start, end),
    }
}

// Check if the move is valid for a pawn
pub fn is_valid_pawn_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    match piece.color {
        Color::White => {
            if start_x == 6 && end_x == 4 && start_y == end_y && board[end_x][end_y].is_none() {
                // Initial double step move
                true
            } else if end_x == start_x - 1 && start_y == end_y && board[end_x][end_y].is_none() {
                // Regular move
                true
            } else if end_x == start_x - 1 && (end_y == start_y - 1 || end_y == start_y + 1) {
                // Capture
                if let Some(end_piece) = &board[end_x][end_y] {
                    end_piece.color == Color::Black
                } else {
                    false
                }
            } else {
                false
            }
        },
        Color::Black => {
            if start_x == 1 && end_x == 3 && start_y == end_y && board[end_x][end_y].is_none() {
                // Initial double step move
                true
            } else if end_x == start_x + 1 && start_y == end_y && board[end_x][end_y].is_none() {
                // Regular move
                true
            } else if end_x == start_x + 1 && (end_y == start_y - 1 || end_y == start_y + 1) {
                // Capture
                if let Some(end_piece) = &board[end_x][end_y] {
                    end_piece.color == Color::White
                } else {
                    false
                }
            } else {
                false
            }
        },
    }
}

// Check if the move is valid for a rook
pub fn is_valid_rook_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // Rooks can move any number of squares along a rank or file
    // They cannot jump over other pieces
    if start_x == end_x {
        // Moving along a file
        let min_y = start_y.min(end_y);
        let max_y = start_y.max(end_y);
        for y in min_y + 1..max_y {
            if board[start_x][y].is_some() {
                return false;
            }
        }
    } else if start_y == end_y {
        // Moving along a rank
        let min_x = start_x.min(end_x);
        let max_x = start_x.max(end_x);
        for x in min_x + 1..max_x {
            if board[x][start_y].is_some() {
                return false;
            }
        }
    } else {
        // Not a valid rook move
        return false;
    }

    // Check if the destination square is occupied by a piece of the same color
    if let Some(end_piece) = &board[end_x][end_y] {
        if end_piece.color == piece.color {
            return false;
        }
    }

    true
}

// Check if the move is valid for a knight
pub fn is_valid_knight_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // Knights can move to any square not on the same rank, file, or diagonal
    // Their move is in the shape of an L - two squares in one direction and then one square perpendicular to that
    let dx = (start_x as i32 - end_x as i32).abs();
    let dy = (start_y as i32 - end_y as i32).abs();
    if (dx == 2 && dy == 1) || (dx == 1 && dy == 2) {
        // Check if the destination square is occupied by a piece of the same color
        if let Some(end_piece) = &board[end_x][end_y] {
            if end_piece.color == piece.color {
                return false;
            }
        }

        true
    } else {
        false
    }
}

// Check if the move is valid for a bishop
pub fn is_valid_bishop_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // Bishops can move any number of squares diagonally
    // They cannot jump over other pieces
    let dx = (start_x as i32 - end_x as i32).abs();
    let dy = (start_y as i32 - end_y as i32).abs();
    if dx == dy {
        // Moving diagonally
        let x_step = if start_x < end_x { 1 } else { -1 };
        let y_step = if start_y < end_y { 1 } else { -1 };
        let mut x = start_x as i32 + x_step;
        let mut y = start_y as i32 + y_step;
        while x != end_x as i32 && y != end_y as i32 {
            if board[x as usize][y as usize].is_some() {
                return false;
            }
            x += x_step;
            y += y_step;
        }
    } else {
        // Not a valid bishop move
        return false;
    }

    // Check if the destination square is occupied by a piece of the same color
    if let Some(end_piece) = &board[end_x][end_y] {
        if end_piece.color == piece.color {
            return false;
        }
    }

    true
}

// Check if the move is valid for a queen
pub fn is_valid_queen_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    // A queen can move any number of squares along a rank, file, or diagonal
    // It combines the power of the rook and bishop and can move like either
    is_valid_rook_move(board, piece, start, end) || is_valid_bishop_move(board, piece, start, end)
}

// Check if the move is valid for a king
pub fn is_valid_king_move(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // A king can move one square in any direction
    let dx = (start_x as i32 - end_x as i32).abs();
    let dy = (start_y as i32 - end_y as i32).abs();
    if dx <= 1 && dy <= 1 {
        // Check if the destination square is occupied by a piece of the same color
        if let Some(end_piece) = &board[end_x][end_y] {
            if end_piece.color == piece.color {
                return false;
            }
        }

        true
    } else {
        false
    }
}

// Check if the move is a valid en passant capture
pub fn is_valid_en_passant(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize), en_passant_square: Option<(usize, usize)>) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // En passant can only be performed by a pawn
    if piece.piece_type != PieceType::Pawn {
        return false;
    }

    // The pawn must be on its fifth rank
    if (piece.color == Color::White && start_x != 3) || (piece.color == Color::Black && start_x != 4) {
        return false;
    }

    // The destination square must be the en passant square
    if Some((end_x, end_y)) != en_passant_square {
        return false;
    }

    true
}

// Check if the move is a valid castling move
pub fn is_valid_castling(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize), can_castle: (bool, bool)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // Castling can only be performed by a king
    if piece.piece_type != PieceType::King {
        return false;
    }

    // The king must not have moved before
    if start_x != if piece.color == Color::White { 7 } else { 0 } || start_y != 4 {
        return false;
    }

    // The destination square must be on the king's initial rank
    if end_x != start_x {
        return false;
    }

    // The king must move two squares towards the rook
    if end_y != 2 && end_y != 6 {
        return false;
    }

    // The corresponding rook must not have moved before
    if end_y == 2 && !can_castle.0 || end_y == 6 && !can_castle.1 {
        return false;
    }

    // There must be no pieces between the king and the rook
    let y_range = if end_y == 2 { 1..4 } else { 5..7 };
    for y in y_range {
        if board[start_x][y].is_some() {
            return false;
        }
    }

    // The king must not be in check in the final position
    let mut new_board = board.clone();
    new_board[end_x][end_y] = Some(piece.clone());
    new_board[start_x][start_y] = None;
    if is_in_check(&new_board, piece.color) {
        return false;
    }

    true
}

// Check if the king is in check
pub fn is_in_check(board: &Board, color: Color) -> bool {
    // Find the king
    let (king_x, king_y) = board.iter().enumerate().find_map(|(x, row)| {
        row.iter().enumerate().find_map(|(y, square)| {
            if let Some(piece) = square {
                if piece.piece_type == PieceType::King && piece.color == color {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }).unwrap();

    // Check if any of the opponent's pieces can capture the king
    for (x, row) in board.iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color != color && is_valid_move(board, piece, (x, y), (king_x, king_y)) {
                    return true;
                }
            }
        }
    }

    false
}

// Check if the game is in checkmate
pub fn is_checkmate(board: &Board, color: Color) -> bool {
    // The player must be in check
    if !is_in_check(board, color) {
        return false;
    }

    // Check if the player has any legal moves
    for (x, row) in board.iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color == color {
                    // Try all possible moves for this piece
                    for dx in 0..8 {
                        for dy in 0..8 {
                            if is_valid_move(board, piece, (x, y), (dx, dy)) {
                                // Check if this move would get the player out of check
                                let mut new_board = board.clone();
                                new_board[dx][dy] = Some(piece.clone());
                                new_board[x][y] = None;
                                if !is_in_check(&new_board, color) {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    true
}

// Check if the game is in stalemate
pub fn is_stalemate(board: &Board, color: Color) -> bool {
    // The player must not be in check
    if is_in_check(board, color) {
        return false;
    }

    // Check if the player has any legal moves
    for (x, row) in board.iter().enumerate() {
        for (y, square) in row.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color == color {
                    // Try all possible moves for this piece
                    for dx in 0..8 {
                        for dy in 0..8 {
                            if is_valid_move(board, piece, (x, y), (dx, dy)) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }

    true
}

// Check if the move is a valid pawn promotion
pub fn is_valid_pawn_promotion(board: &Board, piece: &Piece, start: (usize, usize), end: (usize, usize), promotion: PieceType) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // Pawn promotion can only be performed by a pawn
    if piece.piece_type != PieceType::Pawn {
        return false;
    }

    // The pawn must be on its seventh rank
    if (piece.color == Color::White && start_x != 1) || (piece.color == Color::Black && start_x != 6) {
        return false;
    }

    // The destination square must be on the opponent's side of the board
    if (piece.color == Color::White && end_x != 0) || (piece.color == Color::Black && end_x != 7) {
        return false;
    }

    // The move must be a valid pawn move
    if !is_valid_pawn_move(board, piece, start, end) {
        return false;
    }

    // The promotion must be to a queen, rook, bishop, or knight
    match promotion {
        PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight => true,
        _ => false,
    }
}

// Perform a pawn promotion
pub fn perform_pawn_promotion(board: &mut Board, piece: &Piece, start: (usize, usize), end: (usize, usize), promotion: PieceType) {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    // Remove the pawn from the start square
    board[start_x][start_y] = None;

    // Place the promoted piece on the end square
    board[end_x][end_y] = Some(Piece {
        piece_type: promotion,
        color: piece.color,
    });
}