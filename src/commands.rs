use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::resp::{RespObject, RespValues};

use self::{echo::CommandEcho, ping::CommandPing};

pub(crate) mod ping;
pub(crate) mod echo;

pub enum Commands {
	Ping(CommandPing),
	Echo(CommandEcho),
}

pub async fn respond(stream: &mut TcpStream, response: RespValues) {
	match stream.write_all(response.serialize().as_bytes()).await {
		Ok(_) => (),
		Err(e) => eprintln!("{}", e)
	};
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