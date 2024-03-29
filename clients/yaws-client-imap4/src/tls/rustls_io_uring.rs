use rustls::client::UnbufferedClientConnection as RustlsConnection;
use rustls::unbuffered::ConnectionState as RustlsConnectionState;

use crate::tcp::tokio_uring::Client as IoClient;
use crate::TlsClient;

use std::sync::Arc;
use std::sync::RwLock;
use core::ops::Deref;

/// Used to indicate what state is wanted from Spinner
pub(crate) enum WantFromTlsSpinner {
    Read,
    Write,
}

use bytes::{Buf, BufMut, Bytes, BytesMut};

/// Salad spinner but for yo' TLS :]
pub(crate) struct TlsSpinner {
    pub(crate) decrypted_in: BytesMut,
    pub(crate) encrypt_out: BytesMut,
}

#[derive(Debug)]
pub(crate) enum TlsSpinnerError {
    TlsUnbuffered(String),
    TlsWriteLock,
    // Wanted to Read decrypted but no AppData available
    NothingToRead,
    // Wanted to Write encrypted
    NothingToWrite,
}

// TODO: Trait'Spin'able :)
// TODO: impl Spinner for TlsSalad FtW
impl TlsSpinner {


    pub(crate) fn new() -> Self {
        TlsSpinner { decrypted_in: BytesMut::new(), encrypt_out: BytesMut::new() }
    }
    // TODO: max size Bytes out ?
    pub(crate) fn encrypt_out(&mut self, extend: &[u8]) -> usize {
        // YOLO
        self.encrypt_out.extend_from_slice(extend);

        self.encrypt_out.len()
    }
    // TODO: generalise - this is a messy thing rn - also allow blocking/wasm ..
    pub(crate) async fn go(&mut self, want: WantFromTlsSpinner, io_client: &mut IoClient, arc_conn: Arc<RwLock<RustlsConnection>>) -> Result<(), TlsSpinnerError> {

        let mut tls_conn = arc_conn
            .deref()
            .write()
	        .map_err(|e| TlsSpinnerError::TlsWriteLock)?;

        let mut state_open = true;
        let mut state_error: Option<TlsSpinnerError> = None;
        
        loop {

	        let rustls::unbuffered::UnbufferedStatus { mut discard, state } =
                tls_conn.process_tls_records(&mut io_client.buf_in);
            
            let state = match state {
                Ok(s) => s,
                Err(e) => return Err(TlsSpinnerError::TlsUnbuffered(e.to_string())),
            };

            dbg!(&state);

            println!(".. pre-read status ?");

            match state {
	            RustlsConnectionState::ReadTraffic(mut s) => {
                    todo!();
                },
		        RustlsConnectionState::Closed => {
                    todo!();
                },
                RustlsConnectionState::ReadEarlyData(mut s) => {
                    todo!();
	            },
                RustlsConnectionState::EncodeTlsData(mut s) => {
	                match s.encode(&mut io_client.buf_out) {
			            Ok(len_out) => {
                            println!("sending out {} bytes", len_out);
                            io_client.set_expected_out(len_out);
                        }
	                    Err(e) => panic!("EncodeTLsData PANIC {:}", e),
                    }
                },
                RustlsConnectionState::TransmitTlsData(mut s) => {
                    if let Some(mut may_encrypt) = s.may_encrypt_app_data() {
			            println!("... may encrypt ?");
                    }
                    println!("Data done.");
	                io_client.write_all().await.unwrap();
                    s.done();
	            },
		        RustlsConnectionState::BlockedHandshake => {
                    io_client.read_next().await.unwrap();
                },
                RustlsConnectionState::WriteTraffic(mut s) => {
                    match want {
                        WantFromTlsSpinner::Read => {
                            state_error = Some(TlsSpinnerError::NothingToRead)
                        },
                        WantFromTlsSpinner::Write => {
                            todo!();
                        },
                    }
                },
                // Rustls has non-exhaustive states :E
		        _ => todo!(),
            }
            if discard != 0 {
                // TODO: err
	            assert!(discard <= io_client.buf_size_in);
                // TODO: zeroize?
	            let buf_in_new = io_client.buf_in.split_off(discard);
		        println!("buf_in.len() = {}",io_client.buf_in.len());
                println!("buf_in_new.len() = {}",buf_in_new.len());
                io_client.buf_in = buf_in_new;
	        }
            dbg!(discard);

            match state_error {
                Some(e) => return Err(e),
                None => {},
            }
        }
    }
}
