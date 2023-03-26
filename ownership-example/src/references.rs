pub(crate) fn references_main() {
    //TO DO: Create a function that uses references and the one that doesn't but uses plain values
    // Connected to docs book 4.2
    write_string_and_length_referencing(String::from("Hello references"));


    let mut s1 = String::from("Hello references");
    change_string(&mut s1);

    println!("{s1}");
}

fn _write_string_and_length_owning(s: String) {
    let (s1, len) = _calculate_length(s);

    println!("{s1}: length is {len}");
}

fn _calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

// DOES NOT COMPILE
// fn write_string_and_length_owning(s: String) {
//     let len = calculate_length(s); // s is moved to function, can't be accessed anymore

//     println!("{s}: length is {len}");
// }

// fn calculate_length(s: String) -> usize {
//     let len = s.len();

//     len
// }

// Working with references
fn write_string_and_length_referencing(s: String) {
    let len = calculate_length_referencing(&s);

    println!("{s}: length is {len}");
}

fn calculate_length_referencing(s: &String) -> usize {
    let len = s.len();

    len
}

// Changing the parameter by referencing (mutable references)
fn change_string(s: &mut String) {
    s.insert(5, '_');
}