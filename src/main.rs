use std::{
    io::{prelude ::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let n = stream.read(&mut buf).unwrap();

        if n == 0 {
            break;
        }
        
        stream.write_all(&buf[..n]).unwrap();
    }
}