use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut chars: HashMap<char, u32> = HashMap::new();
    let mut twice = 0;
    let mut thrice = 0;
    for line in contents.lines() {
        for c in line.chars() {
            let counter = chars.entry(c).or_insert(0);
            *counter += 1;
        }

        let mut can_be_2 = true;
        let mut can_be_3 = true;
        for (key, val) in chars.iter() {
            if *val == 2u32 && can_be_2 {
                twice += 1;
                can_be_2 = false;
            }
            if *val == 3u32 && can_be_3 {
                thrice += 1;
                can_be_3 = false;
            }
        }

        chars.clear();
    }

    println!("{} {} {}", twice, thrice, twice * thrice);

    for index_to_remove in 0..25 {
        let mut strings: HashMap<String, u32> = HashMap::new();
        for line in contents.lines() {
            let mut s = String::from(line);
            s.remove(index_to_remove);
            let counter = strings.entry(s).or_insert(0);
            *counter += 1;
        }
        for (key, val) in strings {
            if val == 2{
                println!("key: {} ", key);
            }
        }
    }
}
