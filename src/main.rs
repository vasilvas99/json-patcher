use clap::{Parser, Subcommand};
use merge_jsons::*;
use serde_json;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}
#[derive(Subcommand)]
enum Action {
    /// Apply a patch to a template
    Patch {
        /// The path to the source template that will be patched
        template: PathBuf,
        /// The path to the patch
        patch: PathBuf,
    },
    /// Generate a patch from a template and a "target"
    Diff {
        /// The path to the template that will be "filled in" with the generated patch
        source: PathBuf,
        /// The path to the target that will be obtained after applying the generated patch to the template
        target: PathBuf,
    },
}

fn main() {
    let cli = Args::parse();

    match cli.action {
        Action::Patch { template, patch } => {
            let template = std::fs::read_to_string(template).unwrap();
            let mut template: serde_json::Value = serde_json::from_str(&template).unwrap();

            let patch = std::fs::read_to_string(patch).unwrap();
            let patch: serde_json::Value = serde_json::from_str(&patch).unwrap();

            merge_json(&mut template, &patch);
            println!("{}", serde_json::to_string_pretty(&template).unwrap())
        }
        Action::Diff { source, target } => {
            let source = std::fs::read_to_string(source).unwrap();
            let mut source: serde_json::Value = serde_json::from_str(&source).unwrap();

            let target = std::fs::read_to_string(target).unwrap();
            let target: serde_json::Value = serde_json::from_str(&target).unwrap();

            // Since it's not proven that the Diff is always the inverse, panic if it's not
            let patch = create_patch(source.clone(), target.clone()).unwrap();
            merge_json(&mut source, &patch);
            assert_eq!(
                source, target,
                "The patched source with the generated patch does not equal the target."
            );
            println!("{}", serde_json::to_string_pretty(&patch).unwrap())
        }
    }
}
