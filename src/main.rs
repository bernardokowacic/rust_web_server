use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};
use rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, length, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            let status = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("json_file.json").unwrap();
            let length = contents.len();
        
            (status, length, contents)
        },
        "GET /sleep HTTP/1.1" => {
            let status = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("json_file.json").unwrap();
            let length = contents.len();

            thread::sleep(Duration::from_secs(10));

            (status, length, contents)
        },
        _ => {
            let status = "HTTP/1.1 404 not found";
            let contents = String::new();
            let length = contents.len();
        
            (status, length, contents)
        },
    };
    
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}
