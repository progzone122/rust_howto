fn main() {
    // ANCHOR: example
    let _some_number = Some(5);

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);
    // ANCHOR_END: example
}

#[test]
fn test() {
    main();
}