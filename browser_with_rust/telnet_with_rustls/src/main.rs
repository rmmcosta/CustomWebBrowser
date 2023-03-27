use rustls::{ClientConfig, ClientConnection, ServerName};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let server_name = ServerName::try_from("example.com").expect("invalid DNS name");
    let rc_config = Arc::new(config);
    let mut client = match ClientConnection::new(rc_config, server_name) {
        Ok(it) => it,
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Rustls error: {}", err),
            ))
        }
    };

    client.writer().write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut socket = connect("example.com", 443)?;
    let duration = Some(Duration::from_secs(10));
    loop {
        if client.wants_read() && socket.ready_for_read(duration) {
            client.read_tls(&mut socket).unwrap();
            client.process_new_packets().unwrap();

            let mut plaintext = Vec::new();
            client.reader().read_to_end(&mut plaintext).unwrap();
            io::stdout().write(&plaintext).unwrap();
        }

        if client.wants_write() && socket.ready_for_write(duration) {
            client.write_tls(&mut socket).unwrap();
        }

        socket.wait_for_something_to_happen(duration);
    }
}

fn connect(host: &str, port: u16) -> std::io::Result<TcpStream> {
    let addr = format!("{}:{}", host, port);
    TcpStream::connect(addr)
}
