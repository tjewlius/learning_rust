fn main() {
    // let age = 17;
    control_flow(101);
}

/*
 * all condit
 */

fn control_flow(mut num: u32) {
    while num > 0 {
        if num % 15 == 0 {
            println!("fizzbuzz : {}", num)
        } else if num % 3 == 0 {
            println!("fizz : {}", num)
        } else if num % 5 == 0 {
            println!("buzz : {}", num)
        }

        num -= 1;
    }
}
