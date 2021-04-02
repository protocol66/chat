
use chrono::{prelude::*,
             format::*};
use std::io::*;
use std::thread;
use std::net::{TcpListener,
               TcpStream};

const IP:&str = "127.0.0.1:5000";

fn main() {
    println!("Starting Server at {}", IP);
    println!("{}", get_date_time());
    let listener = TcpListener::bind(IP).unwrap();
    // listener.set_nonblocking(true);

    println!("Waiting for connection...");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let reference = writer.get_mut();
        let mut server_msg = String::new();
        let mut client_msg = String::new();


        println!("Connected accepted from {}", stream.local_addr().unwrap());
        println!("Enter your server messages one by one and press return key!");

        loop {
            reader.read_line(&mut client_msg).unwrap();
            println!("Client ({}): {}", get_date_time() , client_msg);
            stdin().read_line(&mut server_msg);
            send_string(&mut writer, &server_msg);
        }

    }
}

fn get_date_time() -> DelayedFormat<StrftimeItems<'static>>{
    Local::now().format("%a %b %e %r %Y")
}

fn send_string(writer:&mut BufWriter<&TcpStream>, msg: &String) {
    writer.write(msg.as_bytes()).unwrap();
    writer.flush().unwrap();
}

