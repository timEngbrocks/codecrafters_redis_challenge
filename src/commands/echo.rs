use tokio::net::TcpStream;

use crate::{resp::RespValues, util::respond};

use super::Command;

pub struct CommandEcho {}

impl Command for CommandEcho {
	async fn invoke(stream: &mut TcpStream, data: RespValues) {
		match data {
			RespValues::Array(a) => {
				assert!(a.len() == 2);
				let arg = a.get(1).unwrap().clone();
				respond(stream, arg).await;
			},
			_ => eprintln!("Misformed echo command: '{}'", data)
		}
	}
}