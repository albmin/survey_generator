use std::env::args_os;
use std::process;
use std::path::Path;
//use std::ffi::OsStr

fn main() {
    parseargs(env::args().collect);
    
    println!("Hello, world!");
}

//parses cli args and returns a file path to the json
//input file. could probably use a third party 
//party package but purposefully using the stdlib
fn parseFilePath(Vec<_> args) -> &'static Path {
    //only expect one arg 
    if args.len() != 1  {
        println!("No command line arguments specified! Requires filepath");
        process::exit(1);
    }   

    let strPath = String::from(args[0]);
    let path =  Path::new(&strPath);

    //check if file exists
    if !path.is_file() {
        println!("File does not exist! Requires existent file");
        process::exit(1);
    }

    return *path;
}
