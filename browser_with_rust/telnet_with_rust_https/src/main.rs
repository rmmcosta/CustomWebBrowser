extern crate openssl;

use std::io::{Read, Write, self};
use std::net::{TcpStream, ToSocketAddrs};
use std::{str, env};

use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use std::fs::File;

const HTTP_VERSION: &str = "1.0";

enum InternalStream {
    Normal(TcpStream),
    Ssl(SslStream<TcpStream>)
}

impl Read for InternalStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match *self {
            InternalStream::Normal(ref mut s) => s.read(buf),
            InternalStream::Ssl(ref mut s) => s.read(buf),
        }
    }
}

impl Write for InternalStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match *self {
            InternalStream::Normal(ref mut s) => s.write(buf),
            InternalStream::Ssl(ref mut s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match *self {
            InternalStream::Normal(ref mut s) => s.flush(),
            InternalStream::Ssl(ref mut s) => s.flush(),
        }
    }
}

fn main() {
    println!("working directory: {}", std::env::current_dir().unwrap().display());
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments. You must pass the url.");
    }
    let url = &args[1];
    // Print the HTML response to the console
    let host = get_host(url).expect("Error getting host");
    let path = get_path(url).expect("Error getting path");
    let port = get_port(url);

    println!("Connecting to {}:{}", host, port);

    // Connect the socket
    let mut stream = match open_stream(&host, port) {
        Ok(s) => s,
        Err(e) => panic!("Failed to open stream: {}", e),
    };

    // Get something
    let request = format!("GET {} HTTP/{}\r\n\r\n", path, HTTP_VERSION);
    stream.write(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut buf = vec![];
    stream.read_to_end(&mut buf).unwrap();
    print!("{}", str::from_utf8(&buf).unwrap());
}

fn get_host(url: &String) -> io::Result<String> {
    let mut url_without_protocol = url.replace("http://", "");
    url_without_protocol = url_without_protocol.replace("https://", "");
    if !url_without_protocol.contains("/") {
        return Ok(url_without_protocol);
    } else {
        let host = url_without_protocol.split("/").next().unwrap_or("");
        return Ok(host.to_string());
    }
}

fn get_port(url: &String) -> u16 {
    if url.contains("https://") {
        return 443;
    } else if url.contains("http://") {
        return 80;
    } else {
        panic!("Malformed url");
    }
}

fn get_path(url: &String) -> io::Result<String> {
    let mut url_without_protocol = url.replace("http://", "");
    url_without_protocol = url_without_protocol.replace("https://", "");
    if !url_without_protocol.contains("/") {
        return Ok("/".to_string());
    } else {
        let path =
            url_without_protocol.replace(url_without_protocol.split("/").next().unwrap_or(""), "");
        return Ok(path);
    }
}

fn open_socket<A: ToSocketAddrs>(addr: A) -> Result<TcpStream, &'static str> {
    match TcpStream::connect(addr) {
        Ok(socket) => Ok(socket),
        Err(_) => Err("Failed to connect to the server."),
    }
}

fn open_stream(host: &str, port: u16) -> Result<InternalStream, Box<dyn std::error::Error>> {
    let addr = (host, port);
    std::env::set_var("SSL_CERT_FILE", "ca.crt");
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // Load the CA certificate file
    let mut cert_file = File::open("ca.crt").unwrap();
    let mut cert_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_buffer).unwrap();
    // Add the CA certificate to the list of trusted CAs
    builder
        .cert_store_mut()
        .add_cert(openssl::x509::X509::from_pem(&cert_buffer).unwrap())
        .unwrap();
    // Set the SSL verification mode to require a trusted certificate
    builder.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);
    let ssl_connector = builder.build();

    match open_socket(addr) {
        Ok(socket) => match ssl_connector.connect(&host, socket) {
            Ok(s) => Ok(InternalStream::Ssl(s)),
            Err(e) => Err(format!("Failed to establish SSL connection: {}", e).into()),
        },
        Err(e) => Err(format!("Failed to connect to the server: {}", e).into()),
    }
}