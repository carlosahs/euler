#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod utils {}

mod semantics {
    pub enum NumericBase {
        Decimal,
        Hexadecimal,
        Octal,
        Binary,
        //Radix(u32),
    }

    pub enum TokenSet {
        Number,
        Operator,
        Invalid,
    }

    impl NumericBase {
        fn radix(&self) -> u32 {
            match self {
                NumericBase::Decimal => 10,
                NumericBase::Hexadecimal => 16,
                NumericBase::Octal => 8,
                NumericBase::Binary => 2,
                //NumericBase::Radix(r) => *r,
            }
        }
    }

    // pub trait Number<T> {
    //     fn numeric(&self) -> T;
    // }

    pub struct Digit {
        _token_set: TokenSet,
        _radix: NumericBase,
        _value: char,
    }

    impl Digit {
        pub fn new(value: char, radix: NumericBase) -> Option<Digit> {
            if value.is_digit(radix.radix()) {
                return Some(Digit {
                    _token_set: TokenSet::Number,
                    _radix: radix,
                    _value: value,
                });
            }

            None
        }
    }

    // impl Number<u32> for Digit {
    //     fn numeric(&self) -> u32 {
    //         self._value.to_digit(self._radix)
    //     }
    // }
}
