# Creating api with Rust

## Lesson: 2

### TcpStream

TcpStream which is a structure that represents a TCP sequence between a local socket and a remote socket, in a few words see if there is a connection between client and server.

```
for stream in listener.incoming() {
    	match stream {
        	Ok(stream) => {
            	println!("New client!");
        	}
            	Err(e) => {
                	println!("Failed connection")
            	}
    	}
	}
```

Stream is an asynchronous version of an iterator, a stream represents an open connection between client and server. After this INCOMING is an iterator Tcp Listener returns us same iterator that gives a sequence of sequences.

To run the code used:

```cargo run```