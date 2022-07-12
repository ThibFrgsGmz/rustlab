pub(crate) fn main() {
    let my_variable= 12;
    let a_test: i32 = 48u8 as i32;

    let _my_string: &str = "String_1";
    let _my_string_2: String = String::from("String_2");
    let mut _my_mutable_string: String = String::from("String_3");

    _my_mutable_string.push_str("HelloThere");

    println!("{my_variable}");
    println!(r#"{a_test}"#);
    println!("{_my_mutable_string}");
}