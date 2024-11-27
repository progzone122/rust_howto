// ANCHOR: example
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _greeting_file = File::open("temp/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("temp/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO P1
