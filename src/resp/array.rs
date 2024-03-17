use crate::resp::RESP_TERMINATOR;

use super::{RespObject, RespValues};

pub struct RespArray {
	values: Vec<RespValues>
}

impl RespArray {
	pub fn from_raw(values: Vec<RespValues>) -> RespArray {
		RespArray {
			values
		}
	}

	pub fn inner(&self) -> &Vec<RespValues> {
		&self.values
	}

	pub fn get(&self, index: usize) -> Option<&RespValues> {
		self.values.get(index)
	}

	pub fn set(&mut self, index: usize, element: RespValues) {
		self.values[index] = element;
	}

	pub fn len(&self) -> usize {
		self.values.len()
	}
}

impl RespObject for RespArray {
	fn serialize(&self) -> String {
		let serialized_values: Vec<String> = self.values.iter().map(|v| { v.serialize() }).collect();
		let serialized_values = serialized_values.join("\r\n");
		format!("*{}{}{}", self.values.len(), RESP_TERMINATOR, serialized_values)
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());
		assert_eq!(data.chars().next(), Some('*'));

		let mut length: usize = 1;
		let mut offset: usize = 1;
		for i in 1..data.len() - 2 {
			if data.chars().nth(i) == Some('\r') && data.chars().nth(i + 1) == Some('\n') {
				offset = i + 2;
				length = match data[1..length].parse::<usize>() {
					Ok(l) => l,
					Err(e) => panic!("Error reading length from RespArray ('{}'): {}", data, e)
				};
				break;
			}
			match data.chars().nth(i) {
				Some(_) => {
					length += 1
				},
				None => panic!("Misformed RespArray: '{}'", data),
			}
		}
		let mut values = Vec::new();
		for _ in 0..length {
			let (consumed, value) = RespValues::deserialize(&data[offset..]);
			values.push(value);
			offset += consumed;
		}
		
		(offset, RespValues::Array(RespArray {
			values
		}))
	}
}