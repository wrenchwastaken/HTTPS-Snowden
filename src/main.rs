use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let reader = BufReader::new(&_stream);
                let mut lines = reader.lines();
                let first_line = lines.next().unwrap();
                match first_line {
                    Ok(text) => {
                        let mut datas = text.split_whitespace();
                        let _method = datas.next().unwrap();
                        let path = datas.next().unwrap();
                        if path == "/" {
                            let _ = _stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                        } else {
                            let _ = _stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes());
                        }
                    }
                    Err(_) => {
                        println!("no line found")
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}