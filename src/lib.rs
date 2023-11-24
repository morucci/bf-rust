use std::{fs, path};

const TOP: usize = 10;
#[derive(Debug)]
pub struct FileEntity {
    path: path::PathBuf,
    size: u64,
}

#[derive(Debug)]
struct DirEntity {
    files: Vec<FileEntity>,
    dirs: Vec<path::PathBuf>,
}

fn get_dir_entity(path: path::PathBuf) -> Option<DirEntity> {
    match fs::read_dir(path) {
        Ok(i) => {
            let mut files: Vec<FileEntity> = Vec::new();
            let mut dirs: Vec<path::PathBuf> = Vec::new();
            let _: Vec<_> = i
                .map(|r| match r {
                    Ok(dir_entry) => {
                        // println!("Walking through {}", dir_entry.path().display());
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
            keep_biggest_files(&mut files);
            Some(DirEntity { files, dirs })
        }
        Err(_) => None,
    }
}

fn keep_biggest_files(files: &mut Vec<FileEntity>) {
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files.truncate(TOP);
}

fn walk_dir_entity(path: path::PathBuf) -> Vec<FileEntity> {
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
        keep_biggest_files(&mut files);
    }
    files
}

pub fn biggest_files_in_dir(path: path::PathBuf) -> Vec<FileEntity> {
    walk_dir_entity(path)
}
