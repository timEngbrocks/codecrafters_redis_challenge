use tokio::net::TcpStream;

use crate::{replication::replication_state, resp::{bulk_string::RespBulkString, RespValues, RESP_TERMINATOR}, util::respond};

use super::Command;

pub struct CommandInfo {}

impl Command for CommandInfo {
	async fn invoke(stream: &mut TcpStream, _data: RespValues) {
		let mut info = Vec::new();

		let replication_info = replication_state().to_string().split('\n').collect::<Vec<&str>>().join(RESP_TERMINATOR);
		info.push(replication_info);

		let info = info.join(RESP_TERMINATOR);
		let response = RespValues::BulkString(RespBulkString::from_raw(info.into_bytes()));
		respond(stream, response).await;
	}
}