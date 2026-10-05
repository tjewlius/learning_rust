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

    /*
     * variable shadowing
     * re-declaring a variable with the same name
     * the new value shadows the old value
     * while still keeping the variable immutable
     * shadowing always creates a new variable, not a reference
     * the shadowed variable can be of different data type than the original
     */

    let name = "John";
    println!("The name is {name}");

    {
        let name = "Jane"; // shadows the outer `name` variable
        println!("The name is {name}");
    }

    let name = "Henry"; // shadows the outer `name` variable
    println!("The name is {name}");

    let name = name.len(); // changed the data type of `name`
    println!("The name length is {name}");

    // name = "Bob"; // this will not compile and returns an error E0384
    // the variable `name` is immutable and cannot be reassigned just
}
