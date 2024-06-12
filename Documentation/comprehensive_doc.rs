//! This module provides basic arithmetic operations.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = arithmetic::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first.
///
/// # Examples
///
/// ```
/// let result = arithmetic::subtract(5, 3);
/// assert_eq!(result, 2);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers together.
///
/// # Examples
///
/// ```
/// let result = arithmetic::multiply(4, 3);
/// assert_eq!(result, 12);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers.
///
/// # Examples
///
/// ```
/// let result = arithmetic::divide(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function will panic if the divisor is zero.
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}