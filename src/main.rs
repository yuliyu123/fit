use std::process::{Command, Output};

use anyhow::{Context, Result};
use clap::{IntoApp, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[clap(name = "create")]
    Create { branch: String },

    #[clap(name = "checkout")]
    Checkout { branch: String },
    // #[clap(name = "pull", default_value = "")]
    // pull: String,

    // #[clap(name = "push", default_value = "")]
    // push: String,

    // #[clap(name = "rebase", default_value = "")]
    // rebase: String,

    // #[clap(name = "delete", default_value = "")]
    // delete: String,
}

fn main() -> Result<()> {
    println!("Fit -- a git-like command line interface for your terminal easily and fastly");
    // let args = Args::parse();
    // let exectuer = Exectuer::load()?;
    // match args.action {
    //     Action::Create { branch } => exectuer.set_branch(branch),
    //     Action::Checkout { branch } => exectuer.checkout(branch),
    // };
    let output = Command::new("git")
            .arg("branch")
            .arg(" sssbranch")
            .output()
            .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

const STORAGE: &'static str = "registry";
const CREATE: &'static str = "git branch ";

struct Exectuer {
    branch: String,
}

impl Exectuer {
    fn load() -> Result<Self> {
        let branch = confy::load(STORAGE).context("Error loading alias registry")?;
        Ok(Exectuer { branch })
    }

    fn set_branch(mut self, branch: String) -> Result<()> {
        println!("Setting branch to {}", branch);
        confy::store(STORAGE, branch.clone())?;
        let output = Command::new("git")
            .arg("branch")
            .arg(" sssbranch")
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    fn checkout(mut self, branch: String) -> Result<()> {
        self.branch = branch;
        confy::store(STORAGE, self.branch)?;
        Ok(())
    }
}
