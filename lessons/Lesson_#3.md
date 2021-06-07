# Creating api with Rust

## Lesson: 3

### HTTP connection

HTTP messages are the means by which data between servers and clients are exchanged. There are two types of messages: requests sent by the client to the server, to request the beginning of an action;and answers, which are the response of the server.

First, a new library is added in the project:

``` use std::io::prelude::*; ```

This module is the prelude method that helps to read, write and re-read the response received in bytes.

A new function called Handle Connection is also created in this new function, the TCP flow data is read and printed to see the data that is sent from the browser.

```
use std::net::{TcpListener , TcpStream};
use std::io::prelude::*;

fn main() {
    let _listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in _listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {

}
```

A buffer is added to our server with a size of 256 bytes:

```let mut buffer = [0; 256]; ```

The method stream.read, will be responsible for reading all the bytes received.

``` stream.read(&mut buffer).unwrap(); ```

Second, the bytes is converted into the buffer into a chain and printed.

``` println!("Request: {}", String::from_utf8_lossy(&buffer[..])); ```
