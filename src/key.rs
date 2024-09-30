use anyhow::Result;
use openssl::encrypt::{Decrypter, Encrypter};
use openssl::pkey::{PKey, Private};
use openssl::rsa::{Padding, Rsa};

pub struct PrivateKey {
    key: PKey<Private>,
}

impl PrivateKey {
    pub fn generate_rsa(bits: u32) -> Result<PrivateKey> {
        let rsa = Rsa::generate(bits)?;
        let key = PKey::from_rsa(rsa)?;
        Ok(PrivateKey { key })
    }

    pub fn from_pem(pem: &[u8]) -> Result<PrivateKey> {
        let key = PKey::private_key_from_pem(pem)?;
        Ok(PrivateKey { key })
    }

    pub fn private_pem(&self) -> Result<String> {
        let pem = self.key.private_key_to_pem_pkcs8()?;
        let string = String::from_utf8(pem)?;
        Ok(string)
    }

    pub fn public_pem(&self) -> Result<String> {
        let pem = self.key.public_key_to_pem()?;
        let string = String::from_utf8(pem)?;
        Ok(string)
    }
    pub fn encrypt(&self, data: &[u8]) -> Result<String> {
        let mut encrypter = Encrypter::new(&self.key).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        let buffer_len = encrypter.encrypt_len(data).unwrap();
        let mut encrypted = vec![0; buffer_len];
        let encrypted_len = encrypter
            .encrypt(data, &mut encrypted)
            .unwrap();
        encrypted.truncate(encrypted_len);
        Ok(base_encode(&encrypted))
    }

    pub fn decrypt(&self, data: &str) -> Result<String> {
        let data = base_decode(data)?;
        let mut decrypter = Decrypter::new(&self.key).unwrap();
        decrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        let buffer_len = decrypter.decrypt_len(&data).unwrap();
        let mut decrypted = vec![0; buffer_len];
        let decrypted_len = decrypter.decrypt(&data, &mut decrypted).unwrap();
        decrypted.truncate(decrypted_len);
        let string = String::from_utf8(decrypted)?;
        Ok(string)
    }
}

fn base_encode(data: &[u8]) -> String {
    openssl::base64::encode_block(data)
}

fn base_decode(data: &str) -> Result<Vec<u8>> {
    let data = openssl::base64::decode_block(data)?;
    Ok(data)
}



#[cfg(test)] 
mod test {

    use super::*;
    use crate::generator::PasswordGenerator;


    #[test]
    fn encode_decode_password() {
        let pass = PasswordGenerator::new(30).generate();
        let key = PrivateKey::generate_rsa(4094).unwrap();
        let encrypt_string = key.encrypt(pass.as_bytes()).unwrap();
        let decrypt_string = key.decrypt(&encrypt_string).unwrap();
        assert_eq!(decrypt_string,pass);
    }

}