//@run-rustfix
#![warn(clippy::explicit_into_iter_loop)]

fn main() {
    // Issue #4958
    fn _takes_iterator<T>(iterator: &T)
    where
        for<'a> &'a T: IntoIterator<Item = &'a String>,
    {
        for i in iterator.into_iter() {
            println!("{}", i);
        }
    }

    struct T;
    impl IntoIterator for &T {
        type Item = ();
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            vec![].into_iter()
        }
    }

    let t = T;
    let r = &t;
    let rr = &&t;

    // This case is handled by `explicit_iter_loop`. No idea why.
    for _ in t.into_iter() {}

    for _ in r.into_iter() {}

    // No suggestion for this.
    // We'd have to suggest `for _ in *rr {}` which is less clear.
    for _ in rr.into_iter() {}

    // Issue #6900
    struct S;
    impl S {
        #[allow(clippy::should_implement_trait)]
        pub fn into_iter<T>(self) -> I<T> {
            unimplemented!()
        }
    }

    struct I<T>(T);
    impl<T> Iterator for I<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!()
        }
    }

    for _ in S.into_iter::<u32>() {}
}
