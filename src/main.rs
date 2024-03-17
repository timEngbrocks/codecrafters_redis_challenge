

use std::error::Error;

use resp::{RespObject, RespValues};
use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};

use crate::commands::{Command, Commands};

pub(crate) mod commands;
pub(crate) mod resp;

const INPUT_BUFFER_SIZE: usize = 2048;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut input_buffer = [0; INPUT_BUFFER_SIZE];
    loop {
        match stream.read(&mut input_buffer).await {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                let raw_request_data = std::str::from_utf8(&input_buffer[0..n]).expect("RESP should always contain valid ASCII");
                let (consumed, request_data) = RespValues::deserialize(raw_request_data);
                assert_eq!(n, consumed);
                Commands::invoke(&mut stream, request_data).await;
            },
            Err(e) => eprintln!("Terminating connection. Error when reading into input buffer: {e}"),
        }
    }
}
