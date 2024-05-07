use std::fs::File;
use std::path::Path;

pub fn check_setup() -> bool {
	let start_check: &str = "./.check";
	if does_file_exists(start_check) {
		return true;
	} else {
		create_file(start_check);
		return false;
	}
}

fn does_file_exists(path: &str) -> bool {
	Path::new(path).try_exists().is_ok()
}

fn create_file(path: &str) -> bool {
	File::create(path).is_ok()
}
