use std::collections::VecDeque;

fn spinlock(stepsize : usize) -> u32 {
    let mut circular_buffer :VecDeque<u32> = VecDeque::new();

    circular_buffer.push_back(0);
    let mut pos = 0;
    for i in 1..2018 {
        let newpos = ((pos + stepsize) % circular_buffer.len()) + 1;
//        println!("len {}, newpos {}", circular_buffer.len(), newpos);
        circular_buffer.insert(newpos, i);
        pos = newpos;
    }

//    println!("Pos 0: {}, right of Pos 0: {} ", circular_buffer.get(0).unwrap(), circular_buffer.get(1).unwrap());

    return *(circular_buffer.get((pos + 1) % circular_buffer.len()).unwrap());
}

fn spinlock_part2(stepsize : usize) -> u32 {
    let mut pos = 0;
    let mut right_of_zero : u32 = 0;
    for i in 1..50000001 {
        let newpos = ((pos + stepsize) % i) + 1;
//        println!("i {}, newpos {}", i, newpos);
        if newpos ==  1 {
            right_of_zero = i as u32;
//            println!("{} is now right to zero", right_of_zero);
        }
        pos = newpos;
    }

    return right_of_zero;
}

fn main() {
    //let input = 3; // example
    let input = 359; // example

    println!("value to the right of 2017: {}", spinlock(input));

    println!("value to the right of 0: {}", spinlock_part2(input));

}
