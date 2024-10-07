use serde::{Deserialize,Serialize};


#[derive(Debug,Deserialize,Serialize)]
struct Secret {
	user: String,
	password: String
}


impl Secret {
	pub fn new(user:&str, password: &str)  -> Secret {
		Secret{
			user: user.to_owned(),
			password: password.to_owned(),
		}
	}
}