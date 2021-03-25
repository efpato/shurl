use harsh::Harsh;

const _ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Clone)]
pub struct Shortener {
    generator: Harsh,
}

impl Shortener {
    pub fn new() -> Shortener {
        Shortener {
            generator: Harsh::builder().alphabet(_ALPHABET).build().unwrap(),
        }
    }

    pub fn encode(&self, id: u64) -> String {
        self.generator.encode(&[id])
    }

    pub fn decode(&self, hash: String) -> Result<u64, harsh::Error> {
        match self.generator.decode(hash) {
            Ok(v) => Ok(*v.first().unwrap()),
            Err(e) => Err(e),
        }
    }
}
