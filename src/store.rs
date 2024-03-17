use std::{collections::HashMap, time::{Duration, Instant}};

struct StoreValue {
	value: String,
	expiry_time: Option<u64>,
	created_at: Instant,
}

impl StoreValue {
	pub fn new(value: String, expiry_time: Option<u64>) -> StoreValue {
		StoreValue {
			value,
			expiry_time,
			created_at: Instant::now(),
		}
	}

	pub fn value(&self) -> Option<&String> {
		if let Some(expiry_time) = self.expiry_time {
			if (Instant::now() - self.created_at) > Duration::from_millis(expiry_time) {
				return None
			}
		}
		Some(&self.value)
	}
}

pub struct Store {
	data: HashMap<String, StoreValue>
}

static mut GLOBAL_STORE: Option<Store> = None;

pub fn global_store() -> &'static mut Store {
	unsafe {
		if GLOBAL_STORE.is_none() {
			GLOBAL_STORE = Some(Store::init())
		}
		GLOBAL_STORE.as_mut().unwrap()
	}
}

impl Store {
	pub fn init() -> Store {
		Store {
			data: HashMap::new()
		}
	}

	pub fn get(&self, key: String) -> Option<&String> {
		match self.data.get(&key) {
			Some(v) => v.value(),
			None => None,
		}
	}

	pub fn set(&mut self, key: String, value: String, expiry_time: Option<u64>) -> Option<String> {
		match self.data.insert(key, StoreValue::new(value, expiry_time)) {
			Some(v) => v.value().cloned(),
			None => None,
		}
	}

	pub fn has(&self, key: String) -> bool {
		self.data.contains_key(&key)
	}
}