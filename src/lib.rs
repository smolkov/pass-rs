//! # pass-rs
//!
//! `pass` - password storage

pub mod cli;
pub mod command;
pub mod config;
pub mod dirs;
pub mod password;
pub mod key;
pub mod store;
pub mod git;

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
