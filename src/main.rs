use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, contents, length) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            let status = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("json_file.json").unwrap();
            let length = contents.len();
        
            (status, length, contents)
        },
        "GET /sleep HTTP:/1.1" => {
            let status = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("json_file.json").unwrap();
            let length = contents.len();

            thread::sleep(Duration::from_secs(5));

            (status, length, contents)
        },
        _ => {
            let status = "HTTP/1.1 404 not found";
            let contents = fs::read_to_string("404.html").unwrap();
            let length = contents.len();
        
            (status, length, contents)
        },
    };
    
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}
