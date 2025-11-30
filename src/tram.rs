use std::env;
use toml;

use crate::tramtype::{TramFlag, TramErr, TramConfig};

pub fn list() {

}

pub fn open() {

}

pub fn close() {

}

pub fn edit() {
	
}

pub fn init(argv: &Vec<TramFlag>) -> Result<(), TramErr> {
	
	if argv.len() > 1 { return Err(TramErr::Generic("Too many arguments"))}

	let mut name: String = "New project".into();

	if argv.len() == 1 {
		if let TramFlag::Arg(x) = argv[0] {
			name = env::args().nth(x as usize +1).expect("Expected argument for name");
		}
	}

	let working_dir;

	if let Ok(p) = env::current_dir() {
		working_dir = p;
	} else {
		return Err(TramErr::Generic("Cannot access working directory"));
	}

	std::fs::create_dir_all(working_dir.join(".tram/issues"));
	std::fs::File::create(working_dir.join(".tram/issues.toml"));

	

	std::fs::write(working_dir.join(".tram/config.toml"), toml::to_string(&TramConfig::new(name)).unwrap());
	
	Ok(())
}


pub fn config() {

}