use std::{
    env,
    path::{Path, PathBuf},
};
use yaml_reader::add;
fn main() { 
    let current_path = env::current_dir().unwrap();
    let file_path = Path::new("example/simple.yaml");
    let yaml_path = file_path.strip_prefix(&current_path).unwrap_or_else(|_|file_path);
    println!("{}",yaml_path.display());
    println!("example: add{}", add(1, 2));
}
