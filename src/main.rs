use std::{fs, path};

fn main() {
    println!("Hello, world!");

    let path = String::from("/tmp/clouds.yaml");

    match get_file_size(path) {
        Some(size) => println!("{size} bytes"),
        None => println!("Unable to compute file size"),
    }

    let path = String::from("/tmp/");

    let files_paths = get_dir_files(path);

    println!("{:?}", files_paths)
}

fn get_file_size(path: String) -> Option<u64> {
    match fs::metadata(path) {
        Ok(metadata) => Some(metadata.len()),
        Err(_) => None,
    }
}

#[derive(Debug)]
struct FileEntity {
    path: path::PathBuf,
    size: u64,
}

struct DirEntity {
    files: Vec<FileEntity>,
    dir: Vec<String>,
}

fn get_dir_files(path: String) -> Vec<FileEntity> {
    match fs::read_dir(path) {
        Ok(i) => {
            let mut filtered: Vec<FileEntity> = Vec::new();
            let mut raw: Vec<_> = i
                .map(|r| match r {
                    Ok(dir_entry) => Some(FileEntity {
                        path: dir_entry.path(),
                        size: dir_entry.metadata().unwrap().len(),
                    }),
                    Err(_) => None,
                })
                .collect();
            while let Some(Some(pb)) = raw.pop() {
                filtered.push(pb)
            }
            filtered
        }
        Err(_) => vec![],
    }
}
