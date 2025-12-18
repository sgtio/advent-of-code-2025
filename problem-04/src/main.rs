use log::{trace};
use std::io;

/// Get the number of reachable rolls. This solves the first part of the problem.
/// It receives a vector of bytes, and the number of rows and colums. It returns
/// the number of reachable rolls.
fn get_free_rolls(rolls: &Vec<Vec<u8>>, rows: usize, cols: usize) -> u32 {
    let mut free_rolls = 0;

    for x in 0..cols {
        for y in 0..rows {
            if rolls[y][x] != b'@' {
                continue;
            }

            // Check how many rolls around
            let offset: [i32; 3]= [-1, 0, 1];
            let mut numrolls = 0;
            for x_offset in offset {
                for y_offset in offset {
                    let y_pos = y_offset + (y as i32);
                    let x_pos = x_offset + (x as i32);

                    // Check for out of bounds
                    if y_pos < 0 || y_pos >= rows as i32 ||
                       x_pos < 0 || x_pos >= cols as i32 {
                        continue;
                    }

                    // Skip checking the position we are evaluating
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    if rolls[y_pos as usize][x_pos as usize] == b'@' {
                        numrolls += 1;
                    }
                }
            }

            if numrolls < 4 {
                trace!("Roll with free space at {x},{y}");
                free_rolls += 1;
            }

        }
    }

    return free_rolls;
}

/// Remove the rolls that are reachable, iterating until no more rolls can be removed.
/// This function solves the second part of the problem. It receives a mutable vector of
/// bytes and returns the number of rolls it managed to remove.
fn rm_free_rolls(rolls: &mut Vec<Vec<u8>>, rows: usize, cols: usize) -> u32 {
    let mut free_rolls = 0;

    for x in 0..cols {
        for y in 0..rows {
            if rolls[y][x] != b'@' {
                continue;
            }

            // Check how many rolls around
            let offset: [i32; 3]= [-1, 0, 1];
            let mut numrolls = 0;
            for x_offset in offset {
                for y_offset in offset {
                    let y_pos = y_offset + (y as i32);
                    let x_pos = x_offset + (x as i32);

                    // Check for out of bounds
                    if y_pos < 0 || y_pos >= rows as i32 ||
                       x_pos < 0 || x_pos >= cols as i32 {
                        continue;
                    }

                    // Skip checking the position we are evaluating
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    if rolls[y_pos as usize][x_pos as usize] == b'@' {
                        numrolls += 1;
                    }
                }
            }

            if numrolls < 4 {
                trace!("Roll with free space at {x},{y}");
                rolls[y][x] = b'.';
                free_rolls += 1;
            }

        }
    }

    return free_rolls;
}

fn main() {
    /* Init log */
    simple_logger::SimpleLogger::new().init().unwrap();

    /* Create map of rolls */
    let mut rolls = Vec::new();
    let lines = io::stdin().lines();
    for line in lines {
        let data = line.unwrap();
        if data.len() > 0 {
            rolls.push(data.into_bytes());
        }
    }

    /* Assume that all rows have the same amount of chars */
    let rows = rolls.len();
    let cols = rolls[0].len();

    let first_part = get_free_rolls(&rolls, rows, cols);

    let mut second_part = 0;
    loop {
        // Here we iterate through the entire map of rolls and remove the rolls
        // we find. A more optimized implementation would iterate only once, and as
        // it finds rolls to remove, it would check which rolls would become available
        // by removing the roll. But I don't have time for that, sorry :)
        let rolls_removed = rm_free_rolls(&mut rolls, rows, cols);

        if rolls_removed == 0 {
                break;
        }

        second_part += rolls_removed;
    }

    println!("Number of accessible rolls (1st part): {first_part}");
    println!("Number of rolls removed (2nd part): {second_part}");
}
