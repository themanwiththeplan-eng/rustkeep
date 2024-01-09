use passwords::PasswordGenerator;
use std::fs::File;
// use std::path::Path;
use std::{env, fs};

mod encdec;

// FIXME: We should only need 3 args here. (filename, name of password, and encrypt/decrypt)

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let buffer = &args[2];
    let encdec_args = &args[3];

    encdec::encdec(buffer, encdec_args);

    to_file(name).expect("ERR: couldn't write to file");
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

fn to_file(name: &String) -> std::io::Result<()> {
    let file = "new.pwd";
    let pass = create_password(17, true, true, true, true, false, true, false);

    let name_string = name.to_string();
    let namepass = format!("{name_string}:{pass}");
    File::create(file)?;
    fs::write(file, namepass)?;

    Ok(())
}

//TODO: Make a function read file and put decryption here
