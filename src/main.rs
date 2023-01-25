use std::{path::PathBuf, str::FromStr};
use anyhow::{Error, anyhow};
use clap::{self, Parser};
use merge_jsons::*;
use serde_json;
#[derive(clap::Parser)]
struct Args {
    action: Action,

    source: PathBuf,

    target: PathBuf,
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "patch" {
            return Ok(Action::Patch)
        } else if s == "diff" {
            return Ok(Action::Diff)
        }
        else {
            Err(anyhow!("No such command"))
        }
    }
}
#[derive(clap::Subcommand)]
enum Action {
    Patch,
    Diff,
}

fn main() {
    let cli = Args::parse();

    let source = std::fs::read_to_string(cli.source).unwrap();
    let mut source: serde_json::Value = serde_json::from_str(&source).unwrap();

    let target = std::fs::read_to_string(cli.target).unwrap();
    let target: serde_json::Value = serde_json::from_str(&target).unwrap();

    match cli.action {
        Action::Patch => {
            merge_json(&mut source, &target);
            println!("{}", serde_json::to_string_pretty(&source).unwrap())
        }
        Action::Diff => {
            // Since it's not proven that the Diff is always the inverse, panic if it's not
            let patch = create_patch(source.clone(), target.clone()).unwrap();
            merge_json(&mut source, &patch);
            assert_eq!(source, target, "The patched source with the generated patch does not equal the target.");
            println!("{}", serde_json::to_string_pretty(&patch).unwrap())
        }
    }
}
