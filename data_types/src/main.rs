fn main() {
    /*
     * Rust is statically typed thus data types must be specified at compile time
     *
     * Data Types in Rust are divided into 2 subsets: Scalar and Compound
     * 1. Scalar types: Scalar types are data types that represent a single value
     * - integers (i8, i16, i32, i64, i128, isize) e.g 10, -3
     * --- integers are numbers without a decimal point
     * --- integers can be signed(either positive or negative) or unsigned (only positive)
     * --- signed integers are denoted with `i` and unsigned integers are denoted with
     *      `u` eg i8 is signed and u8 is unsigned
     * --- integers can also depend on the target architecture i.e isize(signed) && usize
     *       are 32 bits on 32-bit systems and 64 bits on 64-bit systems
     * --- primary use for isize and usize is when the size of the integer is not known
     *       at compile time / indexing a collection
     * - floats (f32, f64) e.g 3.14, 2.718, -3.14
     * - booleans (bool) e.g true, false
     * - characters (char) e.g 'a', 'b', 'A', 'Z' ** Chars use single quotes **
     *
     * 2. Compound types: rust has 2 primitive compound types:
     * - arrays
     * --- collections of values of the same data type
     * --- fixed length, they do not grow or shrink in size
     * --- indexed using square bracket notation
     * --- values are accessed using their index
     * *** arrays are useful when data will not change eg storing a list of months of the year
     *
     * - tuples
     * --- group multiple values of different types together
     * --- tuples have fixed length, they do not grow or shrink in size
     * --- tuples are indexed using dot(.) notation
     * --- tuples are essential for holding data about a single item with different
     *      data types
     *
     */

    let mut x: u8 = 200;
    println!("{x}");

    // x = 256; this will cause a compile error as 256 is out of the range of u8
    // println!("{x}");

    x = 29;
    println!("{x}");

    let tup = (21, 4.5, "Alice");
    println!("Her name is {}", tup.2); // used dot notation to access "Alice" in the tuple
    // println!("{}", tup.3); // accessing an element out of bounds of the tuple
    // returns error E0609 unknown field

    let (age, gpa, name) = tup; // destructuring a tuple
    println!("Age: {age}, GPA: {gpa}, Name: {name}");

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    println!("First month: {}", months[0]); // accessing the first element in the months array
    println!("Last month: {}", months[11]); // accessing the last element in the months  array
    // println!("{}", months[12]) // accessing an element out of bounds of the error returns
    // an index out of bounds error
}
