use std::fs;
use anyhow::Error;

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
pub struct Flags {
    is_case_insensitive: bool,
    is_inverted: bool,
    is_match_entire_lines: bool,
    is_output_line_numbers: bool,
    is_output_name_of_file: bool
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            is_case_insensitive: flags.contains(&"-i"),
            is_inverted: flags.contains(&"-v"),
            is_match_entire_lines: flags.contains(&"-x"),
            is_output_line_numbers: flags.contains(&"-n"),
            is_output_name_of_file: flags.contains(&"-l")
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    /*
    1: Modify output (-n, -l)
    2: Pattern matching (-i, -v, -x)
    */
    let mut res = vec![];
    let has_multiple_files = files.len() > 1;

    for file in files {
        let file_content = fs::read_to_string(file)?;
        let mut lines: Vec<_> = file_content
            .lines()
            .enumerate()
            .filter(|(_,line)|{
                match (flags.is_case_insensitive, flags.is_match_entire_lines, flags.is_inverted) {
                    (true, false, false) => line.to_lowercase().contains(&pattern.to_lowercase()),
                    (false, true, false) => line == &pattern,
                    (true, true, false) => line.to_lowercase() == pattern.to_lowercase(),
                    (false, false, true) => !line.contains(pattern),
                    (false, true, true) => line != &pattern,
                    _ => line.contains(pattern),
                }
            })
            .map(|(i, line)| {
                match (flags.is_output_line_numbers, has_multiple_files) {
                    (true, true) => format!("{}:{}:{}", file, i+1, line),
                    (true, false) => format!("{}:{}", i+1, line),
                    (false, true) => format!("{}:{}", file, line),
                    (false, false) => line.to_string(),
                }
            })
            .collect();

        if !lines.is_empty() && flags.is_output_name_of_file {
            res.push(file.to_string())
        } else {
            res.append(&mut lines)
        }
    }

    Ok(res)
}