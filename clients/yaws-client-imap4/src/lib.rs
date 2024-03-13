//! yaws IMAP Reference Client implementation

//#![no_std] ip_in_core
#![warn(
    clippy::unwrap_used,
    //missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

use yaws_proto_imap4::IMAP4rev2Context;
use yaws_proto_imap4::Response;

mod tcp;

//use core::net::SocketAddr;
use std::net::SocketAddr;

pub struct IMAP4rev2Client<'a> {
    pub(crate) client: tcp::tokio_uring::Client,
    pub(crate) codec: IMAP4rev2Context,
    pub(crate) current_response: Option<Response<'a>>,
}

#[derive(Debug)]
pub enum IMAP4rev2ClientError {
    Connect(String),
    Read(String),
    ReadNext(String),
}

impl<'a> IMAP4rev2Client<'a> {
    pub async fn connect(dest_addr: SocketAddr) -> Result<Self, IMAP4rev2ClientError> {
        let client = tcp::tokio_uring::Client::connect(dest_addr)
            .await
            .map_err(|e| IMAP4rev2ClientError::Connect(e.to_string()))?;
        let codec = IMAP4rev2Context::new();

        Ok( Self { client, codec, current_response: None })
    }
    pub async fn read_next(&'a mut self) -> Result<&Response<'a>, IMAP4rev2ClientError> {
        self
            .client
            .read_next()
            .await
            .map_err(|e| IMAP4rev2ClientError::Read(e.to_string()))?;
        let buf_size_usize = self.client.buf_size as usize;
        let buf_in = &self.client.buf_in;
        let buf_decode = &buf_in[..buf_size_usize];        

        let response = self.codec.try_next_response(&buf_decode).map_err(|e| IMAP4rev2ClientError::ReadNext(e.to_string()))?;

        self.current_response = Some(response);
        
        Ok(self.current_response.as_ref().expect("Bug"))
    }
}
