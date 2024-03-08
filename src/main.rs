mod http;
mod sql;
mod test;

use http::http_request;

fn main() {
    let response = http_request::send_get_request("192.168.10.187", 8567, "/status");
    match response {
        Ok(res) =>{
            println!("result:\n{}", res)
        },
        Err(e) =>{
            println!("error:\n{}",e)
        }
    }

    let download_path = http_request::send_post_request("192.168.10.187", 8567, "/downloadPath","asd");

    println!("{:?}",download_path)
}


