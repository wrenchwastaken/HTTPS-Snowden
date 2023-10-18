use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

use std::thread;
fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                

                thread::spawn(|| {
                    let mut data: [u8; 1024] = [0; 1024];
                
                    let req = _stream.read(&mut data).unwrap();

                    let str_req = str::from_utf8(&data[0..req]).unwrap();
                
                    handle_connection(str_req, _stream);

                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn get_path(str_req: &str) -> &str {
    let f_line = str_req.lines().next().unwrap();
    let path = f_line.split(" ").nth(1).unwrap();
    path
}
fn handle_connection(str_req: &str, mut _stream: TcpStream) {
    let path = get_path(str_req);
    if path == "/" {
        _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    } else if path.starts_with("/echo/") {
        let body_str = path.split("/echo/").nth(1).unwrap();
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body_str.len(),
            body_str
        );
        _stream.write(resp.as_bytes());
    } else if path.starts_with("/user-agent") {
        let user_agent_data = str_req
            .split("\r\n")
            .find(|&x| x.contains("User-Ag"))
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim();
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent_data.len(),
            user_agent_data
        );
        _stream.write(resp.as_bytes()).unwrap();
    } else {
        _stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
    };

}