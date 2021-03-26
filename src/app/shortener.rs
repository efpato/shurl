type Result<T> = std::result::Result<T, ShortenerError>;

#[derive(Debug, Clone)]
pub struct ShortenerError;

#[derive(Clone)]
pub struct Shortener {
    alphabet: Vec<char>,
}

impl Default for Shortener {
    fn default() -> Self {
        Shortener {
            alphabet: vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            ],
        }
    }
}

impl Shortener {
    pub fn new() -> Self {
        Shortener::default()
    }

    pub fn encode(&self, num: i64) -> String {
        if num < 0 {
            return String::new();
        }

        if num == 0 {
            return self.alphabet[0].to_string();
        }

        let len = self.alphabet.len();
        let mut num = num as usize;
        let mut hash: Vec<char> = vec![];

        while num > 0 {
            let rem = num.rem_euclid(len);
            num = num.div_euclid(len);
            hash.insert(0, self.alphabet[rem])
        }

        hash.into_iter().collect()
    }

    pub fn decode(&self, hash: String) -> Result<i64> {
        if hash == "" {
            return Err(ShortenerError);
        }

        let mut num: usize = 0;

        for (i, ch) in hash.chars().enumerate() {
            let index = self
                .alphabet
                .iter()
                .position(|&c| c == ch)
                .ok_or(ShortenerError)?;

            num = num + index * self.alphabet.len().pow((hash.len() - 1 - i) as u32);
        }

        Ok(num as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! encode_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let shortener = Shortener::default();

                    assert_eq!(shortener.encode(input), String::from(expected));
                }
            )*
        };
    }

    encode_test! {
        encode_0: (0, "a"),
        encode_1: (1, "b"),
        encode_100: (100, "bM"),
        encode_min: (i64::MIN, ""),
        encode_max: (i64::MAX, "k9viXaIfiWh"),
    }

    macro_rules! decode_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() -> Result<()> {
                    let (input, expected) = $value;
                    let shortener = Shortener::default();

                    assert_eq!(shortener.decode(String::from(input))?, expected);
                    Ok(())
                }
            )*
        };
    }

    decode_test! {
        decode_0: ("a", 0),
        decode_1: ("b", 1),
        decode_100: ("bM", 100),
        decode_max: ("k9viXaIfiWh", i64::MAX),
    }

    macro_rules! decode_error {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                #[should_panic]
                fn $name() {
                    let shortener = Shortener::default();

                    shortener.decode(String::from($value)).unwrap();
                }
            )*
        };
    }

    decode_error! {
        decode_empty_str: "",
        decode_invalid_str: "abc$",
    }
}
