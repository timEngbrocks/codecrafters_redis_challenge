use std::fmt::Display;

use self::{array::RespArray, bulk_string::RespBulkString, null::RespNull, simple_string::RespSimpleString};

pub(crate) mod array;
pub(crate) mod bulk_string;
pub(crate) mod simple_string;
pub(crate) mod null;

pub(crate) const RESP_TERMINATOR: &str = "\r\n";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RespValues {
	Array(RespArray),
	BulkString(RespBulkString),
	SimpleString(RespSimpleString),
	Null(RespNull),
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
			RespValues::Null(v) => v.serialize(),
		}
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());

		match data.chars().next() {
			Some('*') => RespArray::deserialize(data),
			Some('$') => RespBulkString::deserialize(data),
			Some('+') => RespSimpleString::deserialize(data),
			Some('_') => RespNull::deserialize(data),
			c => panic!("Unknown data type {:?} in '{data}'", c),
		}
	}
}

