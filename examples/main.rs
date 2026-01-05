use std::fs;

fn main() {
    let mut paths = fs::read_dir("./examples/").unwrap();
    for path in paths {
        println!("Running test in file: {}", path.unwrap().path().display())
    }
}