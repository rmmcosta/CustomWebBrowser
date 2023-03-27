use tcp_stream::{HandshakeError, TcpStream, TLSConfig};

use std::io::{self, Read, Write};

fn main() {
    let mut stream = TcpStream::connect("example.com:443").unwrap();
    stream.set_nonblocking(true).unwrap();

    while !stream.is_connected() {
        if stream.try_connect().unwrap() {
            break;
        }
    }

    let mut stream = stream.into_tls("example.com", TLSConfig::default());

    while let Err(HandshakeError::WouldBlock(mid_handshake)) = stream {
        stream = mid_handshake.handshake();
    }

    let mut stream = stream.unwrap();

    while let Err(err) = stream.write_all(b"GET / HTTP/1.0\r\nHost: example.com\r\n\r\n") {
        if err.kind() != io::ErrorKind::WouldBlock {
            panic!("error: {:?}", err);
        }
    }
    stream.flush().unwrap();
    let mut res = vec![];
    while let Err(err) = stream.read_to_end(&mut res) {
        if err.kind() != io::ErrorKind::WouldBlock {
            panic!("stream error: {:?}", err);
        }
    }
    println!("{}", String::from_utf8_lossy(&res));
}