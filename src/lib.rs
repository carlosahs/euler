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

    pub struct Digit {
        token_set: TokenSet,
        valid: char,
    }
}
