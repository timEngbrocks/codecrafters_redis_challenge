use std::fmt::Display;

use self::{array::RespArray, bulk_string::RespBulkString, simple_string::RespSimpleString};

pub(crate) mod array;
pub(crate) mod bulk_string;
pub(crate) mod simple_string;

const RESP_TERMINATOR: &str = "\r\n";

pub enum RespValues {
	Array(RespArray),
	BulkString(RespBulkString),
	SimpleString(RespSimpleString),
}

pub trait RespObject {
	fn serialize(&self) -> String;
	fn deserialize(data: &str) -> (usize, RespValues);
}

impl Display for RespValues {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.serialize())
	}
}

impl RespObject for RespValues {
	fn serialize(&self) -> String {
		match self {
			RespValues::Array(v) => v.serialize(),
			RespValues::BulkString(v) => v.serialize(),
			RespValues::SimpleString(v) => v.serialize(),
		}
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());

		let (consumed, value) = match data.chars().next() {
			Some('*') => RespArray::deserialize(data),
			Some('$') => RespBulkString::deserialize(data),
			Some('+') => RespSimpleString::deserialize(data),
			c => panic!("Unknown data type {:?} in '{data}'", c),
		};
		assert_eq!(consumed, data.len());
		(consumed, value)
	}
}

