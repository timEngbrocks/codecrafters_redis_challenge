use crate::resp::RESP_TERMINATOR;

use super::{RespObject, RespValues};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNull {}

impl RespObject for RespNull {
	fn serialize(&self) -> String {
		// NOTE: CodeCrafters tests with RESP2 which uses null bulk string replys for gets that do not return a value.
		// The null bulk string reply has already been extracted into its own value,
		// because representing a null bulk string reply as its own value is easier to implement and is the case
		// for future versions of RESP.
		// TODO: Maybe switch to RESP3 in the future which uses the following special null value: _\r\n
		format!("$-1{}", RESP_TERMINATOR)
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());
		assert_eq!(data.chars().next(), Some('_'));
		assert_eq!(data.chars().nth(1), Some('\r'));
		assert_eq!(data.chars().nth(1), Some('\n'));
		
		(3, RespValues::Null(RespNull {}))
	}
}