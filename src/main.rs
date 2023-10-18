
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::io::net::{TcpListener,TcpStream}

fn main() {
    
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("New Connection Accepted");
                handle_client(_stream);
            }
             Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Client Request: {}", String::from_utf8_lossy(&buffer));
}
