mod function_giving_ownership;
mod references;
mod linked_list_safe;
mod mini_linked_list;

fn _using_string() {
    let mut hw = String::from("Hello World");
    hw.push_str(", User");
    println!("{hw}");
}

fn _borrowing() {
    let s1 = String::from("value");
    let s2 = s1;

    // Can't use s1 anymore, will throw error[E0382]: borrow of moved value: `s1` 
    println!("{s2}");

    let s3 = s2.clone();

    println!("{s3}")
}

fn _function_ownership() {
    let s = String::from("hello");  // s comes into scope

    _takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("{s}"); Error: borrow of moved value: `s`

    let x = 5;                      // x comes into scope

    _makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn _takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn _makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn main() {
    references::references_main();
}
