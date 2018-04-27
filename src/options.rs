use clap::{Arg, App};
use std::path::Path;

struct Options {
    survey_count: u64,
    output_dir: &Path, 	    
}


fn parse_options() -> &Options {
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
				.index(1))
			  .get_matches();

    let size = &mut value_t_or_exit!(matches.value_of("survey_count"), usize);
    let o = &mut Path::new(matches.value_of("output_dir").unwrap()); 	
    
    Options { size, o}
}

