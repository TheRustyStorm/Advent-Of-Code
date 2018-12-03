
#[macro_use]
extern crate regex;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn read_input() -> String{
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[derive(Debug)]
struct Square(usize, usize, usize, usize, usize);

fn parse(input :&str) -> Vec<Square>{
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    input.trim().lines().map(|line|{
        let caps = re.captures(line).unwrap();
        let values: Vec<usize> = caps.iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
            .collect();
            
        Square(values[0],values[1],values[2],values[3],values[4])
    }).collect()
}

fn main() {
    let input = read_input();
    let v = parse(&input);

    let mut field = [0u8; 1_000_000];
    for claim in &v{
        for row in claim.2..(claim.2 + claim.4){
            for column in claim.1..(claim.1 + claim.3){
                field[row * 1000 + column] += 1;
            }
        }
    }
    println!("{}",field.iter().filter(|&x| *x > 1).count());

    for claim in &v{
        let mut clean = true;
        for row in claim.2..(claim.2 + claim.4){
            for column in claim.1..(claim.1 + claim.3){
                if field[row * 1000 + column] != 1{
                    clean = false;
                }
            }
            
        }
        if clean{
                println!("{}", claim.0);
            }
    }
}
