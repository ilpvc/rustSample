use std::io::{Read, Write};
use std::net;

pub fn send_get_request(host: &str, port: i32, path: &str) -> Result<String, std::io::Error> {
    let mut stream = net::TcpStream::connect(format!("{}:{}", host, port))?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();

    stream.read_to_string(&mut response)?;

    Ok(response)
}

pub fn send_post_request(host: &str, port: i32, path: &str,data:&str) -> Result<String, std::io::Error> {
    let mut stream = net::TcpStream::connect(format!("{}:{}",host,port))?;
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        path,
        host,
        data.len(),
        data
    );

    stream.write_all(request.as_bytes())?;
    let mut res = String::new();
    stream.read_to_string(&mut res)?;
    Ok(res)
}
