use std::io;
use crate::MatchStatus::{Nearly, Right, Wrong};
use colored::{ColoredString, Colorize};

fn main() {
    println!("Welcome into my wordle game, please enter a word with six letter six times");
    println!("game starting");
    let answer = String::from("please\n");
    let mut display_vec: Vec<Vec<ColoredString>> = Vec::new();
    loop {
        let mut round_result = false;
        for _round in 0..6 {
            round_result |= round_game(&answer, &mut display_vec);
            if round_result {
                println!("Congratulations, you win!!!");
                break;
            }
        }
        if !round_result {
            println!("Sorry, you lose!!!");
        }
        if !new_round() {
            break;
        }
        display_vec.clear();
    }
}

fn new_round() -> bool {
    let mut str = String::new();
    println!("Do you wanna try again? [Y/N]");
    io::stdin().read_line(&mut str).unwrap();
    if str == "Y\n" {
        true
    } else if str == "N\n"{
        println!("Bye, see you next time");
        false
    } else {
        println!("Unrecognized character, Bye");
        false
    }
}

fn round_game(answer: &String, display_vec: &mut Vec<Vec<ColoredString>>) -> bool {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    while !check_input(&line) {
        println!("Input error line, please input again");
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
    }
    let (result_vec, is_right) = compare(&line, answer);
    let string_vec = string_to_vec(line);
    flush_display_vec(display_vec, result_vec, string_vec);
    println_vec(display_vec);
    is_right
}

fn check_input(input: &String) -> bool{
    input.len() == 7
}

fn string_to_vec(str: String) -> Vec<String> {
    let mut result = Vec::new();
    let bytes = str.into_bytes();
    for idx in 0..bytes.len()-1 {
        result.push(String::from_utf8(vec![bytes[idx]]).unwrap());
    }
    result
}

fn flush_display_vec(display_vec: &mut Vec<Vec<ColoredString>>, compare_result: Vec<MatchStatus>,
string_vec: Vec<String>) {
    let mut row: Vec<ColoredString> = Vec::new();

    for idx in 0..compare_result.len() {
        let str = &string_vec[idx];
        let result = &compare_result[idx];
        match result {
            Right => {
                row.push(str.green())
            }
            Nearly => {
                row.push(str.yellow())
            }
            Wrong => {
                row.push(str.red())
            }
        }
    }
    display_vec.push(row)
}

fn println_vec(rows: &Vec<Vec<ColoredString>>) {
    for row in rows {
        for item in row {
            print!("{item} ")
        }
        println!()
    }
}

fn compare(str1: &str, str2: &str) -> (Vec<MatchStatus>, bool) {
    let bytes1 = str1.as_bytes();
    let bytes2 = str2.as_bytes();
    let mut result = Vec::new();
    let mut correct_count = 0;
    for idx in 0..bytes1.len()-1 {
        if bytes1[idx] == bytes2[idx] {
            result.push(Right);
            correct_count += 1;
        } else if bytes2.contains(&bytes1[idx]){
            result.push(Nearly);
        } else {
            result.push(Wrong);
        }
    }
    (result, correct_count==6)
}

enum MatchStatus {
    Right,
    Nearly,
    Wrong
}
