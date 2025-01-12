use std::io::{self, Write};

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;

// 2D array type for the board
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE] // Initialize the board with empty spaces
}

fn print_board(board: &Board) {
    for row in board {
        for &cell in row {
            print!("{} ", cell); // Printing the cells with spaces between them
        }
        println!(); // Move to the next line after each row
    }
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Player {} input (row,col):", current_player);
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim input to remove excess whitespace and parse row and col
        let coordinates: Vec<usize> = input
            .trim()
            .split(',')
            .flat_map(str::parse)
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }

        println!("Invalid input, please try again.");
    }
}

fn check_winner(board: &Board, player: char) -> bool {
    // Check rows, columns, and diagonals for a winner
    for i in 0..BOARD_SIZE {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true; // Row win
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true; // Column win
        }
    }
    // Diagonal win
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true; 
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true; 
    }
    false
}

fn is_draw(board: &Board) -> bool {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if board[i][j] == ' ' {
                return false; 
            }
        }
    }
    true 
}

fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current Board:");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(&board, current_player) {
            println!("Player {} wins!", current_player);
            print_board(&board);
            break;
        }

        if is_draw(&board) {
            println!("It's a draw!");
            print_board(&board);
            break;
        }

        
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}

fn main() {
    println!("Welcome to Tic-Tac-Toe in Rust!");
    play_game();
}
