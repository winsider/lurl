use clap::{Arg, App};
use curl::easy::{Easy};
use std::io::{stdout, Write};
use std::fs::File;

fn main() {
    let matches = App::new("LURL - Load URL tool")
		.version("0.1.0")
		.author("Lars Tungen <lars@tungen.net>")
		.about("Fetch url content.")
		.arg(Arg::with_name("url")
			.short("u")
			.long("url")
			.takes_value(true)
			.help("An url to load"))
		.arg(Arg::with_name("out")
			.short("o")
			.long("output_file")
			.takes_value(true)
			.help("Output file"))
		.get_matches();
	
    let url = matches.value_of("url");
    if url.is_none() {
        println!("No url given.");
        return;
    }
	let url = url.unwrap();
    println!("The url given: {}", url);
    

    // Write the contents of rust-lang.org to stdout
    let outfile = matches.value_of("out");
	let mut easy = Easy::new();
	easy.url(&url).unwrap();
    if outfile.is_some() {
		let outfile : String = outfile.unwrap().to_string();
		easy.write_function(move |data| {
			let f = File::create(&outfile).unwrap();
			return write_file(f, data);
    	}).unwrap();
    } else {
		easy.write_function(write_stdout).unwrap();
    }
	easy.perform().unwrap();
}

fn write_stdout(data : &[u8]) -> Result<usize, curl::easy::WriteError> {
	Ok(stdout().write(data).unwrap())
}

fn write_file(mut outfile : File, data : &[u8]) -> Result<usize, curl::easy::WriteError> {
	Ok(outfile.write(data).unwrap())
}