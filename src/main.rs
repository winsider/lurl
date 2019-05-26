use crate::cmdline::parse_cmdline;
use std::fs;
use scraper::{Html,Selector};

mod cmdline;

fn main() {
	let params = parse_cmdline();

	// Download content
	let body = reqwest::get(&params.url).unwrap().text().unwrap();
	
	match params.processing {
		cmdline::ProcessingType::ListFeeds => {
			// Parse content
			let document = Html::parse_document(&body);
			let types : [&str; 2] = ["application/rss+xml", "application/atom+xml"];
			for typ in &types {
				let selector_expr = format!(r#"html head link[type="{}"]"#, typ);
				let selector = Selector::parse(&selector_expr).unwrap();
				for element in document.select(&selector) {
					println!("{}", element.value().attr("href").unwrap());			
				}
			}
		},
		cmdline::ProcessingType::GetContent => {
			// Write output
			let outfile = params.output_file;
			if !outfile.is_empty() {
				// Write contents to file
				println!("The url given: {}", params.url);
				fs::write(outfile, body).unwrap();
			} else {
				// Write the contents to stdout
				println!("{}", body);		
			}
		}
	};
}

