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
    pub(crate) client: tcp::tokio_uring::Client,
    pub(crate) codec: IMAP4rev2Context,
    pub(crate) tls: Option<TLSClient>,
    pub(crate) current_response: Option<Response<'a>>,
}

#[derive(Debug)]
pub enum IMAP4rev2ClientError {
    Connect(String),
    Read(String),
    Write(String),
    ReadNext(String),
    ParsingHostName(String),
    TlsConnectionError(String),
    NotTlsConnection,
    TlsWriteLock,
    TlsUnbuffered(String),
}

use std::sync::RwLock;

use tls::rustls_codec::TlsContext as ImapTlsContext;

// TODO: move to tls/rustls_codec
use rustls::unbuffered::ConnectionState as RustlsConnectionState;

pub(crate) struct TLSClient {
    pub(crate) rustls_client_config: Arc<rustls::client::ClientConfig>,
    pub(crate) rustls_unbuffered_connection:
        Arc<RwLock<rustls::client::UnbufferedClientConnection>>,
    pub(crate) rustls_codec: ImapTlsContext,
}

// TODO: rustls Arc / Alloc in alloc no_std - gate it
use core::ops::Deref;
use std::sync::Arc;

impl<'a> IMAP4rev2Client<'a> {
    pub async fn connect(dest_addr: SocketAddr) -> Result<Self, IMAP4rev2ClientError> {
        let client = tcp::tokio_uring::Client::connect(dest_addr)
            .await
            .map_err(|e| IMAP4rev2ClientError::Connect(e.to_string()))?;
        let codec = IMAP4rev2Context::new();

        Ok(Self {
            client,
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
        let root_store = rustls::RootCertStore::empty();
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

        let rustls_codec = ImapTlsContext::new();

        myself.tls = Some(TLSClient {
            rustls_client_config: arc_client_config,
            rustls_unbuffered_connection: Arc::new(RwLock::new(rustls_unbuffered_connection)),
            rustls_codec,
        });

        Ok(myself)
    }
    pub async fn read_next_tls(
        &'a mut self,
    ) -> Result<Option<&Response<'a>>, IMAP4rev2ClientError> {
        let tls_conn_arc = match &mut self.tls {
            Some(tls_conn) => &tls_conn.rustls_unbuffered_connection,
            None => return Err(IMAP4rev2ClientError::NotTlsConnection),
        };

        println!(".. pre-write-lock status ?");

        let mut tls_conn = tls_conn_arc
            .deref()
            .write()
            .map_err(|e| IMAP4rev2ClientError::TlsWriteLock)?;

        println!(".. buff status ?");

        //let mut buf_in = &mut self.client.buf_in;
        //let mut buf_out = &mut self.client.buf_out;

        loop {
            let mut io_client = &mut self.client;

            let rustls::unbuffered::UnbufferedStatus { mut discard, state } =
                tls_conn.process_tls_records(&mut io_client.buf_in);

            let state = match state {
                Ok(s) => s,
                Err(e) => return Err(IMAP4rev2ClientError::TlsUnbuffered(e.to_string())),
            };

            dbg!(&state);

            println!(".. pre-read status ?");

            //let tls_codec = self.tls.expect("BUG-Unchecked: No TLS?").rustls_codec;

            // TODO: Move this to the codec.

            //dbg!(&buf_out.len());

            match state {
                RustlsConnectionState::ReadTraffic(mut s) => {
                    todo!();
                }
                RustlsConnectionState::Closed => {
                    todo!();
                }
                RustlsConnectionState::ReadEarlyData(mut s) => {
                    todo!();
                }
                RustlsConnectionState::EncodeTlsData(mut s) => {
                    match s.encode(&mut io_client.buf_out) {
                        Ok(out_len) => {
                            io_client.write_all().await.unwrap();
                        }
                        Err(e) => panic!("EncodeTLsData PANIC {:}", e),
                    }
                    //dbg!(encode_out);
                }
                RustlsConnectionState::TransmitTlsData(mut s) => {
                    if let Some(mut may_encrypt) = s.may_encrypt_app_data() {}
                    io_client.write_all().await.unwrap();
                    s.done();
                }
                RustlsConnectionState::BlockedHandshake => {
                    todo!();
                }
                RustlsConnectionState::WriteTraffic(mut s) => {
                    todo!();
                }
                // Rustls has non-exhaustive states :E
                _ => todo!(),
            }
        }

        self.client
            .read_next()
            .await
            .map_err(|e| IMAP4rev2ClientError::Read(e.to_string()))?;
        let buf_size_usize = self.client.buf_size_in as usize;

        /*

        let buf_decode = &buf_in[..buf_size_usize];

        let response = self.codec.try_next_response(&buf_decode).map_err(|e| IMAP4rev2ClientError::ReadNext(e.to_string()))?;

        self.current_response = Some(response);

        Ok(self.current_response.as_ref().expect("Bug"))
         */

        todo!()
    }
    pub async fn read_next(&'a mut self) -> Result<&Response<'a>, IMAP4rev2ClientError> {
        self.client
            .read_next()
            .await
            .map_err(|e| IMAP4rev2ClientError::Read(e.to_string()))?;
        let buf_size_usize = self.client.buf_size_in as usize;
        let buf_in = &self.client.buf_in;
        let buf_decode = &buf_in[..buf_size_usize];

        let response = self
            .codec
            .try_next_response(&buf_decode)
            .map_err(|e| IMAP4rev2ClientError::ReadNext(e.to_string()))?;

        self.current_response = Some(response);

        Ok(self.current_response.as_ref().expect("Bug"))
    }
}
