// Server Client
use chrono::{format::*, prelude::*};
use std::io::*;
use std::net::{TcpListener, TcpStream};

const IP: &str = "10.129.195.97:5000"; // Server IP

fn main() {
    println!("Starting Server at {}", IP);
    println!("{}", get_date_time());
    let listener = TcpListener::bind(IP).unwrap(); // start TCP socket at IP

    println!("Waiting for connection...");
    for stream in listener.incoming() {
        // iterate over each connection
        let stream = stream.unwrap(); // get TcpStream object from iterator
        let mut reader = BufReader::new(&stream); // reader to make reading from stream more efficient
        let mut writer = BufWriter::new(&stream); // writer to make writing to stream more efficient
        let mut server_msg = String::new();
        let mut client_msg = String::new();

        println!("Connection accepted from {}", stream.peer_addr().unwrap()); // print client IP
        println!("Enter your server messages one by one and press return key!");


        loop {
            client_msg.clear();
            server_msg.clear();
            match reader.read_line(&mut client_msg) { // read incoming text
                Ok(_) => (),  // Do nothing else on success
                Err(_) => {   // Break out of loop if connection dropped and wait for next connection
                    println!("Error: Client Disconnected..... Waiting for next connection...");
                    break;
                },
            };
            println!("Client ({}): {}", get_date_time(), client_msg);
            stdin().read_line(&mut server_msg).unwrap(); // input from cli
            match send_string(&mut writer, &server_msg) { // send msg
                Ok(_) => (), // Do nothing else on success
                Err(_) => {  // Break out of loop if connection dropped and wait for next connection
                    println!("Error: Client Disconnected..... Waiting for next connection...");
                    break;
                },
            }
        }
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
