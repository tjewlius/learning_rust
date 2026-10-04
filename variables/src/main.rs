fn main() {
    // variables in Rust are immutable by default
    let x = 5;
    println!("The value of x is {x}");

    /*
     * x = 6;
     * println!("x = {}", x);
     *
     * this will not compile and returns an error E0384
     * error[E0384]: cannot assign twice to immutable variable `x`
     */

    // to make a variable mutable, use the `mut` keyword before the variable name
    let mut y = 5;
    println!("The value of y is {y}");
    y = 6;
    println!("The new value of y is {y}");

    /*
     * Constants
     * constants are declared with the `const` keyword
     * constants are immutable
     * constants data type must be specified at declaration
     * constants must be initialized with a value at declaration
     * constants are available throughout the module where they are declared
     */

    const SECONDS_IN_AN_HOUR: u32 = 60 * 60;
    const SECONDS_IN_A_DAY: u32 = SECONDS_IN_AN_HOUR * 24;
    const SECONDS_IN_A_WEEK: u32 = SECONDS_IN_A_DAY * 7;

    println!("There are {} seconds in an hour", SECONDS_IN_AN_HOUR);
    println!("There are {} seconds in a day", SECONDS_IN_A_DAY);
    println!("There are {} seconds in a week", SECONDS_IN_A_WEEK);
}
