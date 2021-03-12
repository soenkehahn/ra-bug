fn main() {}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    proptest! {
        #[test]
        // deleting the opening curly brace (`{`)
        // in the following line makes
        // rust-analyzer hang.
        fn test(_ in 0..10) {}
    }
}
