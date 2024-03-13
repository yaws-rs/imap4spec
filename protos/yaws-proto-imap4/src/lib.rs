#![no_std]
#![warn(
    clippy::unwrap_used,
    //missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]
#![allow(dead_code)] // HEAVY TODO

mod capabilities;
mod context;
mod request_response;
mod state;
mod traits;

// TODO Pipelining Deny/Allow-List s.5.5
// https://datatracker.ietf.org/doc/html/rfc9051#section-5.5

pub use context::IMAP4rev2Context;
pub use request_response::{Request, Response, ResponseStatus};
pub use state::{IMAP4rev2State, IMAP4rev2StateIllegalSwitch};
