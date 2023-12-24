# Tests

`cargo test` to run all tests.
`cargo test test_prefix` to run all tests that start with the provided prefix.
`cargo test -- --show-output` to show output (println!) that is otherwise captured during tests.

```rust,editable,ignore
// in the same file than the main code

#[cfg(test)]  // only for unit tests
mod tests {
    use super::*; // access to all objects in the parent module, which contains the main code

    // Test functions must be free, monomorphic functions that take no arguments,
    // and commonly return () or Result<T, E> where T: Termination, E: Debug
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        // or assert_eq!(result, some_const);
        // or assert_ne!
    }

    #[test]
    #[should_panic]  // The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.
    fn another() {
        panic!("Make this test fail");
    }

    // with Result
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // pass if OK
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

}
```

Custom message:

```rust,editable,ignore
fn main() {
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

## See Also

[Approx]( https://docs.rs/approx/latest/approx/index.html )

[cargo-nextest]( https://nexte.st/ ): `cargo nextest run; cargo test --doc`