# Creating api with Rust

## Lesson: 1

### Installation on Linux or Mac OS

```$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh```

We take option 1 "Proceed with Installation (Default)" and continue with the process.

Once installed binary is added to the PATH environment variable with the command:

```source $HOME/.cargo/env```

To check that you already have installed Rust and cargo:

```rustc --version && cargo --version```


## Create a project

Create first project using cargo:

```cargo new api-rest```

where API-REST is the name of the project


## First steps

We should start calling the module will use, in this case we only need to std::net which is a standard library Rust and is sent to call two methods that are TCP Stream and TCP Listener:

``` use std::net::{TcpListener, TcpStream}; ```

Now within the main function type the following line of code:

```
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
}
```

In this case, a value to a variable which was assigned by the struct TcpListener (A structure that represents a socket server) is assigned that you hear from TCP connection 127.0.0.1:8000, For this the bind function allows you to create a new instance.

At the end you add unwrap with which all that indicates is that terminate the connection should it be errors.