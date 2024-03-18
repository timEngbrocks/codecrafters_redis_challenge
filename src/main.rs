

use std::error::Error;

use resp::{RespObject, RespValues};
use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};
use clap::Parser;

use crate::{commands::{Command, Commands}, replication::initialize_replication};

pub(crate) mod commands;
pub(crate) mod resp;
pub(crate) mod store;
pub(crate) mod replication;
pub(crate) mod util;

pub const INPUT_BUFFER_SIZE: usize = 2048;
pub static mut LISTENING_PORT: u16 = 6379;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    port: Option<u16>,

    #[arg(long = "replicaof", value_delimiter = ' ', num_args = 2)]
    replica_of: Option<Vec<String>>
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let listener = unsafe {
        LISTENING_PORT = args.port.unwrap_or(LISTENING_PORT);
        println!("Listening on port: {}", LISTENING_PORT);
        TcpListener::bind(format!("127.0.0.1:{}", LISTENING_PORT)).await?
    };

    server_initialization(args).await;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn server_initialization(args: Args) {
    println!("Initializing server.");

    initialize_replication(args).await;
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
