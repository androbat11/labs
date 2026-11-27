use std::fs;
use std::env;

#[derive(Debug)]
pub struct File {
    pub path: String,
}

impl File {
    pub fn open(path: String) -> File {
        File { path }
    }
}