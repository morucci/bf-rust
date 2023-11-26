use std::path;

use bf_rust;

const TOP: usize = 10;

fn main() {
    let path = path::Path::new("/usr/share").to_path_buf();

    let bf = bf_rust::biggest_files_in_dir(path, TOP);

    println!("{:?}", bf)
}
