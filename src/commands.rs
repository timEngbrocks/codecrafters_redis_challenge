use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::resp::{null::RespNull, simple_string::RespSimpleString, RespObject, RespValues};

use self::{echo::CommandEcho, get::CommandGet, info::CommandInfo, ping::CommandPing, set::CommandSet};

pub(crate) mod ping;
pub(crate) mod echo;
pub(crate) mod set;
pub(crate) mod get;
pub(crate) mod info;

pub enum Commands {
	Ping(CommandPing),
	Echo(CommandEcho),
	Set(CommandSet),
	Get(CommandGet),
	Info(CommandInfo),
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
							"info" => CommandInfo::invoke(stream, data).await,
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