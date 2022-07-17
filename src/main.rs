pub(crate) fn main() {
    let mut mutable_array = [2,4,6,8,10,12,16];
    let reference_on_array = &mut mutable_array[0..4];

    {
        reference_on_array[0] = 2012;
    }

    // println!("{:?}", reference_on_array); // borrow property of mutable_array
    // println!("{:?}", mutable_array); // mutable_array retrieves its property

    mutable_array[0]=2013;

}