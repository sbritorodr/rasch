use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;
use regex::Regex;

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

// This function is used when the filename/pathname has whitespaces AND 
// isn't between quotes For example. If the file is folder/my file.opus 
// and my command is: rasch "ffmpeg -i [i] [o].mp3"
// ffmpeg won't detect the file, because it interprets 'my' and 'file' as
// two different things.

pub fn isInputQuoted (lineCheck: String, files: Vec<PathBuf>) -> bool {
    
    let reQuotedI:Regex = Regex::new(r"(\'\[i\]\')").unwrap(); //if '[i]' is in the input
    let reWhiteSpace:Regex = Regex::new(r"\s").unwrap(); // detect whitespaces regex: /\s/g
    if !reQuotedI.is_match(&lineCheck){
        for file in files{
            let fileStr = file.to_str().unwrap();
            if !reWhiteSpace.is_match(fileStr){
                return false
            }
        }
    }
    
}