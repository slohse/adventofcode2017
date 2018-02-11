use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn usize_or_bust(input: &str) -> usize {
    let num = usize::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn spin(state : &mut VecDeque<char>, size : &str) {
    let num = usize_or_bust(size);
    for _ in 0..num {
        let from_back = state.pop_back().unwrap();
        state.push_front(from_back);
    }
}

fn swap_programs(state : &mut VecDeque<char>, first : char, second : char) {
    let index_first = state.iter().position(|&x| x == first).unwrap();
    let index_second = state.iter().position(|&x| x == second).unwrap();

    state.swap(index_first, index_second);
}

fn initial_state() -> VecDeque<char> {
    let mut dance_state = VecDeque::new();

    for prog in 0..16 as u8 {
        dance_state.push_back(((('a' as u8) + prog) as char));
    }

    return dance_state;
}

fn dance(input : & str, mut dance_state : &mut VecDeque<char> ) {
    for mv in input.split(",") {
//        println!("move '{}'", mv);

        match mv.chars().nth(0).unwrap() {
            's' => spin(&mut dance_state, &mv[1..]),
            'x' => {
                let positions : Vec<&str> = (mv[1..]).split('/').collect();
                let first = usize_or_bust(positions[0]);
                let second = usize_or_bust(positions[1]);

                dance_state.swap(first, second);
            },
            'p' => swap_programs(&mut dance_state, mv.chars().nth(1).unwrap(), mv.chars().nth(3).unwrap()),
            _ => panic!("Unexpected dance move {}", mv.chars().nth(0).unwrap()),
        }
//        println!("({:?})", dance_state);
    }
}


fn main() {
    let args: Vec < String > = env::args().collect();

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string( & mut input).expect("something went wrong reading the file");

    let mut dance_state = initial_state();
    let init_state = String::from_iter(dance_state.clone());
    dance(input.trim(), &mut dance_state);
    let order_str = String::from_iter(dance_state.clone());

    println!("{}", order_str);

    let mut previous_orders = Vec::new();
    previous_orders.push(init_state.clone());
    previous_orders.push(order_str);

    let mut dance_rounds = 1;
    loop {
        dance(input.trim(), &mut dance_state);
        dance_rounds += 1;
        let ostr = String::from_iter(dance_state.clone());
        if ostr == init_state {
            println!("Initial state restored after {} dances", dance_rounds);
            let remainder = 1000000000 % dance_rounds;
            println!("remainder {}", remainder);
            let remainder_state = previous_orders.get(remainder).unwrap();
            println!("remainder state: {}", remainder_state);
            break;
        }
        previous_orders.push(ostr);
    }

}