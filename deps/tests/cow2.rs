fn main() {
    let mut my_string = String::new();
    let example = std::borrow::Cow::from("example");
    my_string.push_str(example.as_ref());
    println!("{}", my_string);
}

#[test]
fn test() {
    main();
}
