use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;


pub fn add_dot(input:String) -> String { // adds dot if the user didn't provide any
    if input.find('.') == None {
        ".".to_owned() + &input
    } else {
        input
    }
}

pub fn create_output_folder(output_path:&PathBuf){
    match fs::create_dir_all(output_path){
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::AlreadyExists => panic!("Output folder already exist"),
            _ => panic!("Couldn't read the path given.")
        }
    };
}



pub fn use_parent_dir(path:&PathBuf) -> String { //if it's a file, uses the parent folder
	let mut str_path:String = path.to_str().unwrap().to_string();
	let dot_loc:usize = str_path.rfind('.').unwrap();
	let slash_loc:usize = str_path.rfind('/').unwrap() + 1;
	//println!("Path: {:?}, dotloc: {:?}", path, dot_loc);
	str_path.truncate(dot_loc);
	str_path.truncate(slash_loc);
	//println!("New path: {:?}", path)
	str_path
}