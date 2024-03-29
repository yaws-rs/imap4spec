use tokio_uring::net::TcpStream;
//use tokio_uring::buf::IoBufMut;
use tokio_uring::buf::IoBuf as UringIoBuf;

use std::net::SocketAddr;

#[derive(Debug)]
pub enum ReadError {}

#[derive(Debug)]
pub enum ClientError {
    Connect(String),
    BugTooBigRead,
    Read(String),
    Write(String),
    Closed,
}

impl core::fmt::Display for ClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Connect(s) => write!(f, "Error - Connection: {}", s),
            Self::BugTooBigRead => write!(f, "Bug - Read Too Big Buffer ?!"),
            Self::Read(s) => write!(f, "Error - Socket Read: {}", s),
            Self::Write(s) => write!(f, "Error - Socket Write: {}", s),
            Self::Closed => write!(f, "Error - Socket Closed"),
        }
    }
}

use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct Client {
    //pub(crate) buf_in: Vec<u8>,
    pub(crate) buf_in: BytesMut,    
    //pub(crate) buf_out: Vec<u8>,
    pub(crate) buf_out: BytesMut,
    pub(crate) buf_size_in: usize,
    pub(crate) buf_size_out: usize,
    pub(crate) client: TcpStream,
}

impl Client {
    pub async fn connect(dest_addr: SocketAddr) -> Result<Self, ClientError> {
        let client = TcpStream::connect(dest_addr)
            .await
            .map_err(|e| ClientError::Connect(e.to_string()))?;

        Ok(Self {
            buf_in: BytesMut::new(),
            // TODO: rustls encode() uses len() not capacity()
            buf_out: BytesMut::zeroed(8192),
            client,
            buf_size_in: 0,
            buf_size_out: 0,
        })
    }
    pub fn set_expected_out(&mut self, out_len: usize) {
        self.buf_size_out = out_len;
    }
    pub async fn write_all(&mut self) -> Result<(), ClientError> {

        // TODO: Windowing & buf handling proper
        let buf_out = self.buf_out.split_to(self.buf_size_out).to_vec();
        
        let (res, _) = self.client.write_all(buf_out).await;

        match res {
            Err(e) => Err(ClientError::Write(e.to_string())),
            Ok(_) => {
                self.buf_size_out = 0;
                self.buf_out = BytesMut::zeroed(8192);
                Ok(())
            },
        }
    }
    pub async fn read_next(&mut self) -> Result<usize, ClientError> {

        let mut total_read = 0;
        
        loop {
            let mut buf_in = vec![0; 8192];
            let (res, buf) = self.client.read(buf_in).await;

            let n = res.map_err(|e| ClientError::Read(e.to_string()))?;
                
            // How ?
            if n >= 8192 {
                return Err(ClientError::BugTooBigRead);
            }

            if n == 0 {
                return Err(ClientError::Closed);
            }

            total_read += n;
            
            self.buf_size_in += n;
            (*self).buf_in.extend_from_slice(&buf[..n]);

            if n != 8192 {
                break;
            }
            

        }
            
        Ok(total_read)

    }
}
