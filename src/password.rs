use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const CHARSET_SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~,.?/[]{}|-_=+()@#$%^&*:;";

pub struct Generator {
    no_symbols: bool,
    len: usize,
}

impl Generator {
    pub fn new(len: usize) -> Generator {
        Generator {
            no_symbols: false,
            len,
        }
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
        println!("{:?}", password);
        password
    }
}
