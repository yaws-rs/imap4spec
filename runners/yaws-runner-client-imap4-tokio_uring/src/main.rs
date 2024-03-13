use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    tokio_uring::start(async {
        //let sockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 143);
        let sockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 993);        

        let mut client = yaws_client_imap4::IMAP4rev2Client::tls_connect(sockaddr)
            .await
            .unwrap();

        let incoming = client.read_next().await;

        dbg!(&incoming);
    });
}
