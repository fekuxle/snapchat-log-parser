//! A simple crate to parse Snapchat chat logs
//! 
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! snapchat_log_parser = "0.2"
//! ```

pub mod types;
mod timestamp;

#[cfg(test)]
mod tests;
