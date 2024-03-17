use std::net::TcpStream;

use crate::resp::{simple_string::RespSimpleString, RespValues};

use super::{respond, Command};

pub struct CommandPing {}

impl Command for CommandPing {
	fn invoke(stream: TcpStream, _data: RespValues) {
		let response = RespValues::SimpleString(RespSimpleString::from_str("PONG"));
		respond(stream, response);
	}
}