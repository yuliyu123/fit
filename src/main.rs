use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
    Push {},

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
        Action::Push {} => Exectuer::push()?,
        Action::Rebase {} => Exectuer::rebase()?,
        Action::List {} => Exectuer::list()?,
        Action::Sync {} => Exectuer::sync()?,
    };
    Ok(())
}

const INIT_CFG_DIR: &'static str = ".fit";
const INIT_CFG: &'static str = ".fit/config.json";

pub fn save(cfg: &Exectuer) -> Result<()> {
    if !Path::new(INIT_CFG_DIR).exists() {
        fs::create_dir(INIT_CFG_DIR)?;
    }

    let json = serde_json::to_string_pretty(cfg);
    let mut file = File::create(Path::new(INIT_CFG))?;
    file.write_all(json?.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<String> {
    let file_str = fs::read_to_string(INIT_CFG)?;
    let cfg: Exectuer = serde_json::from_str(&file_str)?;
    Ok(cfg.branch)
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Exectuer {
    branch: String,
}

impl Exectuer {
    fn set_branch(branch: String) -> Result<()> {
        save(&Exectuer {
            branch: branch.clone(),
        })?;
        Command::new("git")
            .arg("branch")
            .arg(branch)
            .spawn()
            .expect("failed to execute process");
        // Exectuer::checkout(self.branch)?;
        Ok(())
    }

    fn checkout(branch: String) -> Result<()> {
        Command::new("git")
            .arg("checkout")
            .arg(branch)
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }

    fn delete(branch: String) -> Result<()> {
        // Exectuer::checkout("master".to_string())?;
        Command::new("git")
            .arg("branch")
            .arg("-D")
            .arg(branch)
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }

    fn pull() -> Result<()> {
        // git pull upstream master
        Command::new("git")
            .arg("pull")
            .arg("upstream")
            .arg("master")
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }

    fn push() -> Result<()> {
        // git push --set-upstream origin new_branch
        Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(load()?)
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }

    fn rebase() -> Result<()> {
        // git pull upstream master --rebase
        Command::new("git")
            .arg("pull")
            .arg("upstream")
            .arg("master")
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }

    fn list() -> Result<()> {
        // git branch -a
        Command::new("git")
            .arg("branch")
            .arg("-a")
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }

    // fit sync: sync fork rep
    // * git fetch upstream
    // * git checkout master
    // * git merge upstream/master
    fn sync() -> Result<()> {
        Command::new("git")
            .arg("fetch")
            .arg("upstream")
            .spawn()
            .expect("failed to execute process");

        // Exectuer::checkout("master".to_string())?;

        Command::new("git")
            .arg("merge")
            .arg("upstream/master")
            .spawn()
            .expect("failed to execute process");
        Ok(())
    }
}
