use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~,.?/\\[]{}|-_=+()@#$%^&*:;'\"<>";


pub struct PasswordGenerator {
    no_symbols: bool,
    len: usize,
}

impl PasswordGenerator {
    pub fn new(len: usize) -> PasswordGenerator {
        PasswordGenerator {
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
    password: PasswordGenerator
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
impl Iterator for PasswordGenerator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}

#[cfg(test)]
mod test {
    use super::PasswordGenerator;


    #[test]
    fn test_preparations() {
        const TEST_SYMBOLS:&str = "`~,.?/\\[]{}|-_=+()@#$%^&*:;'\"<>";
        for pass in PasswordGenerator::new(20).witch_no_symbol(true).take(4) {
            println!("{}",pass);
        }
        PasswordGenerator::new(20).take(4).for_each(|p| println!("{}",p));
        PasswordGenerator::new(20).take(4).for_each(|p| println!("{}",p.chars().filter(|ch| TEST_SYMBOLS.find(*ch).is_some()).fold(0, |acc, _| acc + 1)));

        // let password = Password::new(10);
        // password.into_iter().take(2).for_each(|p| println!("{}",p));
    
    }
}