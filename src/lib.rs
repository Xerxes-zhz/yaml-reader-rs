use serde_yaml;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


 pub fn file_parse(path: PathBuf) -> Result<Config, MyError> {
    let mut contents = String::new();
    let mut config_file = File::open(path)?;
    config_file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents).unwrap();
    return Ok(config);
} 
