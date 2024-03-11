#![no_std]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

mod capabilities;
mod context;
mod request_response;
mod state;
mod traits;

pub use context::IMAP4rev2Context;
pub use request_response::{Request, Response};
pub use state::IMAP4rev2State;
