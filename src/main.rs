use passwords::PasswordGenerator;
use std::env;
use std::io;

fn main() {
    let pass = create_password(17, true, true, true, true, false, true, false);

    println!("{}", pass);
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
    let args: Vec<String> = env::args().collect();

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

fn true_false(s: &str) {
    match s {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        _ => false,
    };
}

fn to_file(name: String, pass: String) {}
