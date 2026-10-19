fn main() {
    let s = String::from("Hello, world!");
    string_ownership_check(&s);

    println!("{}", s);

    let mut s2 = String::from("hello");
    borrowed_string_mutation(&mut s2);

    println!("{}", s2);

    let r1 = &s2;
    println!("{r1}");
}

/*
 * ==== BORROWING IN RUST ====
 * passing heap-allocated data to a function assigns ownership of that data to the
 * function and can not be reused.
 * to reuse the data without assigning ownership, we pass the data
 * as a reference ie add ampersand to the data annotation of the function parameters
 * as well as an ampersand to the arguments when calling the function
 * here the ampersand refers to the reference of the value without taking ownership
 * 
 * likewise the reference is dropped as soon as it is no longer user
 * 
 * like variables, references are immutable by default as well
 * 
 * to modify a borrowed value:
 * - the value must declared mutable
 * - the function parameter must be passed as mutable
 * - the caller function argument must be passed as mutable 
 * 
 * NOTES:
 * - you can have one mutable reference i.e you can not have other references to the
 *   mutated value `&mut` in the same scope -> &mut T
 * - you can multiple readers of an immutable reference `&` in the same scope -> &T
 * - ownership stays with the original variable, dropping references frees nothing
 * 
 * tldr; you can ,one mutation to a reference or unlimited number of reads of an immutable references 
 *       not both in the same scope
 */

fn string_ownership_check(s: &String){
    println!("{}", &s);
}

fn borrowed_string_mutation(s: &mut String){
    s.push_str(", world!!!!");
    println!("{}", s)
}