/* This is a CLI app
It should work as shown below:

rs-batch ""ffmpeg -i [i] -c copy [o].mp3" -o ./output -f .mov

In this example I'm using ffmpeg, but theoretically should work with any batch process
*/ 

use clap::Parser;
use std::{path::{PathBuf}, fs::{read_dir, ReadDir}, ffi::{OsString}, process::Stdio};

use crate::tools::*;
mod tools;

use execute::Execute;


#[derive(Parser)]
#[clap(
	author = "sbritorodr", 
	version = "v0.1", 
	about = "Automate your commands!", 
	long_about = 
	"Rust CLI used to execute any terminal commands selecting multiple files inside a directory.\n
For example, if you want to convert all files to mp4 in ffmpeg:
rs-batch \"ffmpeg -i [i] -c:v libx264 [o]\" \n
Wether '[i]' is the input files and [o] is the output files.
\n
Also, you can choose not to select files and only loop n times a command. This is
useful for something like repeating \"echo rash\" n times. To a maximun of 255"
)]

struct Cli{ //Here is all the arguments needed
	#[clap(help = "Add the desired command between \"\" and using [i] & [o]")]
	command: String,
	#[clap(short = 'p', long = "path", default_value = "./" ,long ,value_parser, help = "Selects input path files")]
	path: std::path::PathBuf,
	#[clap(short = 'f', long = "file", default_value = ".", long, value_parser, help = "Selects only files with any extension")]
	file_extension: String,
	//recursive flag (Future)
	#[clap(short = 'o', long = "output", default_value = "./output/", long, value_parser, help = "[APLHA] Selects output folder")]
	output: std::path::PathBuf, //output 
	#[clap(short = 'k', long = "keep-ext", required = false, help = "Keep file extensions to output files")] //if it's true, the output will also have the original file name extensions.
	copy_ext: bool,
	#[clap(short = 'v', long = "verbose", required = false, help = "Prints all processing for debugging")]
	verbose:bool,
	#[clap(short = 'l', long = "loop-times", required = false, default_value = "0", help = "Loops the command n times and disables file selection")]
	loop_times:u8
}

fn main() {
	let args = Cli::parse();
	let default_path = read_dir("./").expect("Couldn't read current folder. Check user permissions.");
	let mut files:Vec<PathBuf> = vec![];

	let ext:String = add_dot(args.file_extension.clone());
	if args.verbose { //All diagnostics
		eprintln!("Keep File Extension?: {:?}", args.copy_ext);
		eprintln!("Command chosen: {:?}", args.command);
		eprintln!("Given path name: {:?}", args.path);
		eprintln!("Given output path name: {:?}", args.output);
		eprintln!("Loop times (0 means disabled): {:?}", args.loop_times)
	};

	let dir:ReadDir = match &args.path.is_file() { // this part chooses the parent folder if for some reason the user selected a file instead
		true => read_dir(use_parent_dir(&args.path)).unwrap_or(default_path),
		false => read_dir(&args.path).unwrap_or(default_path),
	};

	for file in dir {
		Vec::push(&mut files, file.unwrap().path())
	};

	let mut filtered_files:Vec<PathBuf> =[].to_vec();
	for i in files.iter() {
		if i.to_string_lossy().find(&ext) != None {
			filtered_files.push(i.to_path_buf());
		}
		else {
			// eprintln!("Error! Selected file extension: {}", ext);
		}
	};
	if filtered_files.is_empty(){ // Halts all if for some reason you didn't select any files
		panic!("No files selected!!")
	};
	if args.verbose {println!("Selected files: {:#?}", filtered_files)};

	//Now, we need to store in a vec the command replacing the '[i]' with each files
	if args.loop_times > 0 {
		let mut i:u8 = 0;
		while i < args.loop_times {
			i += 1;
			let mut command = execute::command(&args.command);
			command.stdout(Stdio::piped());
			let output = command.execute_output().expect("Error at printing out the output of the command. But it worked at the back.");
			print!("{}", String::from_utf8(output.stdout).expect("Error converting the output to utf8, does it use weird characters?. The command is working without printing output info."));
		}
	} else {
		let mut commands:Vec<OsString> = Vec::new();
		let mut filtered_string:String;
		create_output_folder(&args.output);

		
		//Below is the [i] and [o] substitution loop
		for i in filtered_files.iter() {
			let file_stem = if args.copy_ext{ //if the user selected -ce flag, use the filename + extension
				i.as_os_str()
			} else {
				i.file_stem().unwrap()

			};
			//let output_filestem_str = pathbuf_to_str(os_str_to_pathbuf).unwrap(); // Let bindings is giving me crazy
			let mut output_filestem_str:&str = file_stem.to_str().unwrap();
			let output_folder_string = if args.output.to_str().unwrap().chars().last().unwrap() != '/' {
				format!("{}{}", args.output.as_path().to_str().unwrap(), "/")
			} else {
				format!("{}", args.output.as_path().to_str().unwrap())
			};
			let output_folder_str = output_folder_string.as_str();
			
			output_filestem_str = if args.copy_ext {
				&output_filestem_str[1..]
			} else {
				&output_filestem_str
			};
			dbg!(&output_filestem_str);
			let output_string:String = format!("{}{}", output_folder_str, &output_filestem_str);
			let output_str:&str = &output_string.as_str();
			
			filtered_string = args.command.replace("[i]", &i.as_path().to_str().unwrap()).replace("[o]", output_str); 
			commands.push(OsString::from(&filtered_string));
		}
		if args.verbose {eprintln!("\nCommands: {:#?}", commands)};
		for eachcommand in commands.iter(){
			let eachcommand_string = eachcommand.clone().into_string().unwrap();
			let mut command = execute::command(eachcommand_string);
			command.stdout(Stdio::piped());
			let output = command.execute_output().unwrap();
			print!("{}", String::from_utf8(output.stdout).unwrap());
		}
	}
}
