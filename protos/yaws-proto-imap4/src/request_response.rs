pub mod authenticated;
pub mod not_authenticated;
pub mod universal;

use core::result::Result;

use crate::context::IMAP4rev2Context;
use crate::state::IMAP4rev2State;

pub struct Response<'a> {
    pub id: &'a str,
    pub raw: &'a str,
}

pub struct Request<'a> {
    pub id: &'a str,
    pub raw: &'a str,
}

pub enum ScanResponseError {
}

impl Response<'_> {
    pub fn scan_with_context(ctx: &mut IMAP4rev2Context, s: &str) -> Result<(), ScanResponseError> {
        match ctx.rfc_state {
            IMAP4rev2State::NotAuthenticated => {}
            IMAP4rev2State::Authenticated => {}
            IMAP4rev2State::Selected => {}
            IMAP4rev2State::Logout => {}
            IMAP4rev2State::Idle => {}
        }
        Ok(())
    }
}

pub enum ScanRequestError {
}

impl Request<'_> {
    pub fn scan_with_context(ctx: &mut IMAP4rev2Context, s: &str) -> Result<(), ScanRequestError> {
        match ctx.rfc_state {
            IMAP4rev2State::NotAuthenticated => {}
            IMAP4rev2State::Authenticated => {}
            IMAP4rev2State::Selected => {}
            IMAP4rev2State::Logout => {}
            IMAP4rev2State::Idle => {}
        }
        Ok(())
    }
}
