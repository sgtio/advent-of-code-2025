use std::io;
use log::trace;

fn main() -> io::Result<()> {
    /* Init log */
    simple_logger::SimpleLogger::new().init().unwrap();

    let mut position = 50;
    let mut password = 0;
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
        let offset:i32 = data.parse().expect("Failed to parse string to integer: {data}");
        
        /* And calculate the new position. Assume offset will not overflow integer */
        position = (position + offset * direction) % 100;
        if position < 0 {
            position += 100;
        }
        trace!("Position: {position}, offset: {offset}, direction: {direction}");
        
        if position == 0 {
            trace!("Password increase!");
            password += 1;
        }
    }
    
    println!("Password is {password}");
    Ok(())
}
