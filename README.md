# testlog

**Yes, One macro**

A micro-crate that provides `test_log!` – a macro that prints to stderr only when tests are running, and only for the crate where it's used.

## Why?

You need to debug test failures. It's tempting to add print statements to the
library you're testing.  But, adding print statements of any kind pollute the
logs of the _consumers_ of your library. What you want is something _in your
library_ that only prints for _your tests_, and ideally is only shown _on failure_.

This does exactly what you want: **debug output that only appears during
testing of _your_ crate**, and since output from a test is captured on success,
you only see the log when the test fails. Perfect!.

## Usage

Add to `Cargo.toml`:
```toml
[dependencies]
testlog = "0.1.3"
```

Use in your code:
```rust
use testlog::test_log;

fn complex_function(data: &[i32]) -> Vec<i32> {
    test_log!("Processing {} items", data.len());
    
    let result: Vec<i32> = data.iter()
        .map(|&x| {
            test_log!("Processing item: {}", x);
            x * 2
        })
        .collect();
        
    test_log!("Result: {:?}", result);
    result
}

#[test]
fn test_complex_function() {
    let input = vec![1, 2, 3];
    let output = complex_function(&input);
    assert_eq!(output, vec![2, 4, 6]);
    // Debug output appears here when running `cargo test`
}
```

## How it works

The `test_log!` macro checks `cfg!(test)` at compile time:
- **In test builds**: Expands to `eprintln!(...)`, even still, the output is captured on successful tests, so it doesn't pollute. 
- **In production builds**: `if false` should be compiled out in release builds.
- **Crate-local**: Only activates when the *current* crate is in test mode

That's it

