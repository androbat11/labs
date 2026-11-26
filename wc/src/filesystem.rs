use std::fs;
use std::env;


pub struct File<'a> {
    pub path: &'a str,
}

// impl File<'a> {
//     pub fn open(path: &str) -> File {
//         File { path }
//     }
// }