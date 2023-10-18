use std::fs;
use anyhow::{anyhow, Error};

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
        /*        todo!(
            "Given the flags {flags:?} implement your own 'Flags' struct to handle flags-related logic"
        );*/

    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res: Vec<Vec<String>> = vec![];

    for file in files {
        let file_content = fs::read_to_string(file)?;
        let flag_vec = &flags.0.clone();

        if flag_vec.is_empty() {
            res.push(file_content
                .lines()
                .filter_map(|c| {
                    if c.contains(pattern) {
                        Some(c.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
            );
        }

        for &flag in flag_vec {
            match flag {
                "-n" => {
                    res.push(file_content
                        .lines()
                        .enumerate()
                        .filter_map(|(i, c)| {
                            if c.contains(pattern) {
                                Some(format!("{}:{}", i+1, c))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                    )
                },
                "-l" => {
                    if file_content.contains(pattern) {
                        res.push(vec![file.to_string()])
                    }
                },
                "-i" => {
                    res.push(
                        file_content
                            .lines()
                            .filter_map(|c| {
                                if c.to_ascii_lowercase().contains(&pattern.to_ascii_lowercase()) {
                                    Some(c.to_string())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<String>>()
                    )
                },
                "-v" => {},
                "-x" => {
                    res.push(
                    file_content
                        .lines()
                        .filter_map(|c| {
                            if c == pattern {
                                Some(c.to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                    )
                },
                _ => {
                    panic!("Something went wrong")
                }
            }
        }
    }

    Ok(res.into_iter().flatten().collect::<Vec<String>>())
/*    todo!(
        "Search the files '{files:?}' for '{pattern}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{flags:?}'"
    );*/
}
