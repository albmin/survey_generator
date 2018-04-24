extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate survey_generator;

use survey_generator::options::*;
use std::env::args;
use std::env::Args;
use std::process;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::default::Default;
//use std::fmt::Description;

#[derive(Serialize, Deserialize)]
struct Survey {
    questions: Vec<String>,
}

enum SurveyError{
   FileReadError(Error),
   SurveyParseError(),
}


fn main() {
    match parse_options(args()) {
	Ok(o) =>  match survey_from_json(o) {
	    Ok(s) => generate_surveys(s),
	    Err(SurveyError::FileReadError(e)) => {
		println!("{}", e);
	  	process::exit(1);
	    },
	    Err(SurveyError::SurveyParseError()) => {
		println!("Error occurred during file parsing");
		process::exit(1);
	    },
	},
	Err(e) =>  {
	    println!("{}", e);
	    process::exit(1);	
	},
    };
}

//move this and survey into a library and change this parser into a constructor
///parses the json into a survey struct provided a json file
fn survey_from_json(mut file: File) -> Result<Survey, SurveyError>  {	
	//read contents of file
	let mut content = Default::default();
	match file.read_to_string(&mut content) {
	    Ok(_) => (),	
	    Err(e) => return Err(SurveyError::FileReadError(e)), //todo sort out the error type
	}

	match serde_json::from_str(&content) {
		Ok(s) => return Ok(s),	
		Err(_) => return Err(SurveyError::SurveyParseError()),
	}			
}

fn generate_surveys(survey: Survey) {

}
