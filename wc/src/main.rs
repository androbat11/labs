mod filesystem;
mod wc;
mod cli;

use filesystem::File;
fn main() {
    let input_file = File::open("./input.txt".to_string());
    println!("{:?}", input_file.path);
}
