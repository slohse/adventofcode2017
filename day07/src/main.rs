use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct Prog<'a> {
    weight : u32,
    parent: &'a str,
    children: Vec<String>,
}

fn u32_or_bust(input: &str) -> u32 {
    let num = u32::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}

fn parse_parent<'a>(def : &'a str) -> (&'a str, Prog<'a>) {
    let split = def.split_whitespace().collect::<Vec<&str>>();
    let name = split[0];
    let weightstr = split[1].trim();
    let weight = u32_or_bust(&weightstr[1..(weightstr.len() - 1)]);
    let progdef = Prog {
        weight,
        parent: "",
        children: Vec::new(),
    };

    return (name, progdef);
}

fn parse_children<'a>(children: &'a str, parent: &'a str, map : &mut HashMap<String, Prog<'a>>) -> Vec<String> {
    let mut list : Vec<String> = Vec::new();
    for child in children.split(",") {
        let trimmed = String::from(child.trim());
        if map.contains_key(&trimmed) {
//            println!("updating {} to have parent {}", trimmed, parent);
            map.get_mut(&trimmed).unwrap().parent = parent;
        } else {
//            println!("adding {} with parent {}", trimmed, parent);
            map.insert(trimmed.clone(), Prog { parent, weight: 0, children: Vec::new() });
        }
        list.push(trimmed);
    }
    return list;
}

fn to_progmap<'a>(input : &'a str) -> HashMap<String, Prog<'a>> {
    let mut progs : HashMap<String, Prog<'a>> = HashMap::new();

    for line in input.lines() {
        let parent_children = line.split(" -> ").collect::<Vec<&str>>();
        let (parent, mut def) = parse_parent(parent_children[0]);
        let mut has_children = false;

        if parent_children.len() > 1 {
            def.children = parse_children(parent_children[1], &parent, &mut progs);
            has_children = true;
//            println!("Added children {:?}", def.children);
        }

        if progs.contains_key(parent) {
//            println!("updating {} to have weight {}", parent, def.weight);
            let mut parentdef = progs.get_mut(parent).unwrap();
            parentdef.weight = def.weight;
            if has_children {
                parentdef.children = def.children;
            }
        } else {
//            println!("adding {} with unknown parent", parent);
            progs.insert(String::from(parent), def);

        }

    }

    return progs;
}



fn find_unbalanced(map : & HashMap<String, Prog>, root: &str, indent : usize) -> u32 {
    let rootnode = map.get(root).unwrap();

    let mut sum : u32 = 0;
    let mut weights : HashMap<u32, Vec<&str>> = HashMap::new();
    for child in rootnode.children.iter() {
        let childweight = find_unbalanced(map, child, indent + 2);
//        println!("{:>ind$} child {c} has weight {w}", ">", ind = indent, c = child, w = childweight);
        let weightcount = weights.entry(childweight).or_insert(Vec::new());
        weightcount.push(child);
        sum += childweight;
    }

    if weights.len() > 1 {
        println!("{:>ind$} found unbalanced tree at {r}: {w:?}", ">", ind = indent, w= weights, r=root);
    }

    sum += rootnode.weight;
//    println!("{:>ind$} root {r} has weight {w}", ">", ind = indent, r = root, w = sum);

    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please supply the input file as argument to the executable");
    }

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let progs = to_progmap(&input);

    let mut some_program : &str = progs.keys().next().unwrap();

    loop {
        let def = progs.get(some_program);
        if def.is_none() {
            panic!("Whoops, could not find program {}", some_program);
        }
        let parent : &str = def.unwrap().parent;
        if parent == "" {
            break;
        }
        some_program = parent;
    }

    println!("The root is {}", some_program);

    find_unbalanced(&progs, some_program, 0);

}