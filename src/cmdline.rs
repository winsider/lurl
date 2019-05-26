use clap::{App, Arg};

pub enum ProcessingType {
    GetContent,
    ListFeeds
}

pub struct CommandParameters {
    pub url : String,
    pub output_file : String,
    pub processing : ProcessingType,
}

pub fn parse_cmdline() -> CommandParameters {
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

    let params = CommandParameters { 
        processing : match matches.is_present("feeds") {
            true => ProcessingType::ListFeeds,
            false => ProcessingType::GetContent
        },
        url : String::from(matches.value_of("url").unwrap()),
        output_file : match matches.value_of("out") {
            None => Default::default(),
            Some(s) => String::from(s)
        } 
    };

    params
}