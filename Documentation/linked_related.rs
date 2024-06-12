/// Adds two numbers together.
///
/// See also [`subtract`].
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first.
///
/// See also [`add`].
///
/// # Examples
///
/// ```
/// let result = subtract(5, 3);
/// assert_eq!(result, 2);
/// ```
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}