use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::sign::{Signer, Verifier};
use std::str;

use anyhow::Result;

fn main() -> Result<()> {
    let rsa = Rsa::generate(4096)?;

    let privkey_pem = rsa.private_key_to_pem()?;
    let pubkey_pem = rsa.public_key_to_pem()?;
    println!("{:?}", str::from_utf8(privkey_pem.as_slice()).unwrap());
    println!("{:?}", str::from_utf8(pubkey_pem.as_slice()).unwrap());
    let data = b"foobar log message coming from user that may be know my public key";
    let mut buf = vec![0; rsa.size() as usize];
    let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();
    println!(
        "Encrypted len={} DATA:{:?}",
        encrypted_len,
        str::from_utf8(buf.as_slice())
    );

    let mut decryp_data = vec![0; rsa.size() as usize];
    rsa.private_decrypt(&buf, &mut decryp_data, Padding::PKCS1)
        .unwrap();
    println!("{:?}", str::from_utf8(decryp_data.as_slice()).unwrap());

    // let pub_key: Vec<u8> = privkey.public_key_to_pem().unwrap();
    Ok(())
}

fn signer() -> Result<()> {
    // Generate a keypair
    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();

    let data = b"hello, world!";
    let data2 = b"hola, mundo!";

    // Sign the data
    let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(data).unwrap();
    signer.update(data2).unwrap();
    let signature = signer.sign_to_vec().unwrap();

    // Verify the data
    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(data).unwrap();
    verifier.update(data2).unwrap();
    assert!(verifier.verify(&signature).unwrap());
    Ok(())
}
