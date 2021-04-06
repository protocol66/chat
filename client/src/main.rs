// Chat Client
use chrono::{format::*, prelude::*};
use get_if_addrs::get_if_addrs;
use std::env::args;
use std::io::*;
use std::thread;
use std::net::TcpStream;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        let server_ip = args[1].as_str();

        let interfaces = get_if_addrs().expect("Could not obtain network interfaces."); // list of all network interfaces
        let client_ip = interfaces[0].ip(); // pick first interface as client ip (could end up being wrong but doesn't affect program)
        println!("Starting Client at {}", client_ip);

        println!("{}", get_date_time());
        let stream = TcpStream::connect((server_ip, 5000)).expect("Could not connect to server."); // connect to server
        println!("Connected to the server...");

        let mut reader = BufReader::new(&stream); // reader to make reading from stream more efficient
        let mut writer = BufWriter::new(&stream); // writer to make writing to stream more efficient

        let mut server_msg = String::new();
        let mut client_msg = String::new();

        println!("Enter your client messages one by one and press return key!");

        loop {
            server_msg.clear();
            client_msg.clear();
            stdin().read_line(&mut client_msg).unwrap(); // input from cli
            match send_string(&mut writer, &client_msg) { // send msg
                Ok(_) => (),  // Do nothing else on success
                Err(_) => {   // Break out of loop if connection dropped and quit
                    println!("Error: Server Disconnected..... Quiting");
                    break;
                }
            }
            match reader.read_line(&mut server_msg) { // read incoming text
                Ok(_) => (),  // Do nothing else on success
                Err(_) => {   // Break out of loop if connection dropped and quit
                    println!("Error: Server Disconnected..... Quiting");
                    break;
                }
            }
            println!("Server ({}): {}", get_date_time(), server_msg);
        }
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
