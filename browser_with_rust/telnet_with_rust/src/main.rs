use std::env;
use std::io::{prelude::*, Write};
use std::net::TcpStream;
use std::io;
use std::fmt::Write as fmt_write;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments. You must pass the url.");
    }
    let url = &args[1];
    // Print the HTML response to the console
    let html:String;
    match make_request(url) {
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

fn make_request(url: &String) -> io::Result<String> {
    // Open a TCP connection to the server
    if let Ok(mut stream) = TcpStream::connect(url) {
        println!("Connected to the server!");
        // Send the HTTP GET request
        let request = "GET / HTTP/1.0\r\nHost: example.com\r\n\r\n";
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
    } else {
        return Ok("Couldn't connect to server...".to_string());
    }
}

fn extract_text_from_html(html: &String) -> String {
    let mut sb = String::new();
    let mut is_inside_tags = false;
    for c in html.chars() {
        if c=='<' {
            is_inside_tags = true;
        } else if c=='>' {
            is_inside_tags = false
        } else if !is_inside_tags {
            write!(&mut sb, "{}", c).unwrap();
        }
    }
    sb
}
