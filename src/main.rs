// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader};
use std::{io::Write, net::TcpListener};
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(&stream);
                let mut lines = reader.lines();
                let status_line = lines.next().unwrap().unwrap();
                let path = status_line.split_whitespace().nth(1).unwrap();
                let method = status_line.split_whitespace().nth(0).unwrap();

                println!("request method: {}", method);
                if path == "/" {
                    stream
                        .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                        .unwrap();
                } else if path.starts_with("/echo") {
                    let response_body = path.split("/echo/").nth(1).unwrap();
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body);
                    println!("{}", response);
                    stream.write_all(response.as_bytes());
                } else if path.starts_with("/user-agent") {
                    let user_agent = lines
                        .find(|line| line.as_ref().unwrap().starts_with("User-Agent:"))
                        .map(|line_result| match line_result {
                            Ok(line) => {
                                let user_agent =
                                    line.split("User-Agent: ").nth(1).unwrap_or_default();
                                user_agent.to_string()
                            }
                            Err(_) => "Unknown User Agent".to_string(),
                        })
                        .unwrap();
                    // let response_body = path.split("/user-agent").nth(1).unwrap();
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent);
                    println!("response : \r\n{}", response);

                    stream.write(response.as_bytes()).unwrap();
                } else {
                    stream
                        .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                        .unwrap();
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    }
}