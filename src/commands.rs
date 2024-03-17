use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::resp::{null::RespNull, simple_string::RespSimpleString, RespObject, RespValues};

use self::{echo::CommandEcho, get::CommandGet, ping::CommandPing, set::CommandSet};

pub(crate) mod ping;
pub(crate) mod echo;
pub(crate) mod set;
pub(crate) mod get;

pub enum Commands {
	Ping(CommandPing),
	Echo(CommandEcho),
	Set(CommandSet),
	Get(CommandGet),
}

pub async fn respond(stream: &mut TcpStream, response: RespValues) {
	match stream.write_all(response.serialize().as_bytes()).await {
		Ok(_) => (),
		Err(e) => eprintln!("{}", e)
	};
}

pub async fn null_reply(stream: &mut TcpStream) {
	let response = RespValues::Null(RespNull {});
	respond(stream, response).await;
}

pub async fn ok_reply(stream: &mut TcpStream) {
	let response = RespValues::SimpleString(RespSimpleString::from_str("OK"));
	respond(stream, response).await;
}

pub trait Command {
	async fn invoke(stream: &mut TcpStream, data: RespValues);
}

impl Command for Commands {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match &data {
			RespValues::Array(a) => {
				assert!(a.len() > 0);
				match a.get(0).unwrap() {
					RespValues::BulkString(b) => {
						match b.as_str() {
							"ping" => CommandPing::invoke(stream, data).await,
							"echo" => CommandEcho::invoke(stream, data).await,
							"set" => CommandSet::invoke(stream, data).await,
							"get" => CommandGet::invoke(stream, data).await,
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