// Import the crates that I used, clap for CLI and colored for colored output
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::fmt;
use std::io::stdin;
use std::io::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Using clap to have cli input

/// A simple grep-clone that is written in
/// Rust. I made this, because I wanted to 
/// learn Rust. This has the same basic
/// functionality as grep, but it shows the
/// line and column on which a match was found
/// and colors it in the output.
#[derive(Parser)]
struct Cli {
    /// The pattern that will be checked for.
    /// This is interpreted as a regex, so be
    /// careful with special characters
    pattern: String,
    /// The file that will be checked.
    path: Option<std::path::PathBuf>,
}

// A helper struct for outputting lines
struct Out {
    linecount: i32,
    from: usize,
    to: usize,
    text: String,
}

// Implementation of Display for the Out struct, 
// so I can easily print it with println!("{}", Out)
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

fn main() -> Result<()> {

    // parses the command line args
    let args = Cli::parse();

    let regex: Regex = Regex::new(&args.pattern).unwrap();
    // let re: Regex = Regex::new(r"version").unwrap();

    // Check whether the user provided a file.
    // if not we will use the standard input
    if let Some(path) = &args.path {
        match File::open(path) {
            Ok(f) => {
                // read the file that was specified
                let reader = BufReader::new(f);

                // counts on which line we are
                let mut count = 0;
                for line in reader.lines() {
                    match line {
                        Ok(line_string) => {
                            // if we were able to read the line
                            // try to find matches and output them
                            match_and_output(&regex, line_string.as_str(), count);
                        }
                        Err(_) => {}
                    }
                    // increase the line count
                    count += 1;
                }
            }
            Err(_) => {
                println!("Something went wrong while trying to open the file");
            }
        }
    } else {
        // read from the standard input
        let stdin = stdin();
        let handle = stdin.lock();

        // counts on which line we are
        let mut count = 0;
        for line in handle.lines() {
            match line {
                Ok(line_string) => {
                    // if we were able to read the line
                    // try to find matches and output them
                    match_and_output(&regex, line_string.as_str(), count);
                }
                Err(_) => {}
            }
            // increase the line count
            count += 1;
        }
    }
    Ok(())
}

fn match_and_output(regex: &Regex, line: &str, line_count: i32) {
    let matches = regex.find_iter(&line);

    // for each found match we output it
    for found_match in matches {
        let line_out = Out {
            linecount: line_count,
            from: found_match.start(),
            to: found_match.end()-1,
            text: line.to_string(),
        };
        println!("{}", line_out);
    }
}
