use log::{trace};
use std::io;

fn get_max_joltage(battery_bank: &str) -> u64 {
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
 
    let mut total_joltage: u64 = 0;
    let lines = io::stdin().lines();
    for line in lines {
        let data = line.unwrap();
        trace!("Processing: {data}");
        total_joltage += get_max_joltage(&data);
    }

    println!("Total joltage: {total_joltage}");
}
