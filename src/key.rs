use anyhow::Result;
use openssl::encrypt::{Decrypter, Encrypter};
use openssl::pkey::{HasPrivate, HasPublic, PKey, Private, Public};
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
        encrypt_public_base64(&self.key, data)
    }

    pub fn decrypt(&self, data: &str) -> Result<String> {
		decrypt_private_base64(&self.key,data.as_bytes()) 
    }
}

pub struct PublicKey {
    key: PKey<Public>,
}

impl PublicKey {
    pub fn from_pem(pem: &[u8]) -> Result<PublicKey> {
        let key = PKey::public_key_from_pem(pem)?;
        Ok(PublicKey { key })
    }
    pub fn encrypt(&self, data: &[u8]) -> Result<String> {
        encrypt_public_base64(&self.key, data)
    }
}

fn decrypt_private_base64<T>(key:&PKey<T>,data: &[u8]) -> Result<String> where T: HasPrivate{
	let data = base_decode(data)?;
	let mut decrypter = Decrypter::new(key).unwrap();
	decrypter.set_rsa_padding(Padding::PKCS1).unwrap();
	let buffer_len = decrypter.decrypt_len(&data).unwrap();
	let mut decrypted = vec![0; buffer_len];
	let decrypted_len = decrypter.decrypt(&data, &mut decrypted).unwrap();
	decrypted.truncate(decrypted_len);
	let string = String::from_utf8(decrypted)?;
	Ok(string)
}

fn encrypt_public_base64<T>(key: &PKey<T>, data: &[u8]) -> Result<String>
where
    T: HasPublic,
{
    let mut encrypter = Encrypter::new(key).unwrap();
    encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
    let buffer_len = encrypter.encrypt_len(data).unwrap();
    let mut encrypted = vec![0; buffer_len];
    let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
    encrypted.truncate(encrypted_len);
    Ok(base_encode(&encrypted))
}

fn base_encode(data:&[u8]) -> String {
	base64::encode(&base64::encode(&data))
}

fn base_decode(data:&[u8]) -> Result<Vec<u8>> {
	let data =  base64::decode(&base64::decode(data)?)?;	
	Ok(data)
}