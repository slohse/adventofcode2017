use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn step_sums(input : &str) -> HashMap<String, u32> {
    let mut steps : HashMap<String, u32> = HashMap::new();

    for step in input.split(',') {
        let counter = steps.entry(String::from(step)).or_insert(0);
        *counter += 1;
    }

    return steps;
}

fn minimize_steps(steps : &mut HashMap<String, u32>) {
    // {n,s}, {ne, sw} and {se,nw} cancel each other out, so we really only care about their
    // respective difference
    let ns = *(steps.entry(String::from("n")).or_insert(0)) as i32 -
        *(steps.entry(String::from("s")).or_insert(0)) as i32;
    let nesw = *(steps.entry(String::from("ne")).or_insert(0)) as i32 -
        *(steps.entry(String::from("sw")).or_insert(0)) as i32;
    let nwse = *(steps.entry(String::from("nw")).or_insert(0)) as i32 -
        *(steps.entry(String::from("se")).or_insert(0)) as i32;

    println!("N-S: {}, NE-SW: {}, NWSE: {}", ns, nesw, nwse);

    // furthermore
    // (SW + N) == (NW)
    // (SE + N) == (NE)
    // (NW + S) == (SW)
    // (NE + S) == (SE)

}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let mut steps = step_sums(&input);
    minimize_steps(&mut steps);

}
