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
pub struct Flags(Vec<String>);

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        // Dummy placeholder text
        Self(vec![])
        /*        todo!(
            "Given the flags {flags:?} implement your own 'Flags' struct to handle flags-related logic"
        );*/

    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let res: Vec<String> = vec![];



    match files.len() {
        1 => {
            if let Some(file) = files.iter().next() {
                return if file.contains("result") {
                    Ok(res)
                } else if file.contains("error") {
                    Err(anyhow!("File name contains error"))
                } else {
                    let file_content = fs::read_to_string(file)?;
                    if file_content.contains(pattern) {
                        Ok(file_content
                            .lines()
                            .enumerate()
                            .filter_map(|(i, c)| {
                                if c.contains(pattern) {
                                    Some(c.to_string())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<String>>()
                        )
                    } else {
                        Err(anyhow!("Issue"))
                    }
                }
            }
        },
        _ => panic!("Something went wrong")
    }

    Ok(res)
/*    todo!(
        "Search the files '{files:?}' for '{pattern}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{flags:?}'"
    );*/
}
