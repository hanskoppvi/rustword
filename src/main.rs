use std::io;

type Row = [char; 5];
type Board = [Row; 2];

fn print_board(board: Board) {
    for col in board {
        print!("|");
        for square in col {
            print!("{}|", square);
        }
        println!();
    }
}

/**
 * Load a file containing the words for the game
*/
// fn load_file() {
//     // TODO:(hans) implement loading a file
// }

fn main() {
    let mut input = String::new();
    let game_word = String::from("decay");
    let mut board: Board = [[' '; 5]; 2];

    for i in 0..2 {
        print_board(board);
        println!("Enter your guess");
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        input.pop(); // remove enter
        if input.len() == 5 {
            for (j, c) in input.to_string().chars().enumerate() {
                if c == game_word.chars().nth(j).unwrap() {
                    board[i][j] = '1';
                } else if game_word.chars().position(|d| d == c) != None {
                    board[i][j] = '0';
                } else {
                    board[i][j] = c;
                }
            }
        }
    }
    print_board(board);
}
