use std::{
	env::{self},
	fs,
	path::PathBuf,
};

fn main() {
	let contents = fs::read_to_string(get_file_path().unwrap()).unwrap();
	println!("{}", contents);
}

fn get_file_path() -> Option<PathBuf> {
	let args: Vec<String> = env::args().collect();

	if let Some(first_char) = &args[1].chars().next() {
		if first_char.to_string() != "/" {
			let pwd = env::current_dir().unwrap();
			Some(pwd.join(PathBuf::from(&args[1])))
		} else {
			Some(PathBuf::from(&args[1]))
		}
	} else {
		None
	}
}
