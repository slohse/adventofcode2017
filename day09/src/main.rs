use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::Chars;

fn garbage(it : &mut Chars) -> u32 {
    let mut char_count : u32 = 0;
    loop {
        match it.next() {
            None => panic!("whoops, input ended in the middle of garbage"),
            Some(d) => {
                if d == '>' {
                    break;
                } else if d == '!' {
                    it.next();
                } else {
                    char_count += 1;
                }
                // for some reason this match statement does not compile
//                match d {
//                    '>' => break,
//                    '!' => it.next(),
//                    _ => char_count += 1,
//                }
            }
        }
    }
    return char_count;
}

fn group(it : &mut Chars, depth : u32) -> (u32, u32) {
    let mut score = depth;
    let mut garbage_count : u32 = 0;
    loop {
        match it.next() {
            None => {
                if depth == 0 {
                    break;
                }
                else {
                    panic!("whoops, input ended in the middle of a group");
                }
            }
            Some(d) => {
                match d {
                    '{' => {
                        let (s, g) = group(it, depth + 1);
                        score += s;
                        garbage_count += g;
                    }
                    '<' => garbage_count += garbage(it),
                    '}' => break,
                    _ => (),
                };
            }
        };
    }
    return (score, garbage_count);
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    for line in input.lines() {
        let mut chars = line.chars();

//        println!("{}", line);
        let (score, garbage) = group(&mut chars, 0);
        println!("total score: {}, garbage characters: {}", score, garbage);
    }

}
