use rand::Rng;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

pub fn generate_vector(size: &usize, min: &f32, max: &f32) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for _ in 0..*size {
        vec.push(rng.gen_range(*min..*max))
    }
    vec
}

pub fn generate_vectors(count: &usize, size: &usize, min: &f32, max: &f32) -> Vec<Vec<f32>> {
    let mut vectors = Vec::new();

    for _ in 0..*count {
        let vec = generate_vector(size, min, max);
        vectors.push(vec)
    }
    vectors

}

pub fn write_vectors(filename: &String, vectors: &Vec<Vec<f32>>) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let data = serde_json::to_string(&vectors).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(data.as_bytes())?;

    Ok(())

}