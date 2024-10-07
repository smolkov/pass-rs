use serde::{Deserialize,Serialize};


#[derive(Debug,Deserialize,Serialize)]
struct Secret {
	user: String,
	password: String
}