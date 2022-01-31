//read file system
use std::fs;
//prelude to get access to certain traits that let us read from and write to the stream
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    //We can listen TCP connections with TcpListener
    //bind method returns new instance of TcpListener(Result<T, E>)
    //unwrap -> if error -> panic!

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //incoming method return iterator(sequence of streams)
    //full request and response process(client connects to server, sever generates a response and the server allow us to write our response to the stream)
    for stream in listener.incoming() {
        //if the stream has any error -> terminate our program
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

//stream parameter is mut because its internal state might change
fn handle_connection(mut stream: TcpStream) {
    //buffer 1024 bytes in size
    let mut buffer = [0; 1024];
    //read bytes from TcpStream and put them in the buffer
    stream.read(&mut buffer).unwrap();
    //Convert the bytes to a string and print that string
    //from_utf8_lossy takes &[u8] as a parameter
    //lossy : it will replace the invalid sequence with �
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //validating the request
    //Because we are reading raw bytes in buffer
    //transform variable 'get' into a byte string by adding b""
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    //response format is :
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //CRLF is carriage return(\r) and line feed(\n)
    //Below reponse format means Http version:1.1, status code: 200 and Reason-Phrase:Ok
    //header is Content-Length
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    //response(string data) -> as_bytes -> bytes data (because write method on stream takes &[u8])
    stream.write(response.as_bytes()).unwrap();
    //flush() -> will wait and prevent the program from continuing until all the bytes are written to the conncection
    stream.flush().unwrap();
}
