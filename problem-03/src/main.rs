use log::{trace};
use std::io;

/// This function is the generalization of the function below.
/// It receives as input parameter the number of batteries to pick
/// and the data for one battery bank. An assumption is made that the
/// number of batteries that we need to pick will not be larger
/// than the number of batteries in the bank.
fn get_max_joltage_pt2(num_batteries: u32, battery_bank: &str) -> u64 {

    let mut total_jolt = 0u64;
    let mut jolt_idx = 0;
    
    /* We start looking for batteries from the highest order of magnitude.
     * For every digit, we start at the position we found the last digit, and we
     * skip the last n items in the bank, where n is the amount of batteries
     * left to pick. */
    for i in 0..num_batteries {
        /* First, create an iterator. Consume the first 'jolt_idx' elements because the
         * previous digit was there, and we need to pick from the previous digit */
        let mut battery_chars = battery_bank.chars();
        for _ in 0..jolt_idx {
            battery_chars.next();
        }

        /* Then consume the 'num_batteries - i' elements from the back */
        let mut battery_rev = battery_chars.rev();
        for _ in 0..(num_batteries - i - 1) {
            battery_rev.next();
        }
     
       /* And find the maximum number left in the bank. */
       let mut battery_enum = battery_rev.rev().enumerate();
       let mut jolt_idx_delta = 0;
       let mut jolt = 0;
       while let Some((index, battery)) = battery_enum.next() {
           let candidate: u64 = battery.to_digit(10).unwrap().into();
           if jolt < candidate {
               jolt = candidate;
               jolt_idx_delta = index;
           }
       }
        
        /* Now calculate where we need to look for the next jolt */
        jolt_idx += jolt_idx_delta + 1;

        /* And update the total joltage */
        trace!("Found max jolt at {}: {jolt}", jolt_idx - 1);
        total_jolt += jolt * 10u64.pow(num_batteries - i - 1);
    }
    
    return total_jolt;
}

/// Calculate the joltage for part 1. This function is generalized in
/// part 2, but it's left for documenation purposes.
fn get_max_joltage_pt1(battery_bank: &str) -> u64 {
    /* First, number of decenes of jolts. This is the highest number
     * in the battery bank excluding the last character. The last character
     * is excluded because the units of jolts needs to be selected from the
     * digit selected for decajolts. */
    let mut decajolt: u64 = 0;
    let mut decajolt_idx: usize = 0;
    
    /* Remove last element */
    let mut bank_chars = battery_bank.chars();
    bank_chars.next_back();
    
    /* Convert to Enumerate so that we also get the index */
    let mut decajolt_iterator = bank_chars.enumerate();

    while let Some((index, battery)) = decajolt_iterator.next() {
        let candidate: u64 = battery.to_digit(10).unwrap().into();
        if decajolt < candidate {
            decajolt = candidate;
            decajolt_idx = index;
        }
    }
    trace!("Found decajolt at {decajolt_idx}: {decajolt}");

    /* Look for the units of the total joltage. Start from where we found
    * the decajolts. */
    let mut jolt: u64 = 0;
    let mut jolt_iterator = battery_bank.chars().skip(decajolt_idx + 1);
    while let Some(battery) = jolt_iterator.next() {
        let candidate: u64 = battery.to_digit(10).unwrap().into();

        if jolt < candidate {
            jolt = candidate;
        }
    }
    trace!("Found jolt: {jolt}");

    return decajolt * 10 + jolt;
}

fn main() {
    /* Init log */
    simple_logger::SimpleLogger::new().init().unwrap();
 
    let mut total_joltage_pt1 = 0u64;
    let mut total_joltage_pt2 = 0.0f64;
    let lines = io::stdin().lines();
    for line in lines {
        let data = line.unwrap();
        if data.len() == 0 {
            // Ignore empty lines
            continue;
        }
        trace!("Processing: {data}");
        total_joltage_pt1 += get_max_joltage_pt1(&data);
        total_joltage_pt2 += get_max_joltage_pt2(12, &data) as f64;
    }

    println!("Total joltage (Part 1): {total_joltage_pt1}");
    println!("Total joltage (Part 2): {total_joltage_pt2}");
}
