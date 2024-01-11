use std::fs;
use std::fs::File;
use std::path::Path;
//TODO: Make this function write file and put the encryption here

pub fn write_file(filename: &String, password: String) -> std::io::Result<()> {
    let exists = Path::new(filename).exists();

    if exists == true {
        fs::write(filename, password).expect("ERR: unable to write to file");
    } else {
        File::create(filename)?;
        fs::write(filename, password).expect("ERR: unable to write to file");
    }

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
