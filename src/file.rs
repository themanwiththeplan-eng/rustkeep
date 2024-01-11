use std::fs::File;
// use std::io::Read;
// use std::fs;

// TODO: use a hashmap to store the k,v pairs rather than format string that just looks weird

//TODO: Make this function write file and put the encryption here

pub fn write_file(length: usize, filename: &String, name: &String) -> std::io::Result<()> {
    let file = filename;

    // let name_string = name.to_string();
    // File::create(file)?;

    Ok(())
}

//TODO: Make a function read file and put decryption here

// fn read_file(filename: &String) -> std::io::Result<()> {
//     let file = filename;
//     let content = fs::read_to_string(file).unwrap();
//
//     println!("{:?}", content);
//     Ok(())
// }
