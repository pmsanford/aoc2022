use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use duct::cmd;
use git2::{Repository, RepositoryState, Status};
use toml_edit::{Document, Item, Value};
mod day;
use day::Day;

use crate::day::Part;

fn check_repo_dirty(repo: &Repository) -> Result<()> {
    if repo.state() != RepositoryState::Clean {
        bail!("Repo state isn't clean");
    }

    if repo
        .statuses(None)?
        .iter()
        .filter(|status| status.status() != Status::IGNORED)
        .count()
        > 0
    {
        bail!("Repo has uncommitted changes");
    }

    Ok(())
}

fn find_latest_day(path: &Path) -> Result<Option<Day>> {
    let mut max = None;
    for dir in fs::read_dir(path)? {
        let dir_name = dir?.file_name().to_string_lossy().to_string();
        if let Ok(day) = Day::try_from(dir_name) {
            if max.map(|max| max < day).unwrap_or(true) {
                max = Some(day);
            }
        }
    }

    Ok(max)
}

fn main() -> Result<()> {
    let repo = Repository::open_from_env()?;
    let repo_root = PathBuf::from(repo.workdir().unwrap());
    check_repo_dirty(&repo)?;
    let mut workspace_toml = repo_root.clone();
    workspace_toml.push("Cargo.toml");
    let cont = fs::read_to_string(&workspace_toml)?;
    let mut doc = cont.parse::<Document>()?;

    let workspace_members = &mut doc["workspace"]["members"];

    let workspace_members = workspace_members.as_array_mut().unwrap();

    let latest = find_latest_day(Path::new("."))?;
    let next = latest.as_ref().map(Day::next_day).unwrap_or_default();

    env::set_current_dir(&repo_root)?;
    workspace_members.push(next.to_string());

    fs::write(&workspace_toml, doc.to_string())?;

    let mut new_day = repo_root;

    new_day.push(next.to_string());

    if next.part == Part::One {
        cmd!("cargo", "new", next.to_string()).run()?;

        env::set_current_dir(&new_day)?;

        cmd!("cargo", "add", "anyhow").run()?;
        cmd!("cargo", "add", "--path", "../util").run()?;
    } else {
        cmd!("cp", "-R", latest.unwrap().to_string(), next.to_string()).run()?;

        new_day.push("Cargo.toml");

        let cont = fs::read_to_string(&new_day)?;
        let mut doc = cont.parse::<Document>()?;

        doc["package"]["name"] = Item::Value(Value::from(next.to_string()));
        fs::write(&new_day, doc.to_string())?;
    }

    Ok(())
}
