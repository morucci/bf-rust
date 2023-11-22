use std::{fs, path};

#[derive(Debug)]
pub struct FileEntity {
    path: path::PathBuf,
    size: u64,
}

#[derive(Debug)]
pub struct DirEntity {
    files: Vec<FileEntity>,
    dirs: Vec<path::PathBuf>,
}

pub fn get_dir_entity(path: path::PathBuf) -> Option<DirEntity> {
    match fs::read_dir(path) {
        Ok(i) => {
            let mut files: Vec<FileEntity> = Vec::new();
            let mut dirs: Vec<path::PathBuf> = Vec::new();
            let _: Vec<_> = i
                .map(|r| match r {
                    Ok(dir_entry) => {
                        if let Ok(ft) = dir_entry.file_type() {
                            if ft.is_file() {
                                files.push(FileEntity {
                                    path: dir_entry.path(),
                                    size: dir_entry.metadata().unwrap().len(),
                                })
                            } else if ft.is_dir() {
                                dirs.push(dir_entry.path())
                            }
                        }
                    }
                    Err(_) => (),
                })
                .collect();
            Some(DirEntity { files, dirs })
        }
        Err(_) => None,
    }
}

pub fn walk_dir_entity(path: path::PathBuf) -> Vec<FileEntity> {
    let mut files: Vec<FileEntity> = Vec::new();
    let mut dirs_to_process: Vec<path::PathBuf> = vec![path];
    loop {
        match dirs_to_process.pop() {
            Some(current_dir) => match get_dir_entity(current_dir) {
                Some(mut dir_entity) => {
                    files.append(&mut dir_entity.files);
                    dirs_to_process.append(&mut dir_entity.dirs);
                }
                None => (),
            },
            None => break,
        }
    }
    files
}
