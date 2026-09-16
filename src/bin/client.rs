use std::{
    io::prelude::*,
    net::TcpStream,
    time::Instant,
    fs::File,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let message = b"hello!";
    let mut buf = [0; 1024];

    let mut file = File::create("rtt.csv").unwrap();

    for _ in 0..1000 {
        let start = Instant::now();

        stream.write_all(message).unwrap();
        
        stream.read(&mut buf).unwrap();
        let rtt = start.elapsed();

        writeln!(file, "{:.6}", rtt.as_secs_f64() * 1000.0).unwrap();
    }

    println!("Recorded 1000 RTT measurements.");
}