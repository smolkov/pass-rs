//! # pass-rs 
//!
//! `pass_rs` password storage

pub mod command;
pub mod cli;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = pass_rs::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}