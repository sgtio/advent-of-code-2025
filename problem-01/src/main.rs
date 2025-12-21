use std::io;
use log::trace;

fn main() -> io::Result<()> {
    /* Init log */
    simple_logger::SimpleLogger::new().init().unwrap();

    let mut position = 50;
    let mut password = 0;
    let mut password_pt2 = 0;
    let lines = io::stdin().lines();
    for line in lines {
        let mut data = line.unwrap();
        trace!("Input: {data}");

        /* An empty line is the end of our input */
        if data.len() == 0 {
            break;
        }

        /* First, parse the direction of turning */
        let rotation = data.remove(0);
        let direction = match rotation {
            'R' => 1,
            'L' => -1,
            _ => {
                panic!("Unexpected rotation: {rotation}");
            }
        };

        /* Then parse the offset */
        let mut offset:i32 = data.parse().expect("Failed to parse string to integer: {data}");

        /* For part 2, we want to see how many full circles we do with the offset */
        password_pt2 += offset/100;
        offset = offset % 100;

        /* And calculate the new position. */
        position += offset * direction;
        trace!("Position: {position}, offset: {offset}, direction: {direction}");

        /* If we passed through 0, and we were not already in 0, add to the part 2 password */
        if (position <= 0 || position >= 100) && position != offset * direction {
            trace!("Password part 2 increase");
            password_pt2 += 1;
        }

        /* Calculate the resulting position */
        position = position % 100;
        if position < 0 {
            position += 100;
        }
        trace!("Final position: {position}");

        /* And if we landed in 0, update part 1 password */
        if position == 0 {
            trace!("Password increase!");
            password += 1;
        }
    }

    println!("Password for part 1 is {password}");
    println!("Password for part 2 is {password_pt2}");
    Ok(())
}
