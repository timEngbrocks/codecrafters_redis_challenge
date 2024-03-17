use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};

use resp::{RespObject, RespValues};

use crate::commands::{Command, Commands};

pub(crate) mod commands;
pub(crate) mod resp;

const INPUT_BUFFER_SIZE: usize = 2048;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut input_buffer = [0; INPUT_BUFFER_SIZE];
    
    let result = buf_reader.read(&mut input_buffer);
    match result {
        Ok(n) => {
            let raw_request_data = std::str::from_utf8(&input_buffer[..n]).expect("RESP should always contain valid ASCII");
            let (consumed, request_data) = RespValues::deserialize(raw_request_data);
            assert_eq!(n, consumed);
            Commands::invoke(stream, request_data);
        },
        Err(e) => eprintln!("Could not read into input buffer: {}", e)
    }
}
