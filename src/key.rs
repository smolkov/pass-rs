use anyhow::Result;
use openssl::pkey::{PKey,Private};
use openssl::rsa::Rsa;

pub struct PrivateKey {
	key: PKey<Private>
}


impl PrivateKey {
	pub fn generate_rsa(bits:u32) -> Result<PrivateKey>{
		let rsa = Rsa::generate(bits)?;
		let key = PKey::from_rsa(rsa)?;
		Ok(PrivateKey{key})
	}
	pub fn from_pem(pem:&[u8]) -> Result<PrivateKey> {
		let key = PKey::private_key_from_pem(pem)?;
		Ok(PrivateKey { key })
	}
	pub fn private_pem(&self) -> Result<String> {
		let pem = self.key.private_key_to_pem_pkcs8()?;
		let string = String::from_utf8(pem)?;
		Ok(string)
	}
	pub fn public_pem(&self)-> Result<String> {
		let pem = self.key.public_key_to_pem()?;
		let string = String::from_utf8(pem)?;
		Ok(string)
	}
}