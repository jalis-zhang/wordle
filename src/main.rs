use std::io;
use std::process::exit;

fn main() {
    println!("Hello, world!");

    let mut rows = Vec::new();
    rows.push(String::from("sdfasdf"));
    rows.push(String::from("slkdwopeir"));

    println_vec(&rows);


    let row = rows.get(0).unwrap();
    println!("{row}");


    let compare_result = compare("sdf", "sdf");
    println!("{:?}", compare_result);


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
            round += 1;
        }
    }

}


fn println_vec(rows: &Vec<String>) {
    for str in rows {
        println!("{str}")
    }
}


fn compare(str1: &str, str2: &str) -> Vec<bool> {

    let bytes1 = str1.as_bytes();
    let bytes2 = str2.as_bytes();

    let mut result = Vec::new();

    for idx in 0..bytes1.len() {
        if bytes1[idx] == bytes2[idx] {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    result
}
