//Import the crates that I used, clap for CLI and colored for colored output
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

//Using clap to have cli input
#[derive(Parser)]
struct Cli {
    /// The pattern that will be checked for. This is interpreted as a regex, so be careful with special characters
    pattern: String,
    /// The file that will be checked.
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
        let higlighted_text = format!(
            "{}{}{}",
            // the part before the match
            &(self.text)[0..self.from],
            // the part that matches, in blue and bold
            format!("{}", &(self.text)[self.from..self.to + 1])
                .bold()
                .blue(),
            // the remainder of the line
            &(self.text)[self.to + 1..]
        );

        // actually write the output.
        write!(
            f,
            "l{};c{}: {}",
            format!("{}", self.linecount).green(),
            format!("{}", self.from).green(),
            higlighted_text
        )
    }
}

fn main() {
    //parses the command line args
    let args = Cli::parse();

    let re: Regex = Regex::new(&args.pattern).unwrap();
    // let re: Regex = Regex::new(r"version").unwrap();

    //opens the file specified in the args and creates a buffered reader for less memory intensive reading of lines
    let f = File::open(&args.path).unwrap();
    let reader = BufReader::new(f);

    //a counter for the line counter output
    let mut count = 0;

    //loops through all lines in a file
    for line in reader.lines() {
        count = count + 1;
        match line {
            // if we were able to read the line
            Ok(text) => {
                // the text as str, for the regex
                let text_as_str = text.as_str();

                // get the matches in this str
                let matches = re.find_iter(&text_as_str);

                // for each found match we output it
                for found_match in matches {
                    let line_out = Out {
                        linecount: count,
                        from: found_match.start(),
                        to: found_match.end()-1,
                        text: text_as_str.to_string(),
                    };
                    println!("{}", line_out);
                }

            }
            // Handle any errors.
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}
