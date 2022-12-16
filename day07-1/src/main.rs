mod filesystem;

use std::collections::HashMap;

use anyhow::{bail, Result};
use filesystem::{Directory, Filesystem};
use util::Input;

fn show_tree(fs: &Filesystem) {
    let root = fs.root();
    print_dir(fs, &root, 0);
}

fn print_dir(fs: &Filesystem, pwd: &Directory, depth: usize) {
    println!("{}- {} (dir)", " ".repeat(depth * 2), pwd.name());
    let entries = fs.ls_dir(pwd);
    let entries = entries
        .into_iter()
        .map(|entry| (entry.name(), entry))
        .collect::<HashMap<_, _>>();

    let mut alpha_keys = entries.keys().cloned().collect::<Vec<_>>();
    alpha_keys.sort();

    for key in alpha_keys {
        match &entries[&key] {
            filesystem::Entry::Directory(dir) => print_dir(fs, dir, depth + 1),
            filesystem::Entry::File(file) => {
                println!(
                    "{}- {} (file, size={})",
                    " ".repeat((depth + 1) * 2),
                    file.name(),
                    file.size()
                );
            }
        }
    }
}

fn dir_sizes(fs: &Filesystem) -> HashMap<String, usize> {
    let root = fs.root();
    let mut sizes = HashMap::new();
    walk_dirs(fs, &root, &mut sizes, "".into());
    sizes
}

fn walk_dirs(
    fs: &Filesystem,
    pwd: &Directory,
    sizes: &mut HashMap<String, usize>,
    path: String,
) -> usize {
    let entries = fs.ls_dir(pwd);
    let mut size = 0;
    let path = if path.is_empty() {
        "/".into()
    } else if path == "/" {
        format!("/{}", pwd.name())
    } else {
        format!("{}/{}", path, pwd.name())
    };
    for entry in entries {
        size += match entry {
            filesystem::Entry::Directory(dir) => walk_dirs(fs, &dir, sizes, path.clone()),
            filesystem::Entry::File(file) => file.size(),
        }
    }
    sizes.insert(path, size);

    size
}

fn main() -> Result<()> {
    let input = Input::new()?.into_lines()?;

    let mut fs = Filesystem::new();

    for line in input {
        match line.as_str() {
            s if s.starts_with("$ cd ") => {
                let dirname = s[5..].to_owned();
                println!("Changing to dir '{dirname}'");
                fs.cd(dirname)?;
            }
            s if s == "$ ls" => {}
            s if s.starts_with("dir ") => {
                let dirname = s[4..].to_owned();
                println!("Adding dir '{dirname}'");
                fs.add_dir(dirname);
            }
            s if s
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false) =>
            {
                let (size, name) = s.split_once(' ').unwrap();
                let size: usize = size.parse()?;
                println!("Adding file '{name}' with size {size}");
                fs.add_file(name.to_owned(), size);
            }
            s if s.trim() == "" => {}
            _ => bail!("I don't know what to do with {}", line),
        }
    }

    show_tree(&fs);

    let sizes = dir_sizes(&fs);

    println!("Sizes: \n{sizes:?}");

    let total: usize = sizes.into_values().filter(|v| *v <= 100_000).sum();

    println!("Total: {total}");

    Ok(())
}
