use testlog::test_log;
fn main() {
    test_log!("This is a test log message that should not be printed.");
    println!("This message should be printed to the console.");
}
