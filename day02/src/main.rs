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

fn to_number_vector(input : &str) -> Vec<Vec<i32>> {
    let mut numbers : Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let row : Vec<i32> = line.split('\t').map(|x| i32_or_bust(x)).collect();
        numbers.push(row);
    }

    return numbers;
}

fn diff_largest_smallest(numbers : &Vec<i32>) -> i32 {
    if numbers.is_empty() {
        println!("row is empty");
        return 0;
    }

    let mut largest = numbers[0];
    let mut smallest = numbers[0];

    for num in numbers {
        if *num > largest {
            largest = *num;
        }
        if *num < smallest {
            smallest = *num;
        }
    }

    return largest - smallest;
}

fn evenly_divisible_result(numbers : &Vec<i32>) -> i32 {
    let mut sorted = numbers.clone();
    sorted.sort();

    for (index, number) in sorted.iter().enumerate() {
        let mut tail = sorted.iter();
        tail.nth(index + 1);
        for item in tail {
            let remainder : i32 = item % number;
            //println!("{} % {} = {}", item, number, remainder);
            if remainder == 0 {
                //println!("is divisible!");
                return item / number;
            }
        }
    }

    return 0;
}

fn checksum(spreadsheet : &Vec<Vec<i32>>, f: fn(&Vec<i32>) -> i32) -> i32 {
    let mut sum :i32 = 0;

    for row in spreadsheet {
        sum += f(row)
    }

    return sum;
}



fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let spreadsheet = to_number_vector(&input);

    println!("checksum: {}", checksum(&spreadsheet, diff_largest_smallest));
    println!("evenly divisible checksum: {}", checksum(&spreadsheet, evenly_divisible_result));

}
