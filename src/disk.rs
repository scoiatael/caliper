use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::bezier;

#[derive(Serialize, Deserialize)]
pub struct Serialized {
    pub path: PathBuf,
    curves: Vec<Vec<f32>>,
}

impl Serialized {
    pub fn new(path: PathBuf, curves: &bezier::Curves) -> Self {
        Serialized {
            path: path,
            curves: curves.iter().map(|c| c.into()).collect(),
        }
    }

    pub fn curves(&self) -> bezier::Curves {
        use std::convert::TryInto;

        let mut cs = bezier::Curves::default();
        for c in self.curves.iter() {
            cs.push(c.try_into().expect("Failed to deserialize curve"))
        }
        cs
    }
}

pub fn save(s: Serialized, path: &Path) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&s).unwrap();
    let mut file = File::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IOErr(std::io::Error),
    BincodeErr(Box<bincode::ErrorKind>),
}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(err: Box<bincode::ErrorKind>) -> Error {
        Error::BincodeErr(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOErr(err)
    }
}

pub fn load(path: &Path) -> Result<Serialized, Error> {
    let mut file = File::open(path)?;
    // read the same file back into a Vec of bytes
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    let decoded: Serialized = bincode::deserialize(&buffer)?;
    Ok(decoded)
}
