fn print_path(dir: std::path::PathBuf) {
	match std::fs::read_dir(dir) {
		Ok(res) => for entry in res {
			let entry = entry.unwrap();
			let path = entry.path();
			println!("{:?}", path.to_str().ok_or("No filename"));
			
			print_path(path);
		},
		Err(_e) => print!(""),
	}
}
fn main() {
	println!("----------------------------------------");
	println!("test dll");
	println!("----------------------------------------");
	for entry in std::env::vars() {
		println!("{:?}", entry);
	}
	let current_dir = std::env::current_dir().unwrap();
    println!("The current directory is {}", current_dir.display());
	print_path(current_dir);
}
