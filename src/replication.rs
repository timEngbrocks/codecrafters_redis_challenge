use core::panic;
use std::fmt::Display;
use tokio::net::TcpStream;
use crate::{resp::{array::RespArray, bulk_string::RespBulkString, RespValues}, util::{await_response, generate_master_replid, ok_response, ping_response, request}, Args, LISTENING_PORT};

#[derive(PartialEq, Clone)]
pub enum ReplicationRole {
	Master,
	Slave
}

impl Display for ReplicationRole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ReplicationRole::Master => write!(f, "master"),
			ReplicationRole::Slave => write!(f, "slave"),
		}
	}
}

pub struct ReplicationInfo {
	pub role: ReplicationRole,
	pub master_replid: String,
	pub master_repl_offset: usize,
	pub second_repl_offset: usize,
	pub repl_backlog_active: usize,
	pub repl_backlog_size: usize,
	pub repl_backlog_first_byte_offset: usize,
	pub repl_backlog_histlen: usize,
	pub master_host: String,
	pub master_port: u16,
}

impl Display for ReplicationInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "# Replication")?;
		writeln!(f, "role:{}", self.role)?;
		writeln!(f, "connected_slaves:{}", unsafe { REPLICATION_CONFIGURATION.slaves.len() })?;
		writeln!(f, "master_replid:{}", self.master_replid)?;
		writeln!(f, "master_repl_offset:{}", self.master_repl_offset)?;
		writeln!(f, "second_repl_offset:{}", self.second_repl_offset)?;
		writeln!(f, "repl_backlog_active:{}", self.repl_backlog_active)?;
		writeln!(f, "repl_backlog_size:{}", self.repl_backlog_size)?;
		writeln!(f, "repl_backlog_first_byte_offset:{}", self.repl_backlog_first_byte_offset)?;
		writeln!(f, "repl_backlog_histlen:{}", self.repl_backlog_histlen)?;
		if self.role == ReplicationRole::Slave {
			writeln!(f, "master_host:{}", self.master_host)?;
			writeln!(f, "master_port:{}", self.master_port)?;
		}
		Ok(())
	}
}

pub static mut REPLICATION_STATE: Option<ReplicationInfo> = None;
pub fn replication_state() -> &'static ReplicationInfo {
	unsafe {
		match &REPLICATION_STATE {
			Some(v) => v,
			None => panic!("Tried accessing REPLICATION_STATE before server was initialized!"),
		}
	}
}

#[derive(Clone)]
pub struct ReplicationSlave {
	pub port: u16,
	pub capabilities: Vec<String>,
}

impl Display for ReplicationSlave {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Slave: port={}, capabilities={}", self.port, self.capabilities.join(", "))?;
		Ok(())
	}
}

struct ReplicationConfiguration {
	pub slaves: Vec<ReplicationSlave>,
}

static mut REPLICATION_CONFIGURATION: ReplicationConfiguration = ReplicationConfiguration {
	slaves: Vec::new(),
};
pub fn add_replication_slave(slave: ReplicationSlave) {
	unsafe {
		assert!(matches!(REPLICATION_STATE, Some(ReplicationInfo { role: ReplicationRole::Master, ..})));
		REPLICATION_CONFIGURATION.slaves.push(slave);

		println!("Added replication slave! Now got {} slaves", REPLICATION_CONFIGURATION.slaves.len());
		println!("Current slaves are:");
		REPLICATION_CONFIGURATION.slaves.iter().for_each(|s| println!("{s}"));
	}
}

pub async fn initialize_replication(args: Args) {
	println!("Initializing replication.");

	let (role, master_host, master_port) = match args.replica_of {
		Some(v) => {
			assert!(v.len() == 2);
			let master_host = v[0].clone();
			let master_port = match v[1].parse::<u16>() {
				Ok(v) => v,
				Err(e) => panic!("--replicaof: Could not parse master port ('{}'), got: {}", v[1], e),
			};
			(ReplicationRole::Slave, master_host, master_port)
		},
		None => (ReplicationRole::Master, String::from(""), 0)
	};

	unsafe {
		REPLICATION_STATE = Some(ReplicationInfo {
			role: role.clone(),
			master_replid: generate_master_replid(),
			master_repl_offset: 0,
			second_repl_offset: 0,
			repl_backlog_active: 0,
			repl_backlog_size: 0,
			repl_backlog_first_byte_offset: 0,
			repl_backlog_histlen: 0,
			master_host: master_host.clone(),
			master_port,
		});
	}

	match role {
		ReplicationRole::Slave => {
			execute_replication_handshake(&master_host, &master_port).await;
		},
		ReplicationRole::Master => ()
	};
}

