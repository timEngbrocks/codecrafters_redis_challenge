use crate::resp::RESP_TERMINATOR;

use super::{RespObject, RespValues};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNull {}

impl RespObject for RespNull {
	fn serialize(&self) -> String {
		format!("_{}", RESP_TERMINATOR)
	}

	fn deserialize(data: &str) -> (usize, RespValues) {
		assert!(!data.is_empty());
		assert_eq!(data.chars().next(), Some('_'));
		assert_eq!(data.chars().nth(1), Some('\r'));
		assert_eq!(data.chars().nth(1), Some('\n'));
		
		(3, RespValues::Null(RespNull {}))
	}
}