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

    let pass = create_password(
        length.parse::<usize>().unwrap(),
        true,
        true,
        true,
        true,
        true,
        true,
        true,
    );

    let together = format!("{passname}:{pass}");

    let encrypted_pass = encdec::encdec(together, encdec_args);

    //FIXME: Fix this to take a string rather than a result somehow

    file::write_file(filename, encrypted_pass).expect("ERR: unable to write file");
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
