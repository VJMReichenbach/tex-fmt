//! Utilities for reading the command line arguments

use crate::logging::*;
use crate::regexes::*;
use clap::Parser;
use log::Level::{Error, Trace};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;

/// Command line arguments
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Parser, Serialize, Deserialize, Clone)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, help = "Check formatting, do not modify files")]
    pub check: bool,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: bool,
    #[arg(long, short, help = "Keep lines, do not wrap")]
    pub keep: bool,
    #[arg(long, short, help = "Show info log messages")]
    pub verbose: bool,
    #[arg(long, short, help = "Hide warning messages")]
    pub quiet: bool,
    #[arg(long, short, help = "Show trace log messages")]
    pub trace: bool,
    #[arg(help = "List of files to be formatted")]
    pub files: Vec<String>,
    #[arg(
        long,
        short,
        help = "Process STDIN as a single file, output formatted text to STDOUT"
    )]
    pub stdin: bool,
    #[arg(
        long,
        help = "Number of spaces to use as tab size",
        default_value_t = 2
    )]
    pub tab: i8,
    #[arg(long, help = "Use tabs instead of spaces for indentation")]
    pub usetabs: bool,
    #[arg(long, help = "Line length for wrapping", default_value_t = 80)]
    pub wrap: u8,
    #[clap(skip)]
    pub wrap_min: u8,
}

impl Cli {
    /// Get the log level
    pub const fn log_level(&self) -> LevelFilter {
        if self.trace {
            LevelFilter::Trace
        } else if self.verbose {
            LevelFilter::Info
        } else if self.quiet {
            LevelFilter::Error
        } else {
            LevelFilter::Warn
        }
    }

    /// Ensure the provided arguments are consistent
    pub fn resolve(&mut self, logs: &mut Vec<Log>) -> u8 {
        let mut exit_code = 0;
        self.verbose |= self.trace;
        self.print |= self.stdin;
        self.wrap_min = if self.wrap >= 50 {
            self.wrap - 10
        } else {
            self.wrap
        };

        if !self.stdin && self.files.is_empty() {
            record_file_log(
                logs,
                Error,
                "",
                "No files specified. Either provide filenames or provide --stdin.",
            );
            exit_code = 1;
        }
        if self.stdin && !self.files.is_empty() {
            record_file_log(
                logs,
                Error,
                "",
                "Do not provide file name(s) when using --stdin.",
            );
            exit_code = 1;
        }
        exit_code
    }

    #[cfg(test)]
    pub const fn new() -> Self {
        Self {
            check: false,
            print: false,
            keep: false,
            verbose: false,
            stdin: false,
            quiet: false,
            trace: false,
            files: Vec::<String>::new(),
            tab: 2,
            usetabs: false,
            wrap: 80,
            wrap_min: 70,
        }
    }
}

/// Add a missing extension and read the file
pub fn read(file: &str, logs: &mut Vec<Log>) -> Option<(String, String)> {
    // check if file has an accepted extension
    let has_ext = EXTENSIONS.iter().any(|e| file.ends_with(e));
    // if no valid extension, try adding .tex
    let mut new_file = file.to_owned();
    if !has_ext {
        new_file.push_str(".tex");
    };
    if let Ok(text) = fs::read_to_string(&new_file) {
        return Some((new_file, text));
    }
    if has_ext {
        record_file_log(logs, Error, file, "Could not open file.");
    } else {
        record_file_log(logs, Error, file, "File type invalid.");
    }
    None
}

/// Attempt to read from STDIN, return filename `<STDIN>` and text
pub fn read_stdin(logs: &mut Vec<Log>) -> Option<(String, String)> {
    let mut text = String::new();
    match std::io::stdin().read_to_string(&mut text) {
        Ok(bytes) => {
            record_file_log(
                logs,
                Trace,
                "<STDIN>",
                &format!("Read {bytes} bytes."),
            );
            Some((String::from("<STDIN>"), text))
        }
        Err(e) => {
            record_file_log(
                logs,
                Error,
                "<STDIN>",
                &format!("Could not read from STDIN: {e}"),
            );
            None
        }
    }
}
