use core::panic;
use std::env;
use std::fmt::Write as fmt_write;
use std::io::{self, Read, Write};
use tcp_stream::{HandshakeError, TcpStream, TLSConfig};

const HTTP_VERSION: &str = "1.0";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments. You must pass the url.");
    }
    let url = &args[1];
    // Print the HTML response to the console
    let host = get_host(url).expect("Error getting host");
    let path = get_path(url).expect("Error getting path");
    let port = get_port(url);
    let html: String;
    match make_request(host, path, port) {
        Ok(response) => {
            println!("{}", response);
            html = response;
            let text = extract_text_from_html(&html);
            println!("{}", text);
        }
        Err(error) => {
            eprintln!("Error making a request to the url {}: {}", url, error);
        }
    }
    Ok(())
}

fn get_stream(host: &str, port: i16) -> TcpStream {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
    stream.set_nonblocking(true).unwrap();

    while !stream.is_connected() {
        if stream.try_connect().unwrap() {
            break;
        }
    }

    if port == 443 {
        let mut stream = stream.into_tls(host, TLSConfig::default());

        while let Err(HandshakeError::WouldBlock(mid_handshake)) = stream {
            stream = mid_handshake.handshake();
        }

        return stream.unwrap();
    } else {
        return stream;
    }
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

fn get_port(url: &String) -> i16 {
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

fn make_request(host: String, path: String, port: i16) -> io::Result<String> {
    // Open a TCP connection to the server
    let mut stream = get_stream(&host, port);
    println!("Connected to the server!");
    // Send the HTTP GET request
    let request = format!(
        "GET {} HTTP/{}\r\nHost: {}\r\n\r\n",
        path, HTTP_VERSION, host
    );
    stream.write_all(request.as_bytes())?;

    // Read the response from the server
    let mut buffer = [0; 1024];
    let mut response = String::new();
    loop {
        if let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            response.push_str(std::str::from_utf8(&buffer[..bytes_read]).unwrap());
        }
    }
    // return the response
    Ok(response)
}

fn extract_text_from_html(html: &String) -> String {
    let mut sb = String::new();
    let mut is_inside_tags = false;
    for c in html.chars() {
        if c == '<' {
            is_inside_tags = true;
        } else if c == '>' {
            is_inside_tags = false
        } else if !is_inside_tags {
            write!(&mut sb, "{}", c).unwrap();
        }
    }
    sb
}
