use tokio::net::TcpStream;

use crate::{commands::ok_reply, resp::RespValues, store::global_store};

use super::Command;

pub struct CommandSet {}

impl Command for CommandSet {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match data {
			RespValues::Array(a) => {
				assert!(a.len() == 3);
				let key = match a.get(1).unwrap() {
					RespValues::BulkString(b) => {
						b.as_str().to_string()
					},
					d => panic!("set: Expected BulkString for key argument, got: '{}'", d),
				};
				let value = match a.get(2).unwrap() {
					RespValues::BulkString(b) => {
						b.as_str().to_string()
					},
					d => panic!("set: Expected BulkString for value argument, got: '{}'", d),
				};

				global_store().set(key, value);
				ok_reply(stream).await;
			},
			_ => eprintln!("Misformed set command: '{}'", data)
		}
	}
}