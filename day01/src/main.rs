use std::fs::File;
use std::io::prelude::*;

fn sum_neighboring_equal_digits(input : &str) -> u32 {
    let mut prevc :char = ' ';
    let mut sum :u32 = 0;
    let mut first :char = '0';
    for (i, ch) in input.char_indices() {
        if i == 0 {
            first = ch;
        }
        if input.len() > 1 && i == input.len() - 1 {
            if first == ch {
                let digit = ch.to_digit(10);
                match digit {
                    None => println!("found non-digit {} in input", ch),
                    Some(d) => sum += d,
                }
            }
        }
        if ch == prevc {
            let digit = ch.to_digit(10);
            match digit {
                None => println!("found non-digit {} in input", ch),
                Some(d) => sum += d,
            }
        }
        prevc = ch;
    }

    return sum;
}

fn sum_halfway_round_equal_digits(input : &str) -> u32 {
    let (left, right) = input.split_at(input.len() / 2);
    let mut sum :u32 = 0;
    for (lc, rc) in left.chars().zip(right.chars()) {
        if lc == rc {
            let digit = lc.to_digit(10);
            match digit {
                None => println!("found non-digit {} in input", lc),
                Some(d) => sum += d,
            }
        }
    }
    return sum * 2;
}

fn main() {

    let mut f = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("neighbor sum: {}", sum_neighboring_equal_digits(input.trim()));
    println!("halfway sum: {}", sum_halfway_round_equal_digits(input.trim()));

}
