// Chat Client
use chrono::{format::*, prelude::*};
use get_if_addrs::get_if_addrs;
use std::env::args;
use std::io::*;
use std::thread;
use std::net::TcpStream;
use tungstenite::{connect, client::{client, IntoClientRequest}, error::Error, WebSocket, Message};
use tungstenite::client::AutoStream;
use url::Url;

const PORT: u16 = 5000;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        let server_ip = args[1].as_str();

        let interfaces = get_if_addrs().expect("Could not obtain network interfaces."); // list of all network interfaces
        let mut client_ip = interfaces[0].ip(); // pick first interface as client ip (could end up being wrong but doesn't affect program)
        if client_ip.is_loopback() {
            client_ip = interfaces[1].ip();
        }
        println!("Starting Client at {}", client_ip);
        println!("{}", get_date_time());

        // let stream = TcpStream::connect((server_ip, PORT)).unwrap();
        println!("Connected to the server...");
        // let read_stream = stream.try_clone().expect("Failed to Clone TcpStream.");
        // let send_stream = stream.try_clone().expect("Failed to Clone TcpStream.");
        thread::spawn(move || {
            handle_send();
        });
        handle_read();

    } else {
        println!("Not enough arguments.")
    }
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

fn handle_read() {
    loop {
        let (mut socket, _response) = connect(IntoClientRequest::into_client_request("ws://127.0.0.1:5000").unwrap()).expect("Could not create read socket.");
        let server_msg = socket.read_message();
        match server_msg {
            Ok(msg) => println!("Server ({}): {}", get_date_time(), msg),
            Err(e) =>  {
                match e {
                    Error::ConnectionClosed => println!("Error: Server Disconnected."),
                    _ => println!("Some other Error"),
                }
                break;
            }
        }
    }
    println!("Exit loop");
}

fn handle_send() {
    loop {
        let (mut socket, _response) = connect(IntoClientRequest::into_client_request("ws://127.0.0.1:5000").unwrap()).expect("Could not create write socket.");
        let mut client_msg = String::new();
        stdin().read_line(&mut client_msg).unwrap();
        match socket.write_message(Message::from(client_msg)) {
            Ok(_) => (), // Do nothing else on success
            Err(e) =>  {
                match e {
                    Error::ConnectionClosed => println!("Error: Server Disconnected."),
                    _ => (),
                }
                break;
            }
        }
    }
}
