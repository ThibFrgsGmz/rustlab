pub(crate) fn main() {
    let mut mutable_infered_tuple = (1u8, 23.8, 26, "Stringy");
    let mut mutable_tuple:(u8,f64,usize,&str) = (1u8, 23.8, 26, "Stringy"); 
    let mut mutable_tuple_matrix = (1u8, (23.8, 26, "Stringy")); 

    println!("{:?}", mutable_infered_tuple);
    println!("{:?}", mutable_tuple);
    println!("{:?}", mutable_tuple.1);
    println!("{:?}", (mutable_tuple_matrix.1).1);
}