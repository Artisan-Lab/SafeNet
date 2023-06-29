fn hashed_storage_create(&self, data: &[u8]) -> String {
		unsafe { std::mem::transmute::<Vec<u8>, String>([self, data].concat()) }
}
