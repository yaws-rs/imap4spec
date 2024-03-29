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
mod tls;

//use core::net::SocketAddr;
use std::net::SocketAddr;

pub struct IMAP4rev2Client<'a> {
    pub(crate) io_client: tcp::tokio_uring::Client,
    pub(crate) codec: IMAP4rev2Context,
    pub(crate) tls: Option<TlsClient>,
    pub(crate) current_response: Option<Response<'a>>,
}

#[derive(Debug)]
pub enum IMAP4rev2ClientError {
    Codec(String),
    Connect(String),
    Read(String),
    Write(String),
    ReadNext(String),
    ParsingHostName(String),
    TlsConnectionError(String),
    NotTlsConnection,
    TlsWriteLock,
    Login(String),
//    TlsUnbuffered(String),
}

use std::sync::RwLock;

use tls::rustls_io_uring::{TlsSpinner, TlsSpinnerError};
use tls::rustls_io_uring::WantFromTlsSpinner;

pub(crate) struct TlsClient {
    pub(crate) rustls_client_config: Arc<rustls::client::ClientConfig>,
    pub(crate) rustls_unbuffered_connection:
    Arc<RwLock<rustls::client::UnbufferedClientConnection>>,
    
}

// TODO: rustls Arc / Alloc in alloc no_std - gate it
use core::ops::Deref;
use std::sync::Arc;

impl<'a> IMAP4rev2Client<'a> {
    pub async fn connect(dest_addr: SocketAddr) -> Result<Self, IMAP4rev2ClientError> {
        let io_client = tcp::tokio_uring::Client::connect(dest_addr)
            .await
            .map_err(|e| IMAP4rev2ClientError::Connect(e.to_string()))?;
        let codec = IMAP4rev2Context::new();

        Ok(Self {
            io_client,
            codec,
            current_response: None,
            tls: None,
        })
    }
    pub async fn connect_tls(
        dest_addr: SocketAddr,
        server_name: Option<String>,
    ) -> Result<Self, IMAP4rev2ClientError> {
        let mut myself = Self::connect(dest_addr).await.map_err(|e| e)?;
        // let root_store = rustls::RootCertStore::empty();
        // TODO: add root cert
        //  root_store.add(rustls_cert_der).unwrap();
        // TODO: add provider config
        //let config_versions = rustls::ClientConfig::builder_with_provider(Arc::new(rustcrypto_provider()))
        //    .with_safe_default_protocol_versions();
        // TODO: root certs
        //    .with_root_certificates(root_store);

        let builder_versions = rustls::ClientConfig::builder_with_protocol_versions(&[
            &rustls::version::TLS13,
            &rustls::version::TLS12,
        ]);

        let fake_server_cert_verifier = tls::rustls_nocert_verifier::FakeServerCertVerifier;

        let dangerous_verifier = builder_versions
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(fake_server_cert_verifier));

        let no_client_auth = dangerous_verifier.with_no_client_auth();

        let rustls_server_name = match server_name {
            None => rustls::pki_types::ServerName::IpAddress(dest_addr.ip().into()),
            Some(addr) => rustls::pki_types::ServerName::try_from(addr)
                .map_err(|_| IMAP4rev2ClientError::ParsingHostName("Invalid hostname".into()))?,
        };

        let arc_client_config = Arc::new(no_client_auth);

        let rustls_unbuffered_connection = rustls::client::UnbufferedClientConnection::new(
            Arc::clone(&arc_client_config),
            rustls_server_name,
        )
        .map_err(|e| IMAP4rev2ClientError::TlsConnectionError(e.to_string()))?;

        myself.tls = Some(TlsClient {
            rustls_client_config: arc_client_config,
            rustls_unbuffered_connection: Arc::new(RwLock::new(rustls_unbuffered_connection)),
        });

        Ok(myself)
    }
    pub async fn login_tls(&'a mut self, user: &str, pass: &str) -> Result<&Response<'a>, IMAP4rev2ClientError> {
        let tls = match &mut self.tls {
            Some(tls_conn) => tls_conn,
            None => return Err(IMAP4rev2ClientError::NotTlsConnection),
        };

        //let plaintext_out = self.codec.try_login(user, pass).map_err(|e| IMAP4rev2ClientError::Codec(e.to_string()))?;

        let plaintext_out = format!("A0001 LOGIN {} {}\r\n", user, pass);
        
        let mut spinner = TlsSpinner::new();
        spinner.add_to_encrypt(plaintext_out.as_bytes());

        match spinner.spin(WantFromTlsSpinner::Write, &mut self.io_client, Arc::clone(&tls.rustls_unbuffered_connection)).await {
            Err(TlsSpinnerError::WrittenCompleted(c)) => {
                println!("Wrote {} bytes supposedly ?", c);
            },
            _ => todo!(),
        }

        loop {

            println!("Reading ..");
            let read_in = self.io_client.read_next().await.map_err(|e| IMAP4rev2ClientError::Login(e.to_string()))?;

            println!("Cleartext in {} bytes", read_in);
            
            if read_in > 0 {
                match spinner.spin(WantFromTlsSpinner::Read, &mut self.io_client, Arc::clone(&tls.rustls_unbuffered_connection)).await {
                    Err(e) => println!("WTF err {:?}", e),
                    _ => todo!(),
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        
        todo!();
    }
    pub async fn read_next_tls(
        &'a mut self,
    ) -> Result<Option<&Response<'a>>, IMAP4rev2ClientError> {

        let tls = match &mut self.tls {
            Some(tls_conn) => tls_conn,
            None => return Err(IMAP4rev2ClientError::NotTlsConnection),
        };

        let mut spinner = TlsSpinner::new();
        
        match spinner.spin(WantFromTlsSpinner::Read, &mut self.io_client, Arc::clone(&tls.rustls_unbuffered_connection)).await {
            Err(TlsSpinnerError::NothingToRead) => Ok(None),
            _ => todo!(),
        }
    }
    pub async fn read_next(&'a mut self) -> Result<&Response<'a>, IMAP4rev2ClientError> {
        self.io_client
            .read_next()
            .await
            .map_err(|e| IMAP4rev2ClientError::Read(e.to_string()))?;
        let buf_size_usize = self.io_client.buf_size_in as usize;
        let buf_in = &self.io_client.buf_in;
        let buf_decode = &buf_in[..buf_size_usize];

        let response = self
            .codec
            .try_next_response(&buf_decode)
            .map_err(|e| IMAP4rev2ClientError::ReadNext(e.to_string()))?;

        self.current_response = Some(response);

        Ok(self.current_response.as_ref().expect("Bug"))
    }
}
