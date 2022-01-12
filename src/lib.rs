#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod semantics {
    enum TokenSet {
        Number,
        Operator,
        Invalid,
    }

    pub trait Number {
        fn numeric(&self);
    }

    pub struct Digit {
        _token_set: TokenSet,
        value: char,
    }

    impl Digit {
        pub fn new(value: char) -> Option<Digit> {
            if value.is_digit(10) {
                return Some(Digit {
                    _token_set: TokenSet::Number,
                    value,
                });
            }

            None
        }
    }
}
