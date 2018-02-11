use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::LinkedList;


fn usize_or_bust(input: &str) -> usize {
    let num = usize::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn to_number_vector(input : &str) -> Vec<usize> {
    let mut numbers : Vec<usize> = Vec::new();

    for line in input.trim().split(',') {
        let num : usize = usize_or_bust(line);
        numbers.push(num);
    }

    return numbers;
}

fn start_list(len : usize) -> LinkedList<i32> {
    let mut start = LinkedList::new();
    for num in 0..len {
        start.push_back(num as i32);
    }

    return start;
}

fn reverse(numbers : &mut LinkedList<i32>, start : usize, length : usize) {
    let initial_len = numbers.len();
    let mut work = numbers.split_off(start);
    // println!("  work {:?}", work);
    let mut tail =
        if length < work.len() { work.split_off(length) }
            else {LinkedList::new()};

    // println!("  tail {:?}", tail);

    if start + length >= initial_len {
        let num_from_front = (start + length) % initial_len;
        // println!("  wrapping, taking {} from front", num_from_front);
        let mut core =
            if num_from_front < numbers.len() { numbers.split_off(num_from_front) }
                else { LinkedList::new() };
        work.append(numbers);
        // println!("  wrapping, work {:?}", work);
        numbers.append(&mut core);
        // println!("  wrapping, numbers (core) {:?}", numbers);

        let mut index :usize = 0;
        while index < num_from_front {
            numbers.push_front(work.pop_front().unwrap());
            index += 1;
        }
    }

    for num in work.iter().rev() {
        numbers.push_back(*num);
    }

    numbers.append(&mut tail);
}

fn knot(numbers : &mut LinkedList<i32>, lengths : &Vec<usize>) -> i32 {
    let mut skip_size = 0;
    let mut current_position= 0;

    for length in lengths {
        // println!("reversing {:?} at {} with length {}", numbers, current_position, length);
        reverse(numbers, current_position, *length);
        current_position = (current_position + length + skip_size) % numbers.len();
        skip_size += 1;
    }

    let mut it = numbers.iter();
    let first = it.next().unwrap_or(&0);
    let second = it.next().unwrap_or(&0);

    return first * second;
}


//############################################
// Part 2
//############################################

fn start_list2(len : usize) -> LinkedList<u8> {
    let mut start = LinkedList::new();
    for num in 0..len {
        start.push_back(num as u8);
    }

    return start;
}

fn reverse2(numbers : &mut LinkedList<u8>, start : usize, length : usize) {
    let initial_len = numbers.len();
    let mut work = numbers.split_off(start);
    // println!("  work {:?}", work);
    let mut tail =
        if length < work.len() { work.split_off(length) }
            else {LinkedList::new()};

    // println!("  tail {:?}", tail);

    if start + length >= initial_len {
        let num_from_front = (start + length) % initial_len;
        // println!("  wrapping, taking {} from front", num_from_front);
        let mut core =
            if num_from_front < numbers.len() { numbers.split_off(num_from_front) }
                else { LinkedList::new() };
        work.append(numbers);
        // println!("  wrapping, work {:?}", work);
        numbers.append(&mut core);
        // println!("  wrapping, numbers (core) {:?}", numbers);

        let mut index :usize = 0;
        while index < num_from_front {
            numbers.push_front(work.pop_front().unwrap());
            index += 1;
        }
    }

    for num in work.iter().rev() {
        numbers.push_back(*num);
    }

    numbers.append(&mut tail);
}

fn knot2(numbers : &mut LinkedList<u8>, lengths : &Vec<u8>) -> String {
    let mut skip_size = 0;
    let mut current_position= 0;

    for _round in 0..64 {
        for length in lengths {
            // println!("reversing {:?} at {} with length {}", numbers, current_position, length);
            reverse2(numbers, current_position, *length as usize);
            current_position = (current_position + (*length as usize) + skip_size) % numbers.len();
            skip_size += 1;
        }
    }

    let sparse = numbers.iter().collect::<Vec<&u8>>();
    let mut dense_hash = String::new();
    for i in 0..16 {
        let off = i * 16;
        let xor = sparse[off] ^ sparse[off + 1] ^ sparse[off + 2] ^ sparse[off + 3]
            ^ sparse[off + 4] ^ sparse[off + 5] ^ sparse[off + 6] ^ sparse[off + 7]
            ^ sparse[off + 8] ^ sparse[off + 9] ^ sparse[off + 10] ^ sparse[off + 11]
            ^ sparse[off + 12] ^ sparse[off + 13] ^ sparse[off + 14] ^ sparse[off + 15];
            dense_hash += &format!("{:02x}", xor);
    }

    return dense_hash;
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let lengths1= to_number_vector(&input);

    let mut suffix : Vec<u8> = vec![17, 31, 73, 47, 23];
    let mut lengths2= Vec::from(input.trim().as_bytes());
    lengths2.append(&mut suffix);

    let mut list1 = start_list(256);
    let mut list2 = start_list2(256);

    println!("Sum part 1: {}", knot(&mut list1, &lengths1));
    println!("Sum part 2: {}", knot2(&mut list2, &lengths2));

}
