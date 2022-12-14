use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(input_path: &Path) -> Vec<u8> {
    // Create a path to the desired file
    let display = input_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            // println!("read {} bytes from file {}", s.len(), display);
            return s.as_bytes().to_vec();
        }
    }
}

pub fn split_file(input_path: &Path, split_byte: u8) -> Vec<Vec<u8>> {
    let file_contents = read_file(input_path);
    let mut output_vec: Vec<Vec<u8>> = vec![];
    let mut file_part_vec: Vec<u8> = vec![];
    for b in file_contents {
        if b == split_byte {
            output_vec.push(file_part_vec);
            file_part_vec = vec![];
        } else {
            file_part_vec.push(b.clone());
        }
    }
    if file_part_vec.len() > 0 {
        output_vec.push(file_part_vec);
    }

    output_vec
}