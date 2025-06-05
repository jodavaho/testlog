//! # testlog
//!
//! A tiny, focused crate that provides a single macro: `test_log!`
//!
//! This macro prints to stderr **only when tests are running** and **only for the crate where it's used**.
//! Perfect for debugging test failures without cluttering production output.
//!
//! ## Usage
//!
//! ```rust
//! use testlog::test_log;
//!
//! #[test]
//! fn my_test() {
//!     test_log!("Debug info: {}", some_value);
//!     // Output only appears when running tests
//! }
//! ```

/// A macro that prints to stderr only during test execution for the current crate.
///
/// This macro checks `cfg!(test)` at compile time to determine if the current crate
/// is being compiled for testing. If so, it prints the formatted message to stderr
/// using `eprintln!`. If not, the macro expands to nothing and has zero runtime cost.
///
/// # Key Behavior
///
/// - **Crate-local**: Only prints when the *current* crate (where the macro is used) is in test mode
/// - **Test-only**: No output in production builds or when tests aren't running  
/// - **Zero-cost**: Completely eliminated from non-test builds
/// - **stderr output**: Uses `eprintln!` to avoid interfering with test output capture
///
/// # Examples
///
/// ```rust
/// use testlog::test_log;
///
/// fn some_function(x: i32) -> i32 {
///     test_log!("Processing value: {}", x);
///     x * 2
/// }
///
/// #[test]
/// fn test_function() {
///     let result = some_function(5);
///     test_log!("Result: {}", result);
///     assert_eq!(result, 10);
/// }
/// ```
///
/// The debug output will only appear when running `cargo test`, not when using
/// the function in production code.
#[macro_export]
macro_rules! test_log {
    ($($arg:tt)*) => {
        if cfg!(test) {
            eprintln!($($arg)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_in_test_mode() {
        test_log!("This message should appear when running tests");
        assert_eq!(1, 1);
    }

    #[test]
    fn test_log_with_formatting() {
        let value = 42;
        test_log!("Debug value: {}", value);
        test_log!("Multiple values: {} and {}", value, "test");
        assert!(true);
    }

    #[test]
    #[should_panic(expected = "Intentional panic to show test_log output")]
    fn failing_test_shows_debug_output() {
        test_log!("This debug message appears before the test fails");
        test_log!("Processing important data: {}", 123);
        test_log!("About to panic - this helps debug the failure");
        panic!("Intentional panic to show test_log output");
    }
}
