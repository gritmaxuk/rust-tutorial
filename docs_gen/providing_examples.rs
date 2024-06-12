/// Calculates the factorial of a number.
///
/// # Examples
///
/// ```
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
fn factorial(n: u32) -> u32 {
    (1..=n).product()
}