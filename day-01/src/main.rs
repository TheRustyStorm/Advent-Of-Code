use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("file not found");
    let mut already_seen_nums: HashMap<i64, i64> = HashMap::new();

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut acc = 0;
    let mut searching_repetitions = true;

    while searching_repetitions {
        for line in contents.lines() {
            let number_string = line.get(1..line.len()).unwrap();
            let number = number_string.parse::<i64>().unwrap();
            match line.get(0..1).unwrap() {
                "-" => acc -= number,
                "+" => acc += number,
                _ => println!("Error"),
            };
            if searching_repetitions {
                if already_seen_nums.contains_key(&acc) {
                    searching_repetitions = false;
                    println!("Found repetitions at {}", acc);
                } else {
                    already_seen_nums.insert(acc, 0);
                }
            }
        }
        println!("{}", acc);
    }
    
}
