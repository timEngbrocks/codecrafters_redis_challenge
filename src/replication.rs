use std::fmt::Display;
use rand::Rng;
use crate::Args;

#[derive(PartialEq)]
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
	pub connected_slaves: usize,
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
		writeln!(f, "connected_slaves:{}", self.connected_slaves)?;
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

static mut REPLICATION_STATE: Option<ReplicationInfo> = None;
pub fn replication_state() -> &'static ReplicationInfo {
	unsafe {
		match &REPLICATION_STATE {
			Some(v) => v,
			None => panic!("Tried accessing REPLICATION_STATE before server was initialized!"),
		}
	}
}

pub fn initialize_replication(args: Args) {
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
			role,
			connected_slaves: 0,
			master_replid: generate_master_replid(),
			master_repl_offset: 0,
			second_repl_offset: 0,
			repl_backlog_active: 0,
			repl_backlog_size: 0,
			repl_backlog_first_byte_offset: 0,
			repl_backlog_histlen: 0,
			master_host,
			master_port,
		});
	}
}

fn generate_master_replid() -> String {
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