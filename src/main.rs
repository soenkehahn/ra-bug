fn main() {}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test(_ in 0..10) }
    }
}
