extern crate clap;
use clap::{Arg, App, SubCommand};

mod tokenizer;
mod parser;
mod repl;
use repl::Repl;

fn main() {
     let app = App::new("yvi-lang compiler")
         .version("0.1.0")
         .author("Daniel Corn <support@cundd.net>")
         .about("Compile yv files")
         .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
         .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
         .subcommand(SubCommand::with_name("test")
             .about("controls testing features")
             .arg(Arg::with_name("debug")
                 .short("d")
                 .help("print debug information verbosely")));

     // Parse the command line arguments
     let matches = app.get_matches();

     let config = matches.value_of("config").unwrap_or("default.conf");
     let input = matches.value_of("INPUT").unwrap();


    Repl::run();
}
