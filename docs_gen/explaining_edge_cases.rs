/// Returns the nth Fibonacci number.
///
/// # Examples
///
/// ```
/// let result = fibonacci(10);
/// assert_eq!(result, 55);
/// ```
///
/// # Panics
///
/// This function will panic if the input is greater than 92, as the result
/// will overflow a `u64`.
fn fibonacci(n: u32) -> u64 {
    if n > 92 {
        panic!("Input too large");
    }
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}