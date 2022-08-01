// use std::ops::RangeFrom;

fn main() {
    run(0, 10);
}

/**
 * This function prints in the terminal whether an integer is an even
 *  or odd number given a range of number.
 * 
 * It accepts two parameters:
 * start: start value of range
 * limit: end value of range
 * 
 * Example:
 * 
 * run(0, 10);
 * 
 * Should print:
 * 0 is an even number
 * 1 is an odd number
 * 2 is an even number
 * 3 is an odd number
 *          .
 *          .
 *          .
 * 9 is an odd number
 */
pub fn run(start: u32, limit: u32) {
    // Write your code here
    let mut count = start;
    while count < limit{
    
    if count % 2 == 0  {
        // n is even
        println!("{}:is even", count );
    }
    else {
        //otherwise odd
        println!("{}:is odd", count );
    }
    count = count + 1;
    }
}
