/// Divides two numbers.
///
/// # Examples
///
/// ```
/// let result = divide(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// ```
/// // This test is ignored because it demonstrates a panic scenario.
/// # #[doc = include_str!("ignore")]
/// # fn main() {
/// let result = divide(10, 0); // This will panic
/// # }
/// ```
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}