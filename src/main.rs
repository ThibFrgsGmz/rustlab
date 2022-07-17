pub(crate) fn main() {

    let immutable_array: [u8;4]=[1,2,3,4];
    let mut mutable_array: [u8;4]=[1,2,3,4];
    let mut mutable_matrix:[[i8;2];2] =[[1,2],[3,4]];
    let mut mutable_matrix_infered_type=[[1,2],[3,4]];

    // immutable_array[0] = 5; #error `immutable_array` is not declared as mutable
    mutable_array[0] = 6;

    println!("Here is the immutable array: {immutable_array:?}");
    println!("Here is the mutable array: {mutable_array:?}");
    println!("Here is the mutable array: {}", mutable_array.len());
    println!("Here is the mutable matrix: {mutable_matrix:?}");
    println!("Here is the mutable matrix: {mutable_matrix_infered_type:?}");
    println!("Element of mutable_matrix_infered_type: {:?}", mutable_matrix_infered_type[1][1]);
}