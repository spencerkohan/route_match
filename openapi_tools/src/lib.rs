use std::{path::PathBuf, str::FromStr};

use clap::{Parser, ValueEnum};

pub mod merge;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub command: Subcommand,
}
#[derive(clap::Subcommand, Debug, Clone)]
pub enum Subcommand {
    Merge(MergeArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct MergeArgs {
    /// The filename of the main template file
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// The template as json
    #[arg(short, long)]
    pub json: Option<String>,

    /// The template  as yaml
    #[arg(short, long)]
    pub yaml: Option<String>,

    /// The template encoding (defaults to the file extension)
    #[arg(short, long)]
    pub encoding: Option<Encoding>,

    /// Template variables
    #[arg(long = "var", short = 'v')]
    pub template_vars: Vec<TemplateVar>,

    /// Output file path (if not specified, the result will print to stdout)
    #[arg(short = 'o', long = "output-file")]
    pub output: Option<PathBuf>,

    /// Output encoding (default is json)
    #[arg(long = "output-encoding", short = 'E')]
    pub output_format: Option<Encoding>,

    /// Paths in the template will be resolved relative to this directory (defaults to cwd)
    #[arg(long = "working-dir", short = 'w')]
    pub working_directory: Option<PathBuf>,

    /// Enable verbose logging output
    #[arg(long = "verbose", short = 'V', action)]
    pub verbose: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Encoding {
    Json,
    Yaml,
    Yml,
}

#[derive(Debug, Clone)]
pub struct TemplateVar {
    pub key: String,
    pub value: String,
}

impl FromStr for TemplateVar {
    type Err = String;

    /// Parse a string of the form `key=value` into a `Var`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into two parts using '=' as delimiter.
        let mut parts = s.splitn(2, '=');
        let key = parts
            .next()
            .ok_or_else(|| format!("Missing key in '{}'", s))?;
        let value = parts
            .next()
            .ok_or_else(|| format!("Missing value in '{}'. Expected format: key=value", s))?;
        Ok(Self {
            key: key.to_string(),
            value: value.to_string(),
        })
    }
}

impl MergeArgs {
    pub fn working_dir(&self) -> PathBuf {
        if let Some(path) = &self.working_directory {
            return path.clone();
        }
        std::env::current_dir().unwrap()
    }

    pub fn relative_path(&self, path: &PathBuf) -> PathBuf {
        if path.is_absolute() {
            return path.clone();
        }
        self.working_dir().join(path)
    }
}
