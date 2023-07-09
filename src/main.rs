mod rules;
use crate::rules::is_valid_move;


#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

// Define a type alias for the chess board, which is an 8x8 array of optional pieces
pub type Board = Vec<Vec<Option<Piece>>>;

// Define a function to create an empty chess board
fn empty_board() -> Board {
    vec![vec![None; 8]; 8]
}

// Define a function to create a chess board with the starting position
fn starting_position() -> Board {
    let mut board = empty_board();

    // Place black pawns on the second row
    for i in 0..8 {
        board[1][i] = Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        board[6][i] = Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
    }

    // Place black and white back row pieces in their starting positions
    let back_row = [
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

    for (i, piece_type) in back_row.iter().enumerate() {
        board[0][i] = Some(Piece { piece_type: piece_type.clone(), color: Color::Black });
        board[7][i] = Some(Piece { piece_type: piece_type.clone(), color: Color::White });
    }

    board
}

// Define a function to get the possible moves for a pawn at a given position
pub fn get_pawn_moves(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Get the piece at the given position
    let piece = match board[x][y] {
        Some(ref piece) => piece,
        None => return moves,  // If there is no piece at this position, return an empty vector
    };

    // Check the possible moves for a white or black pawn
    match piece.color {
        Color::White => {
            if x > 0 && board[x - 1][y].is_none() {
                moves.push((x - 1, y));  
            }
            if x > 0 && y < 7 && board[x - 1][y + 1].is_some() {
                moves.push((x - 1, y + 1));
            }
            if x > 0 && y > 0 && board[x - 1][y - 1].is_some() {
                moves.push((x - 1, y - 1));
            }
        },
        Color::Black => {
            if x < 7 && board[x + 1][y].is_none() {
                moves.push((x + 1, y)); 
            }
            if x < 7 && y < 7 && board[x + 1][y + 1].is_some() {
                moves.push((x + 1, y + 1));
            }
            if x < 7 && y > 0 && board[x + 1][y - 1].is_some() {
                moves.push((x + 1, y - 1));
            }
        },
    }

    moves
}

// Define a function to get the possible moves for a rook at a given position
pub fn get_rook_moves(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Get the piece at the given position
    let piece = match board[x][y] {
        Some(ref piece) => piece,
        None => return moves,  // If there is no piece at this position, return an empty vector
    };

    // Check the possible moves for a rook
    // Vertical moves
    for i in (0..x).rev() {
        match board[i][y] {
            Some(_) => {
                moves.push((i, y));
                break;
            },
            None => moves.push((i, y)),
        }
    }
    for i in x+1..8 {
        match board[i][y] {
            Some(_) => {
                moves.push((i, y));
                break;
            },
            None => moves.push((i, y)),
        }
    }

    // Horizontal moves
    for i in (0..y).rev() {
        match board[x][i] {
            Some(_) => {
                moves.push((x, i));
                break;
            },
            None => moves.push((x, i)),
        }
    }
    for i in y+1..8 {
        match board[x][i] {
            Some(_) => {
                moves.push((x, i));
                break;
            },
            None => moves.push((x, i)),
        }
    }

    moves
}

// Returns a vector of valid moves for a knight at position (x, y) on the given board
pub fn get_knight_moves(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Get the piece at the given position, if there is one
    let piece = match board[x][y] {
        Some(ref piece) => piece,
        None => return moves,  // If there is no piece at this position, return an empty vector
    };

    // Define the possible offsets for a knight's move
    let offsets = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    // Check each possible move and add it to the vector if it is valid
    for &(dx, dy) in offsets.iter() {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
            moves.push((nx as usize, ny as usize));
        }
    }

    moves
}

// Returns a vector of valid moves for a bishop at position (x, y) on the given board
pub fn get_bishop_moves(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Get the piece at the given position, if there is one
    let piece = match board[x][y] {
        Some(ref piece) => piece,
        None => return moves,  // If there is no piece at this position, return an empty vector
    };

    // Define the possible diagonal directions for a bishop's move
    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    // Check each possible move in each direction and add it to the vector if it is valid
    for &(dx, dy) in directions.iter() {
        let mut i = 1;
        while let Some((nx, ny)) = (x as i32 + dx * i).checked_rem_euclid(8)
            .zip((y as i32 + dy * i).checked_rem_euclid(8)) {
            match board[nx as usize][ny as usize] {
                Some(_) => {
                    moves.push((nx as usize, ny as usize));
                    break;
                },
                None => moves.push((nx as usize, ny as usize)),
            }
            i += 1;
        }
    }

    moves
}

