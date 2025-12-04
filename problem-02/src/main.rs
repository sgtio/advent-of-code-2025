use log::{trace, debug};
use std::io;

fn get_invalid_id_sum(min: &str, max: &str) -> f64 {
    let mut invalid_id_sum = 0.0;
    
    let min_digits = min.len();
    if min_digits == 0 {
        panic!("The length of the minimum range is 0. Input: min: {min}, max: {max}");
    }

    /* All invalid IDs are formed as 'nn', where n is a natural number. First,
     * figure out which is the 'first' 'n' which we need to check by splitting the
     * number in half. If the number cannot be expressed as the concatenation of two
     * natural numbers, we take the next power of 10. E.g. for 333, n = 10, and the
     * first invalid ID is 1010. */
    let n: String;
    if min_digits % 2 == 0 {
        let (half, _) = min.split_at(min_digits/2);
        n = half.to_string();
    } else {
        let (half, _) = min.split_at((min_digits - 1) / 2);
        n = 10u32.pow(half.len().try_into().unwrap()).to_string();
    }
    
    /* Form the candidates 'nn' and check if they are in range. When we get to
     * an n that produces an 'nn' that is out of range, exit. */
    let max_num: u64 = max.parse().unwrap();
    let min_num: u64 = min.parse().unwrap();
    let mut n_num: u64 = n.parse().unwrap();
    trace!("Min: {min}, Max: {max}, n: {n}");
    loop {
        let n_magnitude = n_num.ilog10() + 1;
        let candidate = n_num + n_num * 10u64.pow(n_magnitude);
        trace!("Evaluating n_num: {n_num}, candidate: {candidate}, n_magnitude: {n_magnitude}");
        if candidate < min_num {
            n_num += 1;
            continue;
        }
        else if candidate > max_num {
            debug!("{candidate} outside of range. Invalid ID sum for range: {invalid_id_sum}");
            break; 
        }

        /* Compute the sum in floating point to avoid overflow.
        * Operating with integers up until the sum will avoid rounding logic flaws,
        * but it requires that the minimum and maximum range fit within the size of the biggest
        * unsigned int we can store */
        debug!("Invalid ID found {candidate}");
        invalid_id_sum += candidate as f64;
        n_num += 1;
    }
    
    return invalid_id_sum;
}

fn main() {
    /* Init log */
    simple_logger::SimpleLogger::new().init().unwrap();
    
    let lines = io::stdin().lines();
    let mut invalid_id_sum: f64= 0.0;
    for line in lines {
        let data = line.unwrap();
        for range in data.split(',') {
            trace!("Range read: '{range}'");
            let (min, max) = range.split_once('-').unwrap();
            debug!("Testing range min: {min}, max: {max}");
            invalid_id_sum += get_invalid_id_sum(min, max);
        }
    }
    
    println!("Result: {invalid_id_sum}");
}
