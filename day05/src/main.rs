use std::fs::File;
use std::io::prelude::*;

fn i32_or_bust(input: &str) -> i32 {
    let num = i32::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn to_number_vector(input : &str) -> Vec<i32> {
    let mut numbers : Vec<i32> = Vec::new();

    for line in input.lines() {
        let num : i32 = i32_or_bust(line);
        numbers.push(num);
    }

    return numbers;
}

fn jump_around(mut input: Vec<i32>, part_two : bool) -> u32 {

    let mut index : i32 = 0;
    let mut jumps : u32 = 0;

    while index >= 0 && index < (input.len() as i32) {
        jumps += 1;
        let distance = input[(index as usize)];
        if part_two && distance >= 3 {
            input[(index as usize)] -= 1;
        } else {
            input[(index as usize)] += 1;
        }
        index += distance;
    }

    return jumps;
}

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let mut jumptable= to_number_vector(&input);

    println!("jumps: {}", jump_around(jumptable.clone(), false));
    println!("part two jumps: {}", jump_around(jumptable.clone(), true));

}