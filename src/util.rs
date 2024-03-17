use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use rand::Rng;
use crate::{resp::{null::RespNull, simple_string::RespSimpleString, RespObject, RespValues}, INPUT_BUFFER_SIZE};

pub async fn respond(stream: &mut TcpStream, response: RespValues) {
	match stream.write_all(response.serialize().as_bytes()).await {
		Ok(_) => (),
		Err(e) => eprintln!("{}", e)
	};
}

pub async fn request(stream: &mut TcpStream, request: RespValues) {
	match stream.write_all(request.serialize().as_bytes()).await {
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

pub async fn await_response(stream: &mut TcpStream) -> Option<RespValues> {
	let mut input_buffer = [0; INPUT_BUFFER_SIZE];
	match stream.read(&mut input_buffer).await {
		Ok(n) => {
			if n == 0 {
				return None;
			}
			let raw_response = std::str::from_utf8(&input_buffer[0..n]).expect("RESP should always contain valid ASCII");
			let (_, response) = RespValues::deserialize(raw_response);
			Some(response)
		},
		Err(e) => panic!("Terminating connection. Error when reading into input buffer: {e}"),
	}
}

pub fn generate_master_replid() -> String {
	let mut rng = rand::thread_rng();
	let mut master_replid: Vec<u8> = Vec::new();
	for _ in 0..40 {
		let n: u8 = rng.gen_range(0..=35);
		let c = match n {
			0..=9 => n + 48,
			10..=35 => n + (97 - 10),
			_ => unreachable!(),
		};
		master_replid.push(c);
	}
	String::from_utf8(master_replid).unwrap()
}

pub fn ping_response() -> RespValues {
	RespValues::SimpleString(RespSimpleString::from_str("PONG"))
}