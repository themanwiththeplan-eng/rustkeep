use passwords::PasswordGenerator;
use std::fs::File;
// use std::io::Read;
use std::{env, fs};

mod encdec;

// FIXME: We should only need 3 args here. (filename, name of password, and encrypt/decrypt)

fn main() {
    let args: Vec<String> = env::args().collect();

    let length = &args[1];
    let filename = &args[2];
    let passname = &args[3];
    let encdec_args = &args[4];
    // This string is for testing only for the encryption/decryption functions, delete this once
    // you get this working with entire files
    let test_string = &args[5];

    encdec::encdec(test_string, encdec_args);

    write_file(length.parse::<usize>().unwrap(), filename, passname)
        .expect("ERR: couldn't write to file");
}
fn create_password(
    length: usize,
    numbers: bool,
    lower: bool,
    upper: bool,
    symbols: bool,
    spaces: bool,
    exclude: bool,
    strict: bool,
) -> String {
    let pg = PasswordGenerator::new()
        .length(length)
        .numbers(numbers)
        .lowercase_letters(lower)
        .uppercase_letters(upper)
        .symbols(symbols)
        .spaces(spaces)
        .exclude_similar_characters(exclude)
        .strict(strict);
    let pass = pg.generate_one().unwrap();
    pass
}

//TODO: Make this function write file and put the encryption here

fn write_file(length: usize, filename: &String, name: &String) -> std::io::Result<()> {
    let file = filename;
    let pass = create_password(length, true, true, true, true, false, true, false);

    let name_string = name.to_string();
    let namepass = format!("{name_string}:{pass}");
    File::create(file)?;
    fs::write(file, namepass)?;

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
