pub(crate) fn main() {
    let mut a = String::from("Test");

    {
        let b = &mut a;
        b.push_str(" World!");
    }
    let _c = &a;
    println!("{a}");
}