async fn execute_replication_handshake(master_host: &str, master_port: &u16) {
	let mut stream = match TcpStream::connect(format!("{}:{}", master_host, master_port)).await {
		Ok(s) => s,
		Err(e) => panic!("Could not connect to replication master at '{}:{}', got: {}", master_host, master_port, e),
	};

	let ping_request = RespValues::Array(RespArray::from_raw(vec![
		RespValues::BulkString(RespBulkString::from_raw(String::from("ping").into_bytes())),
	]));

	request(&mut stream, ping_request).await;
	let response = await_response(&mut stream).await;
	match response {
		Some(r) => {
			if r != ping_response() {
				panic!("Replication handshake: Received incorrect response from master for ping request");
			}
		},
		None => panic!("Replication handshake: Master did not reply to ping request"),
	};

	let request_data = RespValues::Array(RespArray::from_raw(vec![
		RespValues::BulkString(RespBulkString::from_raw(String::from("REPLCONF").into_bytes())),
		RespValues::BulkString(RespBulkString::from_raw(String::from("listening-port").into_bytes())),
		RespValues::BulkString(RespBulkString::from_raw(unsafe { LISTENING_PORT.to_string() }.into_bytes())),
	]));
	request(&mut stream, request_data).await;
	let response = await_response(&mut stream).await;
	match response {
		Some(r) => {
			if r != ok_response() {
				panic!("Replication handshake: Received incorrect response from master for 1st REPLCONF request");
			}
		},
		None => panic!("Replication handshake: Master did not reply to 1st REPLCONF request"),
	};

	let request_data = RespValues::Array(RespArray::from_raw(vec![
		RespValues::BulkString(RespBulkString::from_raw(String::from("REPLCONF").into_bytes())),
		RespValues::BulkString(RespBulkString::from_raw(String::from("capa").into_bytes())),
		RespValues::BulkString(RespBulkString::from_raw(String::from("psync2").into_bytes())),
	]));
	request(&mut stream, request_data).await;
	let response = await_response(&mut stream).await;
	match response {
		Some(r) => {
			if r != ok_response() {
				panic!("Replication handshake: Received incorrect response from master for 2nd REPLCONF request");
			}
		},
		None => panic!("Replication handshake: Master did not reply to 2nd REPLCONF request"),
	};

	let request_data = RespValues::Array(RespArray::from_raw(vec![
		RespValues::BulkString(RespBulkString::from_raw(String::from("PSYNC").into_bytes())),
		RespValues::BulkString(RespBulkString::from_raw(String::from("?").into_bytes())),
		RespValues::BulkString(RespBulkString::from_raw(String::from("-1").into_bytes())),
	]));
	request(&mut stream, request_data).await;
	let response = await_response(&mut stream).await;
	match response {
		Some(r) => {
			match r {
				RespValues::SimpleString(s) => {
					let args = s.inner().split(' ').collect::<Vec<&str>>();
					assert!(!args.is_empty());
					match args[0] {
						"FULLRESYNC" => {
							assert!(args.len() == 3);
							let master_replid = args[1].to_string();
							let master_reploffset = match args[2].parse::<usize>() {
								Ok(v) => v,
								Err(e) => panic!("Replication handshake: Can not parse master_reploffset from FULLRESYNC response to PSYNC, got: '{}'", e),
							};
							unsafe {
								if let Some(v) = &mut REPLICATION_STATE {
									v.master_replid = master_replid;
									v.master_repl_offset = master_reploffset;
								} else {
									panic!("Finished replication handshake but REPLICATION_STATE has not been initialized!");
								}
							}
						},
						v => panic!("Replication handshake: Received {v} as a reply to PSYNC from master. Do not know how to handle this yet!"),
					}
				},
				_ => panic!("Replication handshake: Received unexpected response from master to PSYNC request, got: '{}'", r),
			}
		},
		None => panic!("Replication handshake: Master did not reply to PSYNC request"),
	};

	println!("Successfully connected to replication master.")
}

