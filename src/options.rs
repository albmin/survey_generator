mod options;

extern crate clap;

use clap::{Arg, App};

use std::path::Path;

struct Options {
    survey_count: usize,
    output_dir: Path, 	    
}

enum OptionsError{

}

fn parse_options() -> Options {
    let matches = App::new("Survey Generator")
			  .arg(Arg::with_name("survey_count")
				.short("c")
				.long("survey-count")
				.value_name("N")
				.help("count of surveys to create")
				.takes_value(true))
			  .arg(Arg::with_name("output_dir")
				.short("o")
				.long("output-directory")
				.value("DIRECTORY")
				.help("output directory for surveys")
				.required(true)	
				.index(1)
			  .get_matches();

    let mut result;
    result.survey_count = value_t_or_exit!(matches.value_of("survey_count"), usize);
    let o = matches.value_of("output_dir").unwrap()); 	

    

}


///parse cli args and returns a file path to the json
///input file. could probably use a third party 
///party package but purposefully using the stdlib
fn parse_options(args: Args) ->  Result<Options,OptionsError> {
    let mut args = args.skip(1);
    let path = match args.next() {
	Some(p) => p,
	None => return Err("Not enough arguments!".to_string()),
    };
    match args.next() {
	Some(_) => Err("Too many arguments!".to_string()),
	None => match File::open(path)
	{
		Ok(x) => Ok(x),
		Err(e) => Err(format!("Failed to open file: {}", e)), 
	}, 
    }

}


