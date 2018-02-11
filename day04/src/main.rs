use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str;

fn no_anagrams(input: & str) -> i32 {
    let mut sum = 0;
    'lines: for line in input.lines() {
        let mut words = HashSet::new();
        for word in line.split_whitespace() {
            let mut sorted_letters = Vec::from_iter(word.chars());
            sorted_letters.sort();
            let sorted_word = String::from_iter(sorted_letters);
            if words.contains(&sorted_word) {
                continue 'lines;
            } else {
                words.insert(sorted_word);
            }
        }
        sum += 1;
    }

    return sum;
}

fn no_duplicate_words(input: & str) -> i32 {
    let mut sum = 0;
    'lines: for line in input.lines() {
        let mut words = HashSet::new();
        for word in line.split_whitespace() {
            if words.contains(word) {
                continue 'lines;
            } else {
                words.insert(word);
            }
        }
        sum += 1;
    }

    return sum;
}

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("num of valid passphrases: {}", no_duplicate_words(&input[..]));
    println!("num of valid passphrases: {}", no_anagrams(&input[..]));
}
