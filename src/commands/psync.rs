use core::panic;

use tokio::net::TcpStream;

use crate::{replication::REPLICATION_STATE, resp::{simple_string::RespSimpleString, RespValues}, util::respond};

use super::Command;

pub struct CommandPsync {}

impl Command for CommandPsync {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match data {
			RespValues::Array(a) => {
				assert!(a.len() == 3);
				let master_replication_id = match a.get(1).unwrap() {
					RespValues::BulkString(b) => {
						b.as_str().to_string()
					},
					d => panic!("PSYNC: Expected BulkString for master replication id argument, got: '{}'", d),
				};
				let offset = match a.get(2).unwrap() {
					RespValues::BulkString(b) => {
						match b.as_str().to_string().parse::<isize>() {
							Ok(v) => v,
							Err(e) => panic!("Could not parse offset from PSYNC, got: '{}'", e)
						}
					},
					d => panic!("PSYNC: Expected BulkString for offset argument, got: '{}'", d),
				};
				reply_to_psync(stream, master_replication_id, offset).await;
			},
			_ => eprintln!("Misformed get command: '{}'", data)
		}
	}
}

async fn reply_to_psync(stream: &mut TcpStream, master_replication_id: String, offset: isize) {
	if master_replication_id == "?" && offset == -1 {
		let repl_id = unsafe {
			if let Some(v) = &REPLICATION_STATE {
				&v.master_replid
			} else {
				panic!("Received PSYNC but REPLICATION_STATE has not been initialized!");
			}
		};
		let response = RespValues::SimpleString(RespSimpleString::from_str(&format!("FULLRESYNC {} 0", repl_id)));
		respond(stream, response).await;
	}
}