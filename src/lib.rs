#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod semantics {
    enum NumericBase {
        Decimal,
        Hexadecimal,
        Octal,
        Binary,
        Radix(u32),
    }

    impl NumericBase {
        fn radix(&self) -> u32 {
            match self {
                NumericBase::Decimal => 10,
                NumericBase::Hexadecimal => 16,
                NumericBase::Octal => 8,
                NumericBase::Binary => 2,
                NumericBase::Radix(r) => *r,
            }
        }
    }

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
        _value: char,
    }

    impl Digit {
        pub fn new(value: char) -> Option<Digit> {
            if value.is_digit(10) {
                return Some(Digit {
                    _token_set: TokenSet::Number,
                    _value: value,
                });
            }

            None
        }
    }
}
