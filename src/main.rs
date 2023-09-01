use std::io;
use std::process::exit;
use crate::MatchStatus::{Nearly, Right, Wrong};
use colored::Colorize;
use r#typeof::type_of;


fn main() {

    let s2 = String::from("hellow");
    let a = s2.red();
    println!("{}", type_of(a));

    let mut vec = vec![104, 101, 108, 108, 111, 119];


    let str = String::from_utf8(vec).unwrap();
    println!("{str}");


    let answer = String::from("jumps");

    let mut rows = Vec::new();
    rows.push(String::from("sdfasdf"));
    rows.push(String::from("slkdwopeir"));



    let mut round = 0;

    loop {

        if (round == 6) {
            println!("Do you want to continue the game ? [Y/N]");
            let mut is_continue = String::new();
            io::stdin().read_line(&mut is_continue).unwrap();

            if is_continue.eq("Y") {
                round = 0;
            } else if is_continue.eq("N") {
                exit(0);
            } else {
                println!("unknown character, please input again. [Y/N]");
            }
        } else {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut right_count = 0;
            // for single_result in compare(&line, &answer) {
            //     match single_result {
            //         true => {
            //             right_count += 1;
            //
            //         }
            //         false => {}
            //     }
            // }

            round += 1;
        }
    }

}


fn println_vec(rows: &Vec<String>) {
    for str in rows {
        println!("{str}")
    }
}

fn compare(str1: &str, str2: &str) -> Vec<MatchStatus> {
    let bytes1 = str1.as_bytes();
    let bytes2 = str2.as_bytes();
    let mut result = Vec::new();

    for idx in 0..bytes1.len() {
        if bytes1[idx] == bytes2[idx] {
            result.push(Right(String::from("green")));
        } else if bytes2.contains(&bytes1[idx]){
            result.push(Nearly(String::from("yellow")));
        } else {
            result.push(Wrong(String::from("wrong")));
        }
    }
    result
}

enum MatchStatus {
    Right(String),
    Nearly(String),
    Wrong(String)
}
