use std::{
    env::set_current_dir, io::{prelude, BufRead, BufReader, Read, Write}, iter::Map, net::{TcpListener, TcpStream}, thread
};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        match stream {
           Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
           }
           Err(err) => {
            println!("Connection error: {}", err)
           }
        }
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("Failed to read the line"))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:?}", http_request);
        
}