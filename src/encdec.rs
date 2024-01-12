// use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
//
// // TODO: restructure this to be embedded in the other functions other than main, more specifically
// // the read and write file functions
//
// pub fn encdec(input: String, args: &str) {
//     let mut rng = rand::thread_rng();
//     let strength = 2048;
//
//     let privkey = RsaPrivateKey::new(&mut rng, strength).expect("ERR: couldn't create private key");
//
//     let buf = input.as_bytes();
//
//     let pubkey = RsaPublicKey::from(&privkey);
//     let enc = pubkey
//         .encrypt(&mut rng, Pkcs1v15Encrypt, &buf[..])
//         .expect("ERR: failed to encrypt");
//
//     match args {
//         "encrypt" => {
//             println!("{:?}", enc);
//         }
//         "e" => {
//             println!("{:?}", enc);
//         }
//         "decrypt" => {
//             let dec = privkey
//                 .decrypt(Pkcs1v15Encrypt, &enc)
//                 .expect("ERR: failed to decrypt");
//             println!("{:?}", dec);
//         }
//         "d" => {
//             let dec = privkey
//                 .decrypt(Pkcs1v15Encrypt, &enc)
//                 .expect("ERR: failed to decrypt");
//             println!("{:?}", dec);
//         }
//         _ => {}
//     }
// }
