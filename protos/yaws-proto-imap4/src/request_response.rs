pub mod authenticated;
pub mod not_authenticated;
pub mod universal;

use core::result::Result;

use crate::context::IMAP4rev2Context;
use crate::state::IMAP4rev2State;

use logos::Logos;

#[derive(Debug)]
pub enum ResponseStatus {
    Bad,
    Ok,
}

#[derive(Debug)]
pub struct Response<'a> {
    pub id: Option<&'a str>,
    pub status: Option<ResponseStatus>,
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
    InvalidResponseSecond(&'a str),
}

impl core::fmt::Display for ScanResponseError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidUtf8 => write!(f, "Invalid UTF-8 in Response"),
            Self::InvalidResponseFirst(s) => write!(f, "Invalid Response first Part: {}", s),
            Self::InvalidResponseSecond(s) => write!(f, "Invalid Response second Part: {}", s),
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
enum ResponseToken {
    #[regex("[A-Za-z0-9]+")]
    AlphaNums,
    #[token("* BAD")]
    ResponseBad,
    #[token("* OK")]
    ResponseOk,
    #[token("* NO")]
    ResponseNo,
}

#[derive(Logos, Debug, PartialEq)]
enum ResponseTextToken {
    // TODO: cfg_attr strict <CR><LF> ?
    #[regex(r"\r*\n")]
    CRLF,
    #[regex(r"\s[^\r]+")]
    DATA,
}

impl<'a> Response<'a> {
    pub fn scan_with_context(
        ctx: &mut IMAP4rev2Context,
        s: &'a [u8],
    ) -> Result<Self, ScanResponseError<'a>> {
        // TODO: re-impl trait logos codegengen || UTF-8 in-buf at I/O layer ?
        // TODO: check other lexers / parsers out + bench
        let checked_str: &'a str =
            core::str::from_utf8(s).map_err(|_| ScanResponseError::InvalidUtf8)?;

        let mut lex = ResponseToken::lexer(checked_str);

        let mut response = Self {
            id: None,
            status: None,
            raw: Some(checked_str),
        };

        response.status = match lex.next() {
            //Some(first) => println!(first),
            // Star completes Response
            //Some(Ok(ResponseToken::Star)) => {

            //response.status = match lex.next() {
            Some(Ok(ResponseToken::ResponseOk)) => Some(ResponseStatus::Ok),
            Some(Ok(ResponseToken::ResponseBad)) => Some(ResponseStatus::Bad),
            _ => return Err(ScanResponseError::InvalidResponseSecond(lex.slice())),
            //};

            //},
            //_ => return Err(ScanResponseError::InvalidResponseFirst(checked_str)),
        };

        let aa = match ctx.rfc_state {
            IMAP4rev2State::NotAuthenticated => {}
            IMAP4rev2State::Authenticated => {}
            IMAP4rev2State::Selected => {}
            IMAP4rev2State::Logout => {}
            IMAP4rev2State::Idle => {}
        };

        let mut response = Self {
            id: None,
            status: None,            
            raw: Some(checked_str),
        };
        
        Ok(response)
    }
}

#[derive(Debug)]
pub enum ScanRequestError<'a> {
    InvalidUtf8,
    Invalid(&'a str),
}

impl<'a> Request<'a> {
    pub fn scan_with_context(ctx: &mut IMAP4rev2Context, s: &'a [u8]) -> Result<Self, ScanRequestError<'a>> {

        let checked_str: &'a str =
            core::str::from_utf8(s).map_err(|_| ScanRequestError::InvalidUtf8)?;
        
        match ctx.rfc_state {
            IMAP4rev2State::NotAuthenticated => {}
            IMAP4rev2State::Authenticated => {}
            IMAP4rev2State::Selected => {}
            IMAP4rev2State::Logout => {}
            IMAP4rev2State::Idle => {}
        }

        let mut request = Self {
            id: None,
            raw: Some(checked_str),
        };
        
        Ok(request)
    }
}
