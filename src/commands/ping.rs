use tokio::net::TcpStream;

use crate::{resp::RespValues, util::{ping_response, respond}};

use super::Command;

pub struct CommandPing {}

impl Command for CommandPing {
	async fn invoke(stream: &mut TcpStream, _data: RespValues) {
		respond(stream, ping_response()).await;
	}
}