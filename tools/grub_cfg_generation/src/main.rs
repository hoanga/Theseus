// Writes the "/cfg/grub_test.cfg" file

extern crate getopts;

use getopts::Options;
use std::fs;
use std::io::Write;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file path, e.g., \"/my/dir/grub.cfg\"", "OUTPUT_PATH");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage("cargo run -- ", opts);
        process::exit(0);
    }

    // Require input directory 
    let input_directory = match matches.free.len() {
        0 => {
            eprintln!("No input directory");
            process::exit(-1);
        },
        1 => matches.free[0].clone(), 
        _ => { 
            eprintln!("Too many arguments entered");
            process::exit(-1);
        },
    };
    
    let grub_cfg_string = match create_grub_cfg_string(input_directory) {
		Ok(s) => s,
		Err(_e) => { 
			eprintln!("Error: {:?}", _e);
			process::exit(-1);
		}
	};
    
    // Write to file 
    if matches.opt_present("o") {
        let output_file_path = match matches.opt_str("o") {
            Some(s) => s, 
            None    => process::exit(-1)
        };
        write_content(grub_cfg_string, output_file_path);
    }
    // Write to stdout by default
    else {
        println!("{}", grub_cfg_string);
    }

}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] INPUT_DIRECTORY", program);
    print!("{}", opts.usage(&brief));
}

fn create_grub_cfg_string(input_directory: String) -> Result<String, String> {
    // Creates string to write to grub.cfg file by looking through all files in input_directory
    let mut content = String::new();
    
    let mut path_to_exe = std::env::current_exe().unwrap_or(std::path::PathBuf::new());
    // go up three directories to remove the "target/<build_mode>/name"
    path_to_exe.pop(); path_to_exe.pop(); path_to_exe.pop();

    content.push_str("### This file has been autogenerated, do not manually modify it!\n");
    content.push_str(&format!("### Generated by program: \"{}\"\n", path_to_exe.display()));
    content.push_str(&format!("### Input directory: \"{}\"\n\n", &input_directory));
    content.push_str("set timeout=0\n");
    content.push_str("set default=0\n\n");
    content.push_str("menuentry \"Theseus OS\" {\n");
    content.push_str("\tmultiboot2 /boot/kernel.bin \n");

    for path in fs::read_dir(input_directory).map_err(|e| e.to_string())? {
        let path = path.map_err(|e| e.to_string())?;
        let p = path.path();
        let file_name = p.file_name().and_then(|f| f.to_str()).ok_or(format!("Path error in path {:?}", path))?;
        content.push_str(&format!("\tmodule2 /modules/{0:25}\t\t{1:25}\n", file_name, file_name));
    }

    content.push_str("\n\tboot\n}\n");
    Ok(content)
}

fn write_content(content: String, output_file_path: String) {
    if let Ok(mut file) = fs::File::create(output_file_path) {
        if file.write(content.as_bytes()).is_ok(){ process::exit(0); }
    }
}