use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~,.?/[]{}|-_=+()@#$%^&*:;";

pub struct Password {
    no_symbols: bool,
    len: usize,
}

impl Password {
    pub fn new(len: usize) -> Password {
        Password {
            no_symbols: false,
            len,
        }
    }
    pub fn which_length(mut self, len: usize) -> Self {
        self.len = len;
        self
    }
    pub fn witch_no_symbol(mut self, no_symbol: bool) -> Self {
        self.no_symbols = no_symbol;
        self
    }
    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let charset = if self.no_symbols {
            CHARSET
        } else {
            CHARSET_SYMBOLS
        };
        let password: String = (0..self.len)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
        password
    }
}


pub struct PasswordIterator {
    password: Password
}

// impl IntoIterator for Password {
//     type Item = String;
//     type IntoIter = PasswordIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         PasswordIterator{password:self}
//     }
// }

impl Iterator for PasswordIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.password.generate())
    }
}
impl Iterator for Password {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}

#[cfg(test)]
mod test {
    use super::Password;


    #[test]
    fn test_preparations() {
        for pass in Password::new(20).witch_no_symbol(true).take(4) {
            println!("{}",pass);
        }
        Password::new(20).take(4).for_each(|p| println!("{}",p));
        // let password = Password::new(10);
        // password.into_iter().take(2).for_each(|p| println!("{}",p));
    
    }
}