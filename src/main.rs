//Import the crates that I used, clap for CLI and colored for colored output
use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;
use colored::Colorize;

//Using clap to have cli input
#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

//A helper struct for outputting lines
struct Out {
    linecount: i32,
    from: usize,
    to: usize,
    text: String,
}

//Implementation of Display for the Out struct, so I can easily print it with println!("{}", Out)
impl fmt::Display for Out {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let higlighted_text = format!("{}{}{}", 
            &(self.text)[0 .. self.from],
            format!("{}", &(self.text)[self.from .. self.to+1]).bold().red(),
            &(self.text)[self.to+1..]);
        write!(f, "l{};c{}: {}", format!("{}", self.linecount).green(), format!("{}", self.from).green(), higlighted_text)
    }
}

fn main() {
    //parses the command line args
    let args = Cli::parse();

    //opens the file specified in the args and creates a buffered reader for less memory intensive reading of lines
    let f = File::open(&args.path).unwrap();
    let reader = BufReader::new(f);

    //a counter for the line counter output
    let mut count = 0;

    //loops through all lines in a file
    for line in reader.lines() {
        let text = line.unwrap();
        count = count + 1;
        if text.contains(&args.pattern) {
            let index = text.find(&args.pattern);
            let line_out = Out {
                linecount: count,
                from: index.unwrap(),
                to: index.unwrap() + &args.pattern.len() - 1,
                text: text,
            };
            println!("{}", line_out);
        }
    }
}