// Returns a vector of valid moves for a queen at position (x, y) on the given board
pub fn get_queen_moves(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Get the piece at the given position, if there is one
    let piece = match board[x][y] {
        Some(ref piece) => piece,
        None => return moves,  // If there is no piece at this position, return an empty vector
    };

    // Combine the valid moves for a rook and a bishop to get the valid moves for a queen
    moves.extend(get_rook_moves(board, x, y));
    moves.extend(get_bishop_moves(board, x, y));

    moves
}

// Returns a vector of valid moves for a king at position (x, y) on the given board
pub fn get_king_moves(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Get the piece at the given position, if there is one
    let piece = match board[x][y] {
        Some(ref piece) => piece,
        None => return moves,  // If there is no piece at this position, return an empty vector
    };

    // Define the possible offsets for a king's move
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // Check each possible move and add it to the vector if it is valid
    for &(dx, dy) in offsets.iter() {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
            moves.push((nx as usize, ny as usize));
        }
    }

    moves
}


use std::io::{self, Write};

// Function to print the current state of the board
fn print_board(board: &Board) {
    println!("  a b c d e f g h");
    for (i, row) in board.iter().enumerate() {
        print!("{}", 8 - i);
        for square in row {
            print!(" ");
            match square {
                Some(piece) => print!("{}", match piece.color {
                    Color::White => match piece.piece_type {
                        PieceType::Pawn => "P",
                        PieceType::Rook => "R",
                        PieceType::Knight => "N",
                        PieceType::Bishop => "B",
                        PieceType::Queen => "Q",
                        PieceType::King => "K",
                    },
                    Color::Black => match piece.piece_type {
                        PieceType::Pawn => "p",
                        PieceType::Rook => "r",
                        PieceType::Knight => "n",
                        PieceType::Bishop => "b",
                        PieceType::Queen => "q",
                        PieceType::King => "k",
                    },
                }),
                None => print!("."),
            }
        }
        println!();
    }
}

// Function to read a move from the user
fn read_move() -> io::Result<(usize, usize, usize, usize)> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 5 || chars[2] != ' ' {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"));
    }
    let start_x = 8 - chars[1].to_digit(10).unwrap() as usize;
    let start_y = chars[0].to_digit(36).unwrap() as usize - 10;
    let end_x = 8 - chars[4].to_digit(10).unwrap() as usize;
    let end_y = chars[3].to_digit(36).unwrap() as usize - 10;
    Ok((start_x, start_y, end_x, end_y))
}

fn print_menu() {
    println!("Welcome to Rust Chess!");
    println!("1. Start a new game");
    println!("2. Load a saved game");
    println!("3. Instructions");
    println!("4. Quit");
}

fn print_instructions() {
    println!("Instructions:");
    println!("Enter your move in the format 'e2 e4', where 'e2' is the starting square and 'e4' is the ending square.");
    println!("Pawns are represented by 'p' or 'P', rooks by 'r' or 'R', knights by 'n' or 'N', bishops by 'b' or 'B', queens by 'q' or 'Q', and kings by 'k' or 'K'.");
    println!("Lowercase letters represent black pieces and uppercase letters represent white pieces.");
    println!("The game ends when a king is in checkmate (the king is in a position to be captured next turn and there is no way to move the king out of capture).");
}

fn start_new_game() {
    let mut board = starting_position();
    let mut current_color = Color::White;

    loop {
        print_board(&board);
        println!("{} to move", match current_color {
            Color::White => "White",
            Color::Black => "Black",
        });
        print!("Enter move: ");
        io::stdout().flush().unwrap();
        match read_move() {
            Ok((start_x, start_y, end_x, end_y)) => {
                let piece = match board[start_x][start_y] {
                    Some(ref piece) => piece.clone(),
                    None => {
                        println!("No piece at that square");
                        continue;
                    }
                };
                if piece.color != current_color {
                    println!("Not your piece");
                    continue;
                }
                if !is_valid_move(&board, &piece, (start_x, start_y), (end_x, end_y)) {
                    println!("Invalid move");
                    continue;
                }
                board[end_x][end_y] = Some(piece);
                board[start_x][start_y] = None;
                current_color = match current_color {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
            }
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        }
    }
}

fn main() {
    loop {
        print_menu();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(1) => {
                // Start a new game
                start_new_game();
            },
            Ok(2) => {
                // Load a saved game
                // TODO: Implement game saving and loading
            },
            Ok(3) => {
                // Print instructions
                print_instructions();
            },
            Ok(4) => {
                // Quit the game
                break;
            },
            _ => {
                println!("Invalid choice");
            },
        }
    }
}