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
        // format the highlighted text
        let higlighted_text = format!("{}{}{}",
            // the part before the match 
            &(self.text)[0 .. self.from],
            // the part that matches, in blue and bold
            format!("{}", &(self.text)[self.from .. self.to+1]).bold().blue(),
            // the remainder of the line
            &(self.text)[self.to+1..]);

        // actually write the output.
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
        match line {
            Ok(text) => {
                count = count + 1;
                if let Some(index) = text.find(&args.pattern) {
                    let line_out = Out {
                        linecount: count,
                        from: index,
                        to: index + &args.pattern.len() - 1,
                        text
                    };
                    println!("{}", line_out);
                }
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}
