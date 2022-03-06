extern crate rand;
extern crate termion;

use rand::thread_rng;
use rand::Rng;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use termion::color;

#[derive(Copy, Clone, PartialEq)]
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

/**
 * Prints the board with colorings depending if the characters are in the word and in the right position.
 */
fn print_board(board: Board, row: usize) {
    println!();
    let mut current_row: usize = 0;
    for col in board {
        if current_row == row {
            print!(
                "   --> {}|{}",
                color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
                color::Bg(color::Reset)
            );
        } else {
            print!(
                "       {}|{}",
                color::Bg(color::Rgb(0x2c, 0x30, 0x32)),
                color::Bg(color::Reset)
            );
        }
        current_row += 1;
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
 *
 */
fn get_word(word_list: File) -> String {
    let reader = BufReader::new(word_list);

    let lines = reader.lines().collect::<Vec<_>>();
    let word_count = lines.len();

    let y: usize = thread_rng().gen_range(0, word_count - 1);

    match &lines[y] {
        Ok(x) => return x.to_string(),
        Err(_) => {
            print!("Error get_word");
            return String::new();
        }
    }
}
/**
 * Load a file containing the words for the game
*/
fn load_file(path_str: String) -> File {
    let path = Path::new(path_str.as_str());
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    return file;
}

/**
 * Validated that the input is a valid word. And it's in the dictionary.
 */
fn validate_input(input: String) -> i32 {
    if input.len() == 5 {
        return 0;
    } else {
        println!("Please enter a valid word!");
        return -1;
    }
}

/**
 * Copies the input in the corresponding row with the conditioning of the charactes
 */
fn handle_input(input: String, board: &mut Board, current_row: usize, game_word: String) {
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

fn check_win_con(row: Row) -> bool {
    for square in row {
        if square.cond == CharCond::Missplaced || square.cond == CharCond::None {
            return false;
        }
    }
    return true;
}

fn handle_win(won: bool) {
    if won {
        println!("You guessed the word!");
    } else {
        println!("You could not guess the word :(\nTry again!");
    }
}

fn main() {
    let word_list = load_file(String::from("./words/five.txt"));

    let mut input = String::new();
    let game_word = get_word(word_list);
    println!("Word gotten: {}", game_word.to_string());
    let mut board: Board = [[CharElement {
        character: ' ',
        cond: CharCond::None,
    }; 5]; 2];

    println!();
    println!("        Rustword!");

    let mut current_row = 0;
    loop {
        print_board(board, current_row);
        println!("Enter your guess:");
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        input.pop(); // remove enter

        if validate_input(input.to_string()) == 0 {
            handle_input(
                input.to_string(),
                &mut board,
                current_row,
                game_word.to_string(),
            );

            if current_row == board.len() - 1 || check_win_con(board[current_row]) {
                break;
            }
            current_row += 1;
        }
    }

    print_board(board, current_row);
    handle_win(check_win_con(board[current_row]));
}
