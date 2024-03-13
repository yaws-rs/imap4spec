pub mod authenticated;
pub mod not_authenticated;
pub mod universal;

use core::result::Result;

use crate::context::IMAP4rev2Context;
use crate::state::IMAP4rev2State;

use logos::Logos;

#[derive(Debug)]
pub struct Response<'a> {
    pub id: Option<&'a str>,
    pub raw: Option<&'a str>,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub id: Option<&'a str>,
    pub raw: Option<&'a str>,
}

#[derive(Debug)]
pub enum ScanResponseError<'a> {
    InvalidUtf8,
    InvalidResponseFirst(&'a str),
}

impl core::fmt::Display for ScanResponseError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidUtf8 => write!(f, "Invalid UTF-8 in Response"),
            Self::InvalidResponseFirst(s) => write!(f, "Invalid Response first Part: {}", s),
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
enum ResponseToken {
    #[token(r"\s")]
    Space,
    #[token(r"\+")]
    Plus,
    #[token(r"\*")]
    Star,
    #[token("[A-Za-z0-9]+")]
    Tag,
    #[token("BAD")]
    Bad,
    #[token("OK")]
    Ok,
}

impl<'a> Response<'a> {
    pub fn scan_with_context(
        ctx: &mut IMAP4rev2Context,
        s: &'a [u8],
    ) -> Result<Self, ScanResponseError<'a>> {
        
        // TODO: re-impl trait logos codegengen || UTF-8 in-buf at I/O layer ?
        // TODO: check other lexers / parsers out + bench
        let checked_str: &'a str = core::str::from_utf8(s).map_err(|_| ScanResponseError::InvalidUtf8)?;
        
        let mut lex = ResponseToken::lexer(checked_str); 

        let first = match lex.next() {
            //Some(first) => println!(first),
            _ => return Err(ScanResponseError::InvalidResponseFirst(checked_str)),
        };
        
        let aa = match ctx.rfc_state {
            IMAP4rev2State::NotAuthenticated => {}
            IMAP4rev2State::Authenticated => {}
            IMAP4rev2State::Selected => {}
            IMAP4rev2State::Logout => {}
            IMAP4rev2State::Idle => {}
        };
        
        Ok( Self { id: None, raw: None } )
    }
}

pub enum ScanRequestError {}

impl Request<'_> {
    pub fn scan_with_context(ctx: &mut IMAP4rev2Context, s: &[u8]) -> Result<(), ScanRequestError> {
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
