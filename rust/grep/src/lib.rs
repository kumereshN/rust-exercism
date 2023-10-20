use std::fs;
use anyhow::{anyhow, Error};
use regex::Regex;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags<'a>(Vec<&'a str>);

impl<'a> Flags<'a> {
    pub fn new(flags: &[&'a str]) -> Self {
        // Dummy placeholder text
        Self(flags.to_vec())
    }

    fn has(&self, flag: &str) -> bool {
        self.0.contains(&flag)
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    /*
1: Modify output (-n, -l)
2: Pattern matching (-i, -v, -x)
*/
    let mut results: Vec<String> = vec![];

    let flag = if flags.has("-i") { "(?i)" } else { "" };
    let match_entire_line = if flags.has("-x") { "^" } else { "" };
    let pattern = Regex::new(&format!("{}{}{}", flag, match_entire_line, pattern))?;
    let total_files = files.len();


    for file in files {
        let file_content = fs::read_to_string(file)?;

        for (num, line) in file_content.lines().enumerate() {
            let matches = pattern.is_match(line);

            if matches != flags.has("-v") {
                let mut result = String::new();

                if flags.has("-n") {
                    if total_files < 2 {
                        result.push_str(&format!("{}:", num+1));
                    } else {
                        result.push_str(&format!("{}:{}:", file, num+1));
                    }
                }

                if flags.has("-l") {
                    results.push(file.parse()?);
                    break;
                } else {
                    if (total_files < 2) | flags.has("-n") {
                        result.push_str(line);
                    } else {
                        result.push_str(&format!("{}:{}", file, line));
                    }
                    results.push(result);
                }
            }
        }
    }
    Ok(results)
}