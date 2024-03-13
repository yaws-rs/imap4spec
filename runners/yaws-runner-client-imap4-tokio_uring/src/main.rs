use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    tokio_uring::start(async {
        let sockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 143);

        let client = yaws_client_imap4::IMAP4rev2Client::connect(sockaddr)
            .await
            .unwrap();

        let _incoming = client.read_next().await;
    });
}
