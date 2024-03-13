//! TODO
use lunatic::{net, spawn_link, Mailbox};

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct TcpClient {
    remote_address: String,
}

impl TcpClient {
    pub(crate) fn connect(remote_address: &str) -> Self {
        TcpClient {
            remote_address: local_address.to_owned(),
        }
    }
    pub(crate) fn spawn(self) {
        spawn_link!(|input = self| listen(input));
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TcpPeer {
    tcp_stream: lunatic::net::TcpStream,
}
