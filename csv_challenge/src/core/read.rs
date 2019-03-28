// use std::path::PathBuf;
// use std::fs::File;
// use super::Error;
// use std::io::prelude::*;
use super::{Error, File, PathBuf, Read, Write};

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}

pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
    write(csv_data, filename)?;
    Ok(())
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        return Err("Input file missing")?;
    }
    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn write(data: &str, file_name: &str) -> Result<(), Error> {
    let mut buffer = File::create(file_name)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::load_csv;
    use std::path::PathBuf;
    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
}
