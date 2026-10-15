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
 *   operation and use the `Move` trait where original owner is invalid when
 *   ownership is tranferred.
 * - to keep the original owner valid for heap allocated data, we explicitly
 *   use .clone()
 * - passing a value to a function also transfers ownership, to solve this we
 *   borrow the value if we still need to reuse it elsewhere in the program.
 */


fn main() {
    println!("Hello, world!");
}
