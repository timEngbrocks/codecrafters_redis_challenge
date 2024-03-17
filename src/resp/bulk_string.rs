use crate::resp::RESP_TERMINATOR;

use super::{RespObject, RespValues};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespBulkString {
	values: Vec<u8>,
}

impl RespBulkString {
	pub fn from_raw(values: Vec<u8>) -> RespBulkString {
		RespBulkString {
			values,
		}
	}

	pub fn inner(&self) -> &Vec<u8> {
		&self.values
	}

	pub fn as_str(&self) -> &str {
		std::str::from_utf8(&self.values).expect("RESP should always contain valid ASCII")
	}

	pub fn get(&self, index: usize) -> Option<&u8> {
		self.values.get(index)
	}

	pub fn set(&mut self, index: usize, element: u8) {
		self.values[index] = element;
	}

	pub fn len(&self) -> usize {
		self.values.len()
	}
}

impl RespObject for RespBulkString {
	fn serialize(&self) -> String {
		format!("${}{}{}{}", self.len(), RESP_TERMINATOR, self.as_str(), RESP_TERMINATOR)
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());
		assert_eq!(data.chars().next(), Some('$'));

		let mut length: usize = 1;
		let mut offset: usize = 1;
		for i in 1..data.len() - 2 {
			if data.chars().nth(i) == Some('\r') && data.chars().nth(i + 1) == Some('\n') {
				offset = i + 2;
				length = match data[1..length].parse::<usize>() {
					Ok(l) => l,
					Err(e) => panic!("Error reading length from RespBulkString ('{}'): {}", data, e)
				};
				break;
			}
			match data.chars().nth(i) {
				Some(_) => {
					length += 1
				},
				None => panic!("Misformed RespBulkString: '{}'", data),
			}
		}

		let mut values: Vec<u8> = Vec::new();
		for i in 0..length {
			match data.as_bytes().get(offset + i).copied() {
				Some(c) => values.push(c),
				None => panic!("Misformed RespSimpleString: '{}'", data),
			}
		}
		assert_eq!(data.chars().nth(offset + length), Some('\r'));
		assert_eq!(data.chars().nth(offset + length + 1), Some('\n'));
		
		(offset + length + 2, RespValues::BulkString(RespBulkString {
			values,
		}))
	}
}