use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{RwLock, Arc};

enum ThreadState {
    Waiting(usize, usize),
    Processing(usize, usize),
}

fn i64_or_register_value(state : & HashMap<char, i64>, input: &str) -> i64 {
    let num = i64::from_str_radix(input, 10);
    match num {
        Err(_) => {
            let reg : char = (input.chars().collect::<Vec<char>>())[0];
            return *(state.get(&reg).unwrap_or(&0));
        }
        Ok(d) => return d,
    }
}

fn run_parallel_program<'a>(state : & mut HashMap<char, i64>, input : &'a String,
                            receiver : mpsc::Receiver<i64>, sender : mpsc::Sender<i64>,
                            my_state: Arc<RwLock<ThreadState>>,
                            other_state: Arc<RwLock<ThreadState>> ) -> usize {
    let prog = input.lines().collect::<Vec<&str>>();

    let prog_id = state.get(&'p').unwrap().clone();
    let mut pc = 0;
    let mut sent : usize = 0;
    let mut received : usize = 0;

    'interpretloop: loop {
        let cur_ins = prog[pc];
//        println!("pc: {}, ins: {}", pc, cur_ins);
        let tokens = cur_ins.split_whitespace().collect::<Vec<&str>>();

        let reg : char = (tokens[1].chars().collect::<Vec<char>>())[0];
        let ins = tokens[0];
        let val : i64 = i64_or_register_value(&state, tokens.get(2).unwrap_or(&"0"));
        match ins {
            "snd" => {
                let to_send = i64_or_register_value(&state, tokens.get(1).unwrap_or(&"0"));
//                println!("p{} sending {}", prog_id, to_send);
                sender.send(to_send).unwrap();
                sent += 1;
            },
            "set" => {
                state.insert(reg, val);
            },
            "add" => {
                let old : i64 = *(state.get(&reg).unwrap_or(&0));
                state.insert(reg, (old + val));
            },
            "mul" => {
                let old: i64 = *(state.get(&reg).unwrap_or(&0));
                state.insert(reg, (old * val));
            },
            "mod" => {
                let old: i64 = *(state.get(&reg).unwrap_or(&0));
                state.insert(reg, (old % val));
            },
            "rcv" => {
                {
                    let mut waitstate = my_state.write().unwrap();
                    *waitstate = ThreadState::Waiting(sent, received);
                }
                loop {
                    let received_val = receiver.try_recv();
                    match received_val {
                        Ok(d) => {
                            state.insert(reg, d);
                            received += 1;
                            {
                                let mut waitstate = my_state.write().unwrap();
                                *waitstate = ThreadState::Processing(sent, received);
                            }
//                            if received % 100 == 0 {
//                                println!("p{} received {}", prog_id, received);
//                            }
                            break;
                        },
                        _ => {},
                    }
                    {
                        let other_waitstate = other_state.read().unwrap();
                        match *other_waitstate {
                            ThreadState::Waiting(s, r) => {
//                                println!("both waiting, {} received {} of {}, other received {} of {}", prog_id, received, s, r, sent);
                                if s == received && r == sent {
//                                    println!("ending thread {} after {} received", prog_id, received);
                                    break 'interpretloop;
                                }
                            }
                            _ => {}
                        }
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            },
            "jgz" => {
                let regval = i64_or_register_value(&state, tokens.get(1).unwrap_or(&"0"));
                if regval > 0 {
                    pc = (pc as i64 + val) as usize;
                    continue;
                }
            },
            _ => panic!("p{}: Unexpected instruction {}", prog_id, ins),
        }

        pc += 1 as usize;
    }

    return sent;
}



fn run_program<'a>(state : & mut HashMap<char, i64>, input : &'a String) {
    let prog = input.lines().collect::<Vec<&str>>();

    let mut pc = 0;
    let mut last_sound = 0;

    loop {
        let cur_ins = prog[pc];
//        println!("{}", cur_ins);
        let tokens = cur_ins.split_whitespace().collect::<Vec<&str>>();

        let reg : char = (tokens[1].chars().collect::<Vec<char>>())[0];
        let ins = tokens[0];
        let val : i64 = i64_or_register_value(&state, tokens.get(2).unwrap_or(&"0"));
        match ins {
            "snd" => {
                last_sound = *(state.get(&reg).unwrap_or(&0));
            },
            "set" => {
                state.insert(reg, val);
            },
            "add" => {
                let old : i64 = *(state.get(&reg).unwrap_or(&0));
                state.insert(reg, (old + val));
            },
            "mul" => {
                let old: i64 = *(state.get(&reg).unwrap_or(&0));
//                println!("multiplying {} * {}", old, val);
                state.insert(reg, (old * val));
            },
            "mod" => {
                let old: i64 = *(state.get(&reg).unwrap_or(&0));
                state.insert(reg, (old % val));
            },
            "rcv" => {
                let regval = *(state.get(&reg).unwrap_or(&0));
//                println!("recovering if {} is larger 0 (is {})", reg, regval);
                if regval != 0 {
                    println!("last sound played was {}", last_sound);
                    return;
                }
            },
            "jgz" => {
                let regval : i64 = i64_or_register_value(&state, tokens.get(1).unwrap_or(&"0"));
//                println!("jumping if {} is larger 0 (is {})", reg, regval);
                if regval > 0 {
                    pc = (pc as i64 + val) as usize;
//                    println!("jumping {} to {}", val, pc);
                    continue;
                }
            },
            _ => panic!("Unexpected instruction {}", ins),
        }

        pc += 1 as usize;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please supply the input file as argument to the executable");
    }

    let mut f = File::open(args[1].clone()).expect("file not found");

    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    let input0 = input.clone();
    let input1 = input.clone();

    let mut state : HashMap<char, i64> = HashMap::new();

    run_program(&mut state, &input);

    let (tx0, rx1) = mpsc::channel();
    let (tx1, rx0) = mpsc::channel();

    let mut state0 : HashMap<char, i64> = HashMap::new();
    state0.insert('p', 0);
    let mut state1 : HashMap<char, i64> = HashMap::new();
    state1.insert('p', 1);

    let lock0 = Arc::new(RwLock::new(ThreadState::Processing(0, 0)));
    let lock1 = Arc::new(RwLock::new(ThreadState::Processing(0, 0)));

    let my_lock0 = lock0.clone();
    let other_lock0 = lock1.clone();

    let my_lock1 = lock1.clone();
    let other_lock1 = lock0.clone();

    let handle0 = thread::Builder::new().name("thread0".to_string()).spawn(move || {
//        let num_of_sends = run_parallel_program(&mut state0, &input0, rx0, tx0, my_lock0, other_lock0);
        run_parallel_program(&mut state0, &input0, rx0, tx0, my_lock0, other_lock0);
//        println!("prog0 sent {}", num_of_sends);
    }).unwrap();

    let handle1 = thread::Builder::new().name("thread1".to_string()).spawn(move || {
        let num_of_sends = run_parallel_program(&mut state1, &input1, rx1, tx1, my_lock1, other_lock1);
        println!("prog1 sent {}", num_of_sends);
    }).unwrap();

    let _sent0 = handle0.join();
    let _sent1 = handle1.join();

}
