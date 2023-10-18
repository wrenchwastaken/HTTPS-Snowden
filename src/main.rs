use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
fn main(){
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming(){
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(&stream);
                let mut lines = reader.lines();
                let request = lines.next().unwrap().unwrap();
                let path = request.split_whitespace().nth(1).unwrap();
                match path{
                    "/" => {
                        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    }
                    _ => {
                        println!("request path {:?}",path);
                        if path.starts_with("/echo/") {
                            let content = path.split("/echo/").nth(1).unwrap();
                            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\n",content.len(),content);
                            stream.write(response.as_bytes()).unwrap();
                        } else {
                            stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}",e);
            }
        }
    }
}