use clap::{App, Arg};
use std::fs;
use scraper::{Html,Selector};

fn main() {
	let matches = App::new("LURL - Load URL tool")
		.version("0.1.0")
		.author("Lars Tungen <lars@tungen.net>")
		.about("Fetch url content.")
		.arg(
			Arg::with_name("url")
				.short("u")
				.long("url")
				.takes_value(true)
				.required(true)
				.help("An url to load"),
		)
		.arg(
			Arg::with_name("out")
				.short("o")
				.long("output_file")
				.takes_value(true)
				.help("Output file"),
		)
		.arg(
			Arg::with_name("feeds")
				.short("f")
				.long("feeds")
				.takes_value(false)
				.help("List feeds in HTML document (rss)"),
		)
		.get_matches();

	// Download content
	let url = matches.value_of("url");
	let url = url.unwrap();
	let body = reqwest::get(url).unwrap().text().unwrap();

	if matches.value_of("feeds").is_some() {
		// Parse content
		let document = Html::parse_document(&body);
		let selector = Selector::parse("html head link[type='application/rss+xml']").unwrap();
		for element in document.select(&selector) {
			println!("found: {}", element.value().name());
		}
	}
	// Write output
	let outfile = matches.value_of("out");
	if outfile.is_some() {
		// Write contents to file
		println!("The url given: {}", url);
		let outfile: String = outfile.unwrap().to_string();
		fs::write(outfile, body).unwrap();
	} else {
		// Write the contents to stdout
		println!("{}", body);		
	}
}

