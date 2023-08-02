use std::net::{TcpListener, TcpStream, Shutdown};
use threadpool::ThreadPool;

pub fn server_start(port : u16, num_threads : usize) {
    let tcp_listner = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(listner) => listner, 
        Err(_) => panic!("Failed to start tcp socket")
    };

    let pool = ThreadPool::new(num_threads);

    for stream in tcp_listner.incoming() {
        match stream {
            Ok(stream) => println!("{:?}", stream), 
            Err(err) => println!("{}", err)
        }
    }


} 
