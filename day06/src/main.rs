use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn u32_or_bust(input: &str) -> u32 {
    let num = u32::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn to_number_vector(input : &str) -> Vec<u32> {
    let mut numbers : Vec<u32> = Vec::new();

    for line in input.split_whitespace() {
        let num : u32 = u32_or_bust(line);
        numbers.push(num);
    }

    return numbers;
}

// If there are several elements that are equally maximum, Rust's max_by() from stdlib returns
// the last of them. We need the first of them, hence our own implementation here
fn find_largest(input: & Vec<u32>) -> (usize, u32) {
    let mut largest : (usize, & u32) = (0 as usize, input.first().unwrap());
    for current in input.iter().enumerate() {
        if current.1 > largest.1 {
            largest = current;
        }
    }
    return (largest.0, *(largest.1));
}

fn find_and_distribute(input: &mut Vec<u32>) {
    let num_of_banks = input.len();
    let largest = find_largest(input);

    let per_bank : u32 = largest.1 / (input.len() as u32);
    let remainder : usize = (largest.1 as usize) % input.len();

    for (index, used) in input.iter_mut().enumerate() {
        if index == largest.0 {
            *used = per_bank;
        } else {
            *used += per_bank;
        }
        if (index > largest.0 && index <= largest.0 + remainder)
            || (largest.0 + remainder >= num_of_banks && index <= (largest.0 + remainder) % num_of_banks) {
            *used += 1;
        }
    }

//    println!("New state: {:?}", input);
}

fn balance_memory_banks(input: &Vec<u32>) -> u32 {
    let mut previous_configs : HashMap<Vec<u32>, u32> = HashMap::new();
    let mut current_config = input.clone();
    let mut cycles : u32 = 0;

    let sanity_check = input.iter().fold( 0, |sum : u32, x| sum + x);

    previous_configs.insert(input.clone(), cycles);

    loop {
        find_and_distribute(&mut current_config);
        let sum = current_config.iter().fold( 0, |sum : u32, x| sum + x);
        if sum != sanity_check {
            println!("Sanity check failed, sum should be {}, was {}", sanity_check, sum);
            break;
        }
        cycles += 1;
        if previous_configs.contains_key(&current_config) {
            let loop_start = previous_configs.get(&current_config).unwrap();
            println!("Configuration already present in cycle {}, i.e. loop size is {}",
                     loop_start, cycles - loop_start);
            break;
        }

        previous_configs.insert(current_config.clone(), cycles);
    }

    return cycles;
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let memory_banks= to_number_vector(&input);

    println!("Cycles: {}", balance_memory_banks(&memory_banks));
}
