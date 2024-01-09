use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub fn encdec(buffer: &String, args: &String) {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("ERR: couldn't create private key");

    println!("{:#?}", private_key);

    let public_key = RsaPublicKey::from(&private_key);

    let data = buffer.as_bytes();
    let enc = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("ERR: failed to encrypt");

    //FIXME: Move the below to main.rs if possible
    //FIXME: Make this a match statement

    if args == "encrypt" {
        println!("{:#?}", enc);
    } else {
        let dec = private_key
            .decrypt(Pkcs1v15Encrypt, &enc)
            .expect("ERR: failed to decrypt");
        println!("{:#?}", dec);
    }
}
