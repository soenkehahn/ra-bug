fn main() {}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    fn foo(_: i32) {}

    proptest! {
        #[test]
        fn bar(ast in 0..100) {
            foo(ast);
        }
    }
}
