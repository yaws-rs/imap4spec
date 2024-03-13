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

mod tcp;

//use core::net::SocketAddr;
use std::net::SocketAddr;

pub struct IMAP4rev2Client {
    pub(crate) client: tcp::tokio_uring::Client,
    pub(crate) codec: IMAP4rev2Context,
}

#[derive(Debug)]
pub enum IMAP4rev2ClientError {
    Connect(String),
    Read(String),
}

impl IMAP4rev2Client {
    pub async fn connect(dest_addr: SocketAddr) -> Result<Self, IMAP4rev2ClientError> {
        let client = tcp::tokio_uring::Client::connect(dest_addr)
            .await
            .map_err(|e| IMAP4rev2ClientError::Connect(e.to_string()))?;
        let codec = IMAP4rev2Context::new();

        Ok(IMAP4rev2Client { client, codec })
    }
    pub async fn read_next(mut self) -> Result<(), IMAP4rev2ClientError> {
        self.client = self
            .client
            .read_next()
            .await
            .map_err(|e| IMAP4rev2ClientError::Read(e.to_string()))?;
        let buf_decode = &self.client.buf_in[..self.client.buf_size as usize];
        self.codec.try_next_response(&buf_decode);
        Ok(())
    }
}
