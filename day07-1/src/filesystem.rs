use std::collections::HashMap;

use anyhow::{bail, Result};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Directory {
    id: usize,
    parent: usize,
    name: String,
    files: HashMap<String, usize>,
    subdirs: HashMap<String, usize>,
}

impl Directory {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct File {
    id: usize,
    parent: usize,
    name: String,
    size: usize,
}

impl File {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct Filesystem {
    directories: Vec<Directory>,
    files: Vec<File>,
    pwd: usize,
}

pub enum Entry {
    Directory(Directory),
    File(File),
}

impl Entry {
    pub fn name(&self) -> String {
        match self {
            Entry::Directory(dir) => dir.name.clone(),
            Entry::File(file) => file.name.clone(),
        }
    }
}

impl Filesystem {
    pub fn new() -> Self {
        let mut fs = Self {
            directories: vec![],
            files: vec![],
            pwd: 0,
        };

        fs.directories.push(Directory {
            id: 0,
            parent: 0,
            name: "/".into(),
            files: HashMap::new(),
            subdirs: HashMap::new(),
        });

        fs
    }

    pub fn root(&self) -> Directory {
        self.directories[0].clone()
    }

    fn current_dir(&self) -> Directory {
        self.directories[self.pwd].clone()
    }

    pub fn ls_dir(&self, dir: &Directory) -> Vec<Entry> {
        let mut subdirs = dir
            .subdirs
            .values()
            .map(|id| self.directories[*id].clone())
            .map(Entry::Directory)
            .collect::<Vec<_>>();
        let mut files = dir
            .files
            .values()
            .map(|id| self.files[*id].clone())
            .map(Entry::File)
            .collect::<Vec<_>>();

        subdirs.append(&mut files);

        subdirs
    }

    pub fn cd(&mut self, newdir: String) -> Result<()> {
        let dir = self.current_dir();
        if newdir == "/" {
            self.pwd = 0;
        } else if newdir == ".." {
            self.pwd = dir.parent;
        } else if let Some(newdir) = dir.subdirs.get(&newdir) {
            self.pwd = *newdir;
        } else {
            bail!("No such directory");
        }

        Ok(())
    }

    pub fn add_file(&mut self, name: String, size: usize) {
        let id = self.files.len();
        self.files.push(File {
            id,
            parent: self.pwd,
            name: name.clone(),
            size,
        });

        self.directories[self.pwd].files.insert(name, id);
    }

    pub fn add_dir(&mut self, name: String) {
        let id = self.directories.len();
        self.directories.push(Directory {
            id,
            parent: self.pwd,
            name: name.clone(),
            files: HashMap::new(),
            subdirs: HashMap::new(),
        });

        self.directories[self.pwd].subdirs.insert(name, id);
    }
}
