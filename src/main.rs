use async_std::task;
use std::process::Command;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[clap(name = "create", alias = "cr")]
    Create { branch: String },

    #[clap(name = "checkout", alias = "co")]
    Checkout { branch: String },

    #[clap(name = "pull")]
    Pull {},

    #[clap(name = "push")]
    Push { branch: String },

    #[clap(name = "rebase", alias = "reb")]
    Rebase {},

    #[clap(name = "delete", alias = "del")]
    Delete { branch: String },

    #[clap(name = "list", alias = "l")]
    List {},

    #[clap(name = "sync")]
    Sync {},
}

fn main() -> Result<()> {
    println!("Fit -- a git-like command line interface for your terminal easily and fastly");
    let args = Args::parse();

    match args.action {
        Action::Create { branch } => Exectuer::set_branch(branch)?,
        Action::Checkout { branch } => Exectuer::checkout(branch)?,
        Action::Delete { branch } => Exectuer::delete(branch)?,
        Action::Pull {} => Exectuer::pull()?,
        Action::Push { branch } => Exectuer::push(branch)?,
        Action::Rebase {} => Exectuer::rebase()?,
        Action::List {} => Exectuer::list()?,
        Action::Sync {} => Exectuer::sync()?,
    };
    Ok(())
}

pub struct Exectuer {}

impl Exectuer {
    fn set_branch(branch: String) -> Result<()> {
        task::block_on(async {
            Command::new("git")
                .arg("branch")
                .arg(branch.clone())
                .output()
                .expect("failed to execute process");
        });
        Exectuer::checkout(branch)?;
        Ok(())
    }

    fn checkout(branch: String) -> Result<()> {
        task::block_on(async {
            let output = Command::new("git")
                .arg("checkout")
                .arg(branch)
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });
        Ok(())
    }

    fn delete(branch: String) -> Result<()> {
        Exectuer::checkout("master".to_string())?;
        task::block_on(async {
            let output = Command::new("git")
                .arg("branch")
                .arg("-D")
                .arg(branch)
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });
        Ok(())
    }

    fn pull() -> Result<()> {
        // git pull upstream master
        task::block_on(async {
            let output = Command::new("git")
                .arg("pull")
                .arg("upstream")
                .arg("master")
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });
        Ok(())
    }

    fn push(branch: String) -> Result<()> {
        // git push --set-upstream origin new_branch(default master)
        task::block_on(async {
            let output = Command::new("git")
                .arg("push")
                .arg("--set-upstream")
                .arg("origin")
                .arg(branch)
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });
        Ok(())
    }

    fn rebase() -> Result<()> {
        // git pull upstream master --rebase
        task::block_on(async {
            let output = Command::new("git")
                .arg("pull")
                .arg("upstream")
                .arg("master")
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });
        Ok(())
    }

    fn list() -> Result<()> {
        // git branch -a
        task::block_on(async {
            let output = Command::new("git")
                .arg("branch")
                .arg("-a")
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        });
        Ok(())
    }

    // fit sync: sync fork rep
    // * git fetch upstream
    // * git checkout master
    // * git merge upstream/master
    fn sync() -> Result<()> {
        task::block_on(async {
            let output = Command::new("git")
                .arg("fetch")
                .arg("upstream")
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });

        Exectuer::checkout("master".to_string())?;

        task::block_on(async {
            let output = Command::new("git")
                .arg("merge")
                .arg("upstream/master")
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        });
        Ok(())
    }
}
