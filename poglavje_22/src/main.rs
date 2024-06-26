use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self},
    time::Duration,
};
use poglavje_22::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
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

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

// Request: [
//     "GET / HTTP/1.1",
//     "Host: 127.0.0.1:7878",
//     "Connection: keep-alive",
//     "Cache-Control: max-age=0",
//     "sec-ch-ua: \"Chromium\";v=\"124\", \"Google Chrome\";v=\"124\", \"Not-A.Brand\";v=\"99\"",
//     "sec-ch-ua-mobile: ?0",
//     "sec-ch-ua-platform: \"Windows\"",
//     "Upgrade-Insecure-Requests: 1",
//     "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
//     "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
//     "Sec-Fetch-Site: none",
//     "Sec-Fetch-Mode: navigate",
//     "Sec-Fetch-User: ?1",
//     "Sec-Fetch-Dest: document",
//     "Accept-Encoding: gzip, deflate, br, zstd",
//     "Accept-Language: sl-SI,sl;q=0.9,en-GB;q=0.8,en;q=0.7",
// ]

// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body



// IP internet protocol
// ICP transmission control protocol
// HTTP hypertext transfer protocol