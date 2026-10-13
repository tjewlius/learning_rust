fn main() {
    println!("The area of this rectangle is {}", area(15, 27));

    println!("{:?}", fee(1500));

    let x = greet("Julio");

    println!("{x:?}");
}

/*
 * Rust does not care where you define a function as long as it is
 * in scope with the caller. it can be before or after the caller.
 *
 * functions are defined by the `fn` key word followed by the
 * function name, brackets(parentheses) and curly bracces to
 * hold the function body
 *
 * if a function has parameters(special variables passed to functions
 * for passing real values later), the parameter data type has to be
 * defined ie parameters require type anatonnations
 *
 *
 * Statements and expressions
 * statements do not return a value e.g function, variable declarations
 * expressions return a value eg calling a function, calling a macro
 *
 * statements end with a semicolon and ending an expression with a semicolon
 * turns it into a statement
 */

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn fee(transaction: i32) -> i32 {
    if transaction > 10_000 {
        transaction * 2 / 100
    } else {
        0
    }
}

fn greet(name: &str) {
    println!("Hello {name}!")
}
