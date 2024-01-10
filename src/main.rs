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
