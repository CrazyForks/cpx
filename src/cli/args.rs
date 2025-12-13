use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CLIArgs {
    #[arg(required = true)]
    pub sources: Vec<PathBuf>,

    /// Destination path (used without -t/--target-directory)
    #[arg(last = true)]
    pub destination: Option<PathBuf>,

    #[arg(
        short = 't',
        long = "target-directory",
        value_name = "DIRECTORY",
        conflicts_with = "destination",
        help = "copy all SOURCE arguments into DIRECTORY"
    )]
    pub target_directory: Option<PathBuf>,

    #[arg(short, long, help = "Progress bar style: default, minimal, detailed")]
    pub style: Option<String>,

    #[arg(short, long, help = "Copy directories recursively")]
    pub recursive: bool,

    #[arg(
        short = 'j',
        default_value_t = 4,
        help = "Number of concurrent copy operations for multiple files"
    )]
    pub concurrency: usize,

    #[arg(
        short = 'c',
        long = "continue",
        help = "Continue copying by skipping files that are already complete"
    )]
    pub continue_copy: bool,

    #[arg(
        short = 'f',
        long,
        help = "if an existing destination file cannot be opened, remove it and try again"
    )]
    pub force: bool,

    #[arg(short = 'i', long, help = "prompt before overwrite")]
    pub interactive: bool,

    #[arg(long, help = "use full source file name under DIRECTORY")]
    pub parents: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct CopyOptions {
    pub recursive: bool,
    pub concurrency: usize,
    pub resume: bool,
    pub force: bool,
    pub interactive: bool,
    pub parents: bool,
}

impl From<&CLIArgs> for CopyOptions {
    fn from(cli: &CLIArgs) -> Self {
        Self {
            recursive: cli.recursive,
            concurrency: cli.concurrency,
            resume: cli.continue_copy,
            force: cli.force,
            interactive: cli.interactive,
            parents: cli.parents,
        }
    }
}

impl CLIArgs {
    pub fn validate(self) -> Result<(Vec<PathBuf>, PathBuf, CopyOptions), String> {
        let options = CopyOptions::from(&self);

        let destination = if let Some(target) = self.target_directory {
            target
        } else if let Some(dest) = self.destination {
            dest
        } else {
            return Err(
                "Missing destination: specify last argument or use --target-directory".to_string(),
            );
        };

        Ok((self.sources, destination, options))
    }
}
