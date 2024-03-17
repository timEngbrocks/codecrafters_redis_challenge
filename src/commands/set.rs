use core::panic;
use std::collections::HashMap;

use tokio::net::TcpStream;

use crate::{commands::ok_reply, resp::RespValues, store::global_store};

use super::Command;

pub struct CommandSet {}

impl Command for CommandSet {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match data {
			RespValues::Array(a) => {
				assert!(a.len() >= 3);
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

				let mut args = HashMap::new();
				if a.len() >= 5 {
					for i in (3..a.len()).step_by(2) {
						let option_name = match a.get(i).unwrap() {
							RespValues::BulkString(b) => {
								b.as_str().to_string()
							},
							d => panic!("set: Expected BulkString for option name argument, got: '{}'", d),
						};
						let option_value = match a.get(i + 1).unwrap() {
							RespValues::BulkString(b) => {
								b.as_str().to_string()
							},
							d => panic!("set: Expected BulkString for option value argument, got: '{}'", d),
						};
						args.insert(option_name, option_value);
					}
				}

				let expiry_time = if let Some(v) = args.get("px") {
					match v.parse::<u64>() {
						Ok(v) => Some(v),
						Err(e) => panic!("set: Could not parse value ('{v}') for argument px, got: '{e}'"),
					}
				} else {
					None
				};

				global_store().set(key, value, expiry_time);
				ok_reply(stream).await;
			},
			_ => eprintln!("Misformed set command: '{}'", data)
		}
	}
}