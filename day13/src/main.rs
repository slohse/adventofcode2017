use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn usize_or_bust(input: &str) -> usize {
    let num = usize::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn read_key_value(input : &str) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let kv : Vec<&str>= line.split(": ").collect();
        let key = usize_or_bust(kv[0]);
        let value = usize_or_bust(kv[1]);
        map.insert(key, value);
    }

    return map;
}

fn caught(step : usize, depth : usize) -> u32 {
    if step % ((depth - 1) * 2) == 0 {
        return (depth * step) as u32
    }
    return 0;
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let firewall = read_key_value(&input);

    let mut severity : u32 = 0;
    for (layer, depth) in firewall.clone() {
        severity += caught(layer, depth);
    }

    println!("total severity: {}", severity);

    // There are certainly analytical ways to solve this, but I'm too lazy too think right now
    let mut delay = 0;
    'delaybrute: loop {
        for (layer, depth) in firewall.clone() {
            if caught(layer + delay, depth) != 0 {
                delay += 1;
                continue 'delaybrute;
            }
        }
        break;
    }

    println!("minimal delay: {}", delay);

}
