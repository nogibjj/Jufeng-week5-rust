use std::io::{self, Write};

pub struct Board {
    board: [char; 9],
    current_player: char,
}

impl Board {
    fn new() -> Board {
        Board {
            board: [' '; 9],
            current_player: 'X',
        }
    }

    fn draw_board(&self) {
        println!(" {} | {} | {}", self.board[0], self.board[1], self.board[2]);
        println!("-----------");
        println!(" {} | {} | {}", self.board[3], self.board[4], self.board[5]);
        println!("-----------");
        println!(" {} | {} | {}", self.board[6], self.board[7], self.board[8]);
    }

    fn make_move(&mut self, position: usize) -> bool {
        if self.board[position] == ' ' {
            self.board[position] = self.current_player;
            self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
            true
        } else {
            false
        }
    }

    fn check_win(&self) -> bool {
        // Check rows
        for i in (0..9).step_by(3) {
            if self.board[i] != ' ' && self.board[i] == self.board[i + 1] && self.board[i + 1] == self.board[i + 2] {
                return true;
            }
        }

        // Check columns
        for i in 0..3 {
            if self.board[i] != ' ' && self.board[i] == self.board[i + 3] && self.board[i + 3] == self.board[i + 6] {
                return true;
            }
        }

        // Check diagonals
        if self.board[0] != ' ' && self.board[0] == self.board[4] && self.board[4] == self.board[8] {
            return true;
        }

        if self.board[2] != ' ' && self.board[2] == self.board[4] && self.board[4] == self.board[6] {
            return true;
        }

        false
    }
}

fn main() {
    let mut board = Board::new();
    let mut position: usize;
    loop {
        board.draw_board();
        print!("{}'s turn. Enter a position (1-9): ", board.current_player);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        position = match input.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => continue,
        };

        if position < 9 {
            if board.make_move(position) {
                if board.check_win() {
                    board.draw_board();
                    println!("{} wins!", board.current_player);
                    break;
                }
            } else {
                println!("Position already taken.");
            }
        } else {
            println!("Invalid position.");
        }
    }
}
