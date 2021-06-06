use std::net::{TcpListener , TcpStream};

fn main() {
    let _listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in _listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client!");
            }
            Err(e)=>{
                    println!("Failed connection")
            }
        }
    }

}
