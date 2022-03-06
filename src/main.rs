extern crate termion;

use std::io;
use termion::color;

#[derive(Copy, Clone)]
enum CharCond {
    None,
    Right,
    Missplaced,
}

#[derive(Copy, Clone)]
struct CharElement {
    character: char,
    cond: CharCond,
}
type Row = [CharElement; 5];
type Board = [Row; 2];

fn print_board(board: Board) {
    println!();
    for col in board {
        print!(
            "       {}|{}",
            color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
            color::Bg(color::Reset)
        );
        for square in col {
            match square.cond {
                CharCond::None => print!(
                    "{}{}{}{}{}{}|{}",
                    color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
                    termion::style::Bold,
                    color::Fg(color::Rgb(0xe8, 0xe6, 0xe3)),
                    square.character,
                    color::Fg(color::Reset),
                    color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
                    color::Bg(color::Reset),
                ),
                CharCond::Missplaced => print!(
                    "{}{}{}{}{}{}|{}",
                    color::Bg(color::Rgb(0x91, 0x7f, 0x2f)),
                    termion::style::Bold,
                    color::Fg(color::Rgb(0xe8, 0xe6, 0xe3)),
                    square.character,
                    color::Fg(color::Reset),
                    color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
                    color::Bg(color::Reset),
                ),
                CharCond::Right => print!(
                    "{}{}{}{}{}{}|{}",
                    color::Bg(color::Rgb(0x42, 0x71, 0x3e)),
                    termion::style::Bold,
                    color::Fg(color::Rgb(0xe8, 0xe6, 0xe3)),
                    square.character,
                    color::Fg(color::Reset),
                    color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
                    color::Bg(color::Reset),
                ),
            }
        }
        println!();
    }
    println!();
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
    let mut board: Board = [[CharElement {
        character: ' ',
        cond: CharCond::None,
    }; 5]; 2];

    println!();
    println!("         Wordle!");

    for current_row in 0..board.len() {
        print_board(board);
        println!("Enter your guess:");
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        input.pop(); // remove enter
        if input.len() == 5 {
            for (current_col, input_char) in input.to_string().chars().enumerate() {
                board[current_row][current_col].character = input_char;
                // Is the current char from the input in this column the same as in the word.
                if input_char == game_word.chars().nth(current_col).unwrap() {
                    board[current_row][current_col].cond = CharCond::Right;
                }
                // Is the current char from the input in the word.
                else if game_word.chars().position(|d| d == input_char) != None {
                    board[current_row][current_col].cond = CharCond::Missplaced;
                }
            }
        }
    }
    print_board(board);
}
