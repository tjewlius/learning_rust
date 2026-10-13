fn main() {
    println!("Hello, world!");

    anonther_function(90, "John".to_string());
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

fn anonther_function(age: i8, name: String){
    println!("His name is {} and he is {} years old", name, age)
}