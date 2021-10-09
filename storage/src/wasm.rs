use sapp_jsutils::{JsObject, JsObjectWeak};

// Inspired by:
// github.com/optozorax/quad-storage

extern "C" {
	fn storage_contains(key: JsObjectWeak) -> u32;
	fn storage_get(key: JsObjectWeak) -> JsObject;
	fn storage_set(key: JsObjectWeak, data: JsObjectWeak);
}

/// Write to local storage with specified key
pub fn write(key: &str, data: &str) {
	unsafe {
		storage_set(JsObject::string(key).weak(), JsObject::string(data).weak());
	}
}

fn to_string(object: JsObject, contains: u32) -> Option<String> {
	match contains {
		1 => {
			let mut result = String::new();
			object.to_string(&mut result);
			Some(result)
		}
		_ => None,
	}
}

/// Read from local storage with specified key
pub fn read(key: &str) -> Option<String> {
	let object = JsObject::string(key);
	let weak_object = object.weak();

	let result = to_string(unsafe { storage_get(weak_object) }, unsafe {
		storage_contains(weak_object)
	});

	drop(object);
	result
}
