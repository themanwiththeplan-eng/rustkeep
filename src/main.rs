use passwords::PasswordGenerator;
use std::env;

mod encdec;
mod file;
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
    file::write_file(length.parse::<usize>().unwrap(), filename, passname)
        .expect("ERR: could not write to file");
}

pub fn create_password(
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
