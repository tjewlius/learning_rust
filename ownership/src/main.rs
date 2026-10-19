/*
 * ===== OWNERSHIP IN RUST =====
 * Facts:
 * - Rust has no garbage collector like Java or Ruby
 * - Rust manages memory through ownership
 * - **Ownership: every value has one owner, one owner can exist at a time,
 *                value is dropped as soon as owner is out of scope
 * - values stored on the stack(integers, booleans, floats, arrays, tuples )
 *   transfer ownership through `Copy` trait i.e initial owner and new owner
 *   remain valid
 * - heap allocated values(Strings, Vec) ownership transfer is an expensive
 *   operation and all other types move by default where original owner is invalid
 *   when ownership is tranferred.
 * - to keep the original owner valid for heap allocated data, we explicitly
 *   use .clone()
 * - passing a value to a function also transfers ownership, to solve this we
 *   borrow the value if we still need to reuse it elsewhere in the program.
 */

fn main() {
    let a = String::from("Hello, world!");
    let _b = a; // here ownership is transferred "Hello, world!" string
    // is now owned by _b and println!("{a}") returns an error
    // println!("{a}"); // error[E0382]: borrow of moved value a
    // to keep a valid, we explicitly use .clone() when reassigning it to b
    let num1 = 20;
    let num2 = num1;
    println!("{} {}", num1, num2); // this is valid because values on the
    // stack are reassigned by `Copy`

    let s = String::from("String here");
    string_ownership_check(&s); // passing a value to a function also changes ownership
    // println!("{s}"); // this also returns error[E0382]
    num_ownership_check(num1); // num1 is still valid here even after reassignment and
    // passing it to function as only it's copy is used

    let t = String::from("This is a borrowed string");
    // borrowing_example(&t);

    println!("{t}");

    // borrowing_example();
}

fn string_ownership_check(s: &String) {
    println!("{s}");
}

fn num_ownership_check(num: i32) {
    println!("{num}");
}


// fn borrowing_example(){
//     let mut s = String::from("hello");

//     {
//         let m1 = &mut s;

//         m1.push_str("aa");

//         println!("{m1}")
//     }

//     let m2 = &mut s;
//     m2.push_str("bb");

//     println!("{m2}")
// }

// fn shouts(S: &mut String){
//     s.push_str("!!!");
//     s
// }