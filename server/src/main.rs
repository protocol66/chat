// Server Client
use chrono::{format::*, prelude::*, };
use std::io::*;
use std::net::{TcpListener, TcpStream, };
use std::thread;
use tungstenite::{server::accept, error::Error, WebSocket, Message};

const IP: &str = "127.0.0.1:5000"; // Server IP

fn main() {
    println!("Starting Server at {}", IP);
    println!("{}", get_date_time());
    let listener = TcpListener::bind(IP).unwrap(); // start TCP socket at IP

    println!("Waiting for connection...");
    for stream in listener.incoming() {
        // // iterate over each connection
        thread::spawn(move || {
            let stream = stream.expect("Connection Failed.");
            let read_stream = stream.try_clone().expect("Failed to Clone TcpStream.");
            let send_stream = stream.try_clone().expect("Failed to Clone TcpStream.");
            thread::spawn(move || {
                handle_read(read_stream);
            });
            thread::spawn(move || {
                handle_send(send_stream);
            });
        });

    }

    drop(listener);
}

// I: None
// O: formatted data and time for use in println!()
fn get_date_time() -> DelayedFormat<StrftimeItems<'static>> {
    Local::now().format("%a %b %e %r %Y")
}

// I: writer and msg to be sent
// O: Nothing if success, error if msg could not be sent
fn send_string(writer: &mut BufWriter<&TcpStream>, msg: &str) -> Result<()> {
    writer.write(msg.as_bytes()).unwrap();
    writer.flush()
}

fn handle_read(stream: TcpStream) {
    let mut socket = accept(stream).unwrap();
    loop {
        let client_msg = socket.read_message();
        match client_msg {
            Ok(msg) => println!("Client ({}): {}", get_date_time(), msg),
            Err(e) =>  {
                match e {
                    Error::ConnectionClosed => println!("Error: Client Disconnected."),
                    _ => (),
                }
                break;
            }
        }
    }
}

fn handle_send(stream: TcpStream) {
    let mut socket = accept(stream).unwrap();
    loop {
        let mut server_msg = String::new();
        stdin().read_line(&mut server_msg).unwrap();
        match socket.write_message(Message::from(server_msg)) {
            Ok(_) => (), // Do nothing else on success
            Err(e) =>  {
                match e {
                    Error::ConnectionClosed => println!("Error: Client Disconnected."),
                    _ => (),
                }
                break;
            }
        }
    }
}

