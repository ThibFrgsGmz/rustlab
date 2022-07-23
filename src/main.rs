use std::vec;

pub(crate) fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let mut mutable_infered_vector = vec![12,45];
    mutable_infered_vector.push(85);
    println!("{:?}", mutable_infered_vector);

    // mutable_infered_vector.remove(85); /* crash: removal index (is 85) should be < len (is 3)' */
    // println!("{:?}", mutable_infered_vector.get(85)); // Display "None"
    
    println!("{:?}", mutable_infered_vector.get(1)); // Display "None"

    match mutable_infered_vector.get(85) {
        Some(x) => println!("{x}"),
        None => println!("No value available"),
    };

    let immutable_vector = vec![12,13,14,15,16];
    let sliced_vector = &immutable_vector[0..2];

    println!("{:?}", sliced_vector);

    let mut uninitialzed_vector = Vec::new();
    uninitialzed_vector.push(10u8); // Implies a u8-vector
    uninitialzed_vector.push(10u8); // Error:  change the type of the numeric literal from `u32` to `u8`

    println!("{:?}", uninitialzed_vector);


}