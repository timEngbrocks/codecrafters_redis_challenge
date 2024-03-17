use std::{io::Write, net::TcpStream};

use crate::resp::{RespObject, RespValues};

use self::ping::CommandPing;

pub(crate) mod ping;

pub enum Commands {
	Ping(CommandPing),
}

pub fn respond(mut stream: TcpStream, response: RespValues) {
	match stream.write_all(response.serialize().as_bytes()) {
		Ok(_) => (),
		Err(e) => eprintln!("{}", e)
	};
}

pub trait Command {
	fn invoke(stream: TcpStream, data: RespValues);
}

impl Command for Commands {
	fn invoke(stream: TcpStream, data: RespValues) {
		match &data {
			RespValues::Array(a) => {
				assert_eq!(a.len(), 1);
				match a.get(0).unwrap() {
					RespValues::BulkString(b) => {
						match b.as_str() {
							"ping" => CommandPing::invoke(stream, data),
							command => panic!("Unknown command: '{command}'"),
						}
					},
					_ => panic!("Unknown command structure: '{data}'")
				}
			},
			_ => panic!("Unknown command structure: '{data}'")
		}
	}
}