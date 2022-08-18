use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[derive(Debug)]
enum ParseError<'a>{
    MalformedRequest(&'a str),
    UnsupportedMethod(&'a str),
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ParseError::MalformedRequest(request) => format!("Malformed request:\n{}", request),
            ParseError::UnsupportedMethod(method) => format!("Unsupported method: {}", method),
        };
        write!(f, "{}", message)
    }
}

fn parse_request<'a>(request: &'a str) -> Result<&'a str, ParseError<'a>> {
    match request.lines().take(1).next() {
        Some(first_line) => {
            let tokens: Vec<&str> = first_line.split_whitespace().collect();
            if tokens.len() == 3 {
                if tokens[0] != "GET" {
                    Err(ParseError::UnsupportedMethod(tokens[0]))
                } else if tokens[1] == "/" {
                    Ok("/index.html")
                } else {
                    Ok(tokens[1])
                }
            } else {
                Err(ParseError::MalformedRequest(request))
            }
        }
        None => Err(ParseError::MalformedRequest(request))
    }
}

fn respond(request: Cow<str>) -> String {
    // TODO: What would be a good way to separate out the error handling?
    let response = match parse_request(request.as_ref()) {
        Ok(requested_resource) => {
            if requested_resource == "/slowpage.html" {
                // Simulate slow request
                thread::sleep(Duration::from_secs(5));
            }
            match fs::read_to_string(format!("static{}", requested_resource)) {
                Ok(html) => format!("HTTP/1.1 200 OK\r\n\r\n{}", html),
                Err(_) => {
                    eprint!("Resource not found: {}", requested_resource);
                    let html = fs::read_to_string("static/404.html").unwrap();
                    format!("HTTP/1.1 404 not found\r\n\r\n{}", html)
                }
            }
        }
        Err(error) => {
            eprint!("{}", error);
            String::from("HTTP/1.1 400 bad request")
        }
    };
    response
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer);

    let response = respond(request);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on: {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
