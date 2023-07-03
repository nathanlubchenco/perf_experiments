use serde_json;
use std::fs::File;
use std::vec::Vec;

pub fn load_vectors(filename: &str) -> std::io::Result<Vec<Vec<f32>>> {
    let file = File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let vectors: Vec<Vec<f32>> = serde_json::from_reader(reader).unwrap();
    Ok(vectors)
}