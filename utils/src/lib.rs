use std::{fs::File, io::Read, path::Path};


pub fn read_input_string<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut data = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut data)?;

    Ok(data)
}

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let string = read_input_string(path)?;

    Ok(string.lines().map(String::from).collect())
}
