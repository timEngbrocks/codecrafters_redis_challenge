use tokio::net::TcpStream;

use crate::{resp::{bulk_string::RespBulkString, RespValues}, store::global_store, util::{null_reply, respond}};

use super::Command;

pub struct CommandGet {}

impl Command for CommandGet {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match data {
			RespValues::Array(a) => {
				assert!(a.len() == 2);
				let key = match a.get(1).unwrap() {
					RespValues::BulkString(b) => {
						b.as_str().to_string()
					},
					d => panic!("set: Expected BulkString for key argument, got: '{}'", d),
				};

				if let Some(value) = global_store().get(key) {
					let response = RespValues::BulkString(RespBulkString::from_raw(value.clone().into_bytes()));
					respond(stream, response).await;
				} else {
					null_reply(stream).await;
				}
			},
			_ => eprintln!("Misformed get command: '{}'", data)
		}
	}
}