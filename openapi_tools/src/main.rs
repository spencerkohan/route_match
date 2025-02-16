use clap::Parser;
use openapi_tools::{Args, Subcommand, merge};

fn main() {
    let args = Args::parse();

    match args.command {
        Subcommand::Merge(merge) => {
            if let Some(result) = merge::exec(merge) {
                println!("{}", result);
            }
        }
    }
}
