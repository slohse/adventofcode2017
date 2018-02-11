use std::env;

fn manhattan_distance(num : i32) -> i32 {
    // the lower right corner in each concentric (geometric) square
    // is the (arithmetic) square of consecutive odd numbers, i.e.
    // 1, 9, 25, 49, ...
    // I use this as a base from which I determine where on
    // the (geometric) square the number in question is
    let ballpark = (num as f64).sqrt() as i32;

    let baseroot = if ballpark % 2 == 0 { ballpark - 1 } else { ballpark };
    let base = baseroot * baseroot;
    let diff_to_base = num - base;

    // if we happen to be on the corner with the squared odd number
    // it's fairly straight forward
    if diff_to_base == 0 {
        return baseroot - 1;
    }

    // if we are somewhere else, I found I can look at the rows that run along the
    // outside of our square, for example:
    //
    //  --- --- --- --- ---
    // | y | y | y | y | x |
    //  --- --- --- --- ---
    // | z | 5 | 4 | 3 | x |
    //  --- --- --- --- ---
    // | z | 6 | 1 | 2 | x |
    //  --- --- --- --- ---
    // | z | 7 | 8 | 9 | x |
    //  --- --- --- --- ---
    // | z | u | u | u | 25|
    //  --- --- --- --- ---
    //
    // let's say 9 is our base and thus 3 the baseroot, 4 the next_even
    // I then take the difference from the base and use the modulo operator % on it
    // This divides the outer edges of the square that I build from 9 onwards into
    // the stretches I marked with 'x', 'y', 'z' and 'u'. Going from any square in each of
    // these stretches to the middle can for example be accomplished by going to the middle
    // of the edge and then heading straight inward to the '1'. Going to the middle of the edge
    // is what the absolute value of
    // (next_even / 2) - (diff_to_base % next_even)
    // represents.
    // Going inward from there is (next_even / 2)
    //
    // for example, going from 13 is going down twice, then left twice
    //
    //  --- --- --- --- ---
    // | y | y | y | y | v |
    //  --- --- --- --- ---
    // | z | 5 | 4 | 3 | v |
    //  --- --- --- --- ---
    // | z | 6 | 1 | < | < |
    //  --- --- --- --- ---
    // | z | 7 | 8 | 9 | x |
    //  --- --- --- --- ---
    // | z | u | u | u | 25|
    //  --- --- --- --- ---
    let next_even = baseroot + 1;

    return (next_even / 2) +
        i32::wrapping_abs((next_even / 2) - (diff_to_base % next_even));
}

fn main() {
    let args : Vec<String> = env::args().collect();
    for argument in &args[1..] {
        let num = i32::from_str_radix(&argument[..], 10);
        match num {
            Err(msg) => println!("found non-number {} in input, error: {}", argument, msg),
            Ok(d) => println!("distance from {}: {}", d, manhattan_distance(d)),
        }
    }
}
