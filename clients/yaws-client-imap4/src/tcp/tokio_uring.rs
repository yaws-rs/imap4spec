use tokio_uring::net::TcpStream;
//use tokio_uring::buf::IoBufMut;

use std::net::SocketAddr;

#[derive(Debug)]
pub enum ReadError {}

#[derive(Debug)]
pub enum ClientError {
    Connect(String),
    BugTooBigRead,
    Read(String),
}

impl core::fmt::Display for ClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Connect(s) => write!(f, "Error - Connection: {}", s),
            Self::BugTooBigRead => write!(f, "Bug - Read Too Big Buffer ?!"),
            Self::Read(s) => write!(f, "Error - Socket Read: {}", s),
        }
    }
}

pub struct Client {
    pub(crate) buf_in: Vec<u8>,
    pub(crate) buf_size: u16,
    pub(crate) client: TcpStream,
}

impl Client {
    pub async fn connect(dest_addr: SocketAddr) -> Result<Self, ClientError> {
        let client = TcpStream::connect(dest_addr)
            .await
            .map_err(|e| ClientError::Connect(e.to_string()))?;
        let buf_in = vec![0; 8192];

        Ok(Self {
            buf_in,
            client,
            buf_size: 0,
        })
    }
    pub async fn read_next(mut self) -> Result<Self, ClientError> {
        let (res, buf) = self.client.read(self.buf_in).await;

        let n = res.map_err(|e| ClientError::Read(e.to_string()))?;
        
        // How ?
        if n >= 8192 {
            return Err(ClientError::BugTooBigRead);
        } else {
            self.buf_size = n as u16;
        }

        self.buf_in = buf;

        Ok(self)
    }
}
