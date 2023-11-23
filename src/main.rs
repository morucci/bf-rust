use std::path;

use bf_rust;

fn main() {
    let path = path::Path::new("/tmp/").to_path_buf();

    let bf = bf_rust::biggest_files_in_dir(path);

    println!("{:?}", bf)
}
