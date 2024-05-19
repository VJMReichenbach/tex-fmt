use clap::Parser;

const EXTENSIONS: [&str; 4] = [".tex", ".bib", ".sty", ".cls"];

#[derive(Parser)]
pub struct Cli {
    #[arg(long, short, help = "Indent only, do not modify line breaks")]
    pub indent: bool,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: bool,
    #[arg(long, short, help = "Increase verbosity")]
    pub verbose: bool,
    #[arg(long, short, help = "Debug, do not modify files")]
    pub debug: bool,
    #[arg(required = true)]
    pub filenames: Vec<String>,
}

impl Cli {
    #[cfg(test)]
    pub fn new() -> Self {
        Cli {
            indent: false,
            print: false,
            verbose: false,
            debug: false,
            filenames: Vec::<String>::new(),
        }
    }
}

pub fn check_extension_valid(filename: &str) -> bool {
    EXTENSIONS.iter().any(|e| filename.ends_with(e))
}
