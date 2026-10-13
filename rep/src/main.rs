fn main() {
    let mut balance = 1000;

    balance -= 275;

    println!("Your balance is: {}", balance);


    let name = "Alice";
    // name = "Bob"; // This line will cause a compile-time error because `name` is immutable
    println!("Your name is: {}", name);

    // let num: u32 = -42; // This line will cause a compile-time error because `-42` is not a valid `u32` value

    // println!("The number is: {}", num);
}
