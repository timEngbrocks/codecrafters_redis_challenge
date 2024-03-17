use std::collections::HashMap;

pub struct Store {
	data: HashMap<String, String>
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
		self.data.get(&key)
	}

	pub fn set(&mut self, key: String, value: String) -> Option<String> {
		self.data.insert(key, value)
	}

	pub fn has(&self, key: String) -> bool {
		self.data.contains_key(&key)
	}
}