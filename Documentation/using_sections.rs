/// Divides two numbers.
///
/// # Examples
///
/// ```
/// let result = divide(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function will panic if the divisor is zero.
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}