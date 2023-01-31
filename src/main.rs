use std::{net::{TcpListener, TcpStream}, 
io::{BufReader, BufRead, Write}, fs, 
thread::{   self}, time::Duration};

use multithreaded_web_server::ThreadPool;

fn main() {
    println!("Server listening on 127.0.0.1:7878...");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let http_request = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect::<Vec<_>>();

    // println!("Request: {:#?},", http_request);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let body = fs::read_to_string(filename).unwrap();
    let content_length = format!("Content-Length: {}", body.len());

    let response = format!("{status_line}\r\n{content_length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
