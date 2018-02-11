use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn i32_or_bust(input: &str) -> i32 {
    let num = i32::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn less_than(x : i32, y : i32) -> bool {
    x < y
}

fn greater_than(x : i32, y : i32) -> bool {
    x > y
}

fn less_equal(x : i32, y : i32) -> bool {
    x <= y
}

fn greater_equal(x : i32, y : i32) -> bool {
    x >= y
}

fn equal(x : i32, y : i32) -> bool {
    x == y
}

fn not_equal(x : i32, y : i32) -> bool {
    x != y
}


fn test_condition<'a>(state : &'a HashMap<&str, i32>, register: &'a str, condition: &'a str, value: &'a str) -> bool {
    let val = i32_or_bust(value);

    let regval = state.get(register).unwrap_or(&0);

    let comp = match condition {
        "<" => less_than,
        ">" => greater_than,
        "<=" => less_equal,
        ">=" => greater_equal,
        "==" => equal,
        "!=" => not_equal,
        _ => panic!("Unexpected condition {}", condition),
    };

    return comp(*regval, val);
}

fn perform_instruction<'a>(state : & mut HashMap<&'a str, i32>, line : &'a str) {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();

    assert_eq!(tokens.len(), 7);

    if test_condition(state, tokens.get(4).unwrap(),
                      tokens.get(5).unwrap(), tokens.get(6).unwrap()) {
        let value = i32_or_bust(tokens.get(2).unwrap());
        let reg = tokens.get(0).unwrap();
        let op = tokens.get(1).unwrap();

//        println!("condition true, performing {} on {} with {}", op, reg, value);

        if !state.contains_key(reg) {
//            println!("inserting register {}", reg);
            state.insert(reg, 0);
        }

        let regval = state.get_mut(reg).unwrap();


        match *op {
            "inc" => *regval += value,
            "dec" => *regval -= value,
            _  => panic!("Unexpected op {}", op),
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please supply the input file as argument to the executable");
    }

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let mut state : HashMap<&str, i32> = HashMap::new();
    let mut max : i32 = 0;

    for line in input.lines() {
        perform_instruction(&mut state, line);
        if *(state.values().max().unwrap()) > max {
            max = *(state.values().max().unwrap());
        }
    }

    println!("largest register value at end is {:?}", state.clone().values().max().unwrap());
    println!("largest register value ever is {}", max);

}
