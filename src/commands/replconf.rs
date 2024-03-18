use tokio::net::TcpStream;

use crate::{replication::{add_replication_slave, ReplicationSlave}, resp::RespValues, util::ok_reply};

use super::Command;

pub struct CommandReplconf {}

static mut NEXT_SLAVE: ReplicationSlave = ReplicationSlave {
    port: 0,
    capabilities: Vec::new(),
};

impl Command for CommandReplconf {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match data {
			RespValues::Array(a) => {
				assert!(a.len() >= 3);
				let stage = match a.get(1).unwrap() {
					RespValues::BulkString(b) => {
						b.as_str().to_string()
					},
					d => panic!("REPLCONF: Expected BulkString as 2nd argument, got: '{}'", d),
				};
				match stage.as_str() {
					"listening-port" => {
						let slave_port = match a.get(2).unwrap() {
							RespValues::BulkString(b) => {
								match b.as_str().parse::<u16>() {
									Ok(v) => v,
									Err(e) => panic!("REPLCONF: Could not parse slave port, got: {}", e),
								}
							},
							d => panic!("REPLCONF: Expected BulkString as 3rd argument for listening-port, got: '{}'", d),
						};
						unsafe {
							assert!(NEXT_SLAVE.capabilities.is_empty());
							NEXT_SLAVE.port = slave_port;
						}
						ok_reply(stream).await;
					},
					"capa" => {
						let mut capabilities = Vec::new();
						for i in (1..a.len()).step_by(2) {
							let key = match a.get(i).unwrap() {
								RespValues::BulkString(b) => {
									b.as_str().to_string()
								},
								d => panic!("REPLCONF: Expected BulkString as argument, got: '{}'", d),
							};
							assert_eq!(key, String::from("capa"));
							let value = match a.get(i + 1).unwrap() {
								RespValues::BulkString(b) => {
									b.as_str().to_string()
								},
								d => panic!("REPLCONF: Expected BulkString as argument, got: '{}'", d),
							};
							capabilities.push(value);
						}
						unsafe {
							assert!(NEXT_SLAVE.port > 0);
							NEXT_SLAVE.capabilities = capabilities;
							add_replication_slave(NEXT_SLAVE.clone());
							NEXT_SLAVE.port = 0;
							NEXT_SLAVE.capabilities.clear();
						}
						ok_reply(stream).await;
					},
					_ => panic!("REPLCONF: Unknown handshake stage: '{}'", stage)
				}
			},
			_ => eprintln!("Misformed get command: '{}'", data)
		}
	}
}