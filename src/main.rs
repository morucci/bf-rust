use std::path;

use bf_rust;

fn main() {
    println!("Hello, world!");

    let path = path::Path::new("/tmp/").to_path_buf();

    let dir_entity = bf_rust::walk_dir_entity(path);

    println!("{:?}", dir_entity)
}
