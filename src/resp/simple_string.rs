use crate::resp::RESP_TERMINATOR;

use super::{RespObject, RespValues};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespSimpleString {
	value: String
}

impl RespSimpleString {
	pub fn from_str(value: &str) -> RespSimpleString {
		RespSimpleString {
			value: value.to_string()
		}
	}

	pub fn inner(&self) -> &str {
		&self.value
	}

	pub fn len(&self) -> usize {
		self.value.len()
	}
}

impl RespObject for RespSimpleString {
	fn serialize(&self) -> String {
		format!("+{}{}", self.value, RESP_TERMINATOR)
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());
		assert_eq!(data.chars().next(), Some('+'));

		let mut value = Vec::new();
		for i in 1..data.len() - 1 {
			if data.chars().nth(i) == Some('\r') && data.chars().nth(i + 1) == Some('\n') {
				return (i + 1, RespValues::SimpleString(RespSimpleString { value: String::from_iter(value) }))
			}
			match data.chars().nth(i) {
				Some(c) => value.push(c),
				None => panic!("Misformed RespSimpleString: '{}'", data),
			}
		}
		panic!("Unterminated RespSimpleString: '{}'", data);
	}
}