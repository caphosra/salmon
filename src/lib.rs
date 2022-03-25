use std::pin::Pin;

use futures::Future;

pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T>>>;

#[macro_export]
macro_rules! salmon {
    (pub $fn_name:tt : $ret:ty = |$($name:tt),*| $body:block) => {
        pub fn $fn_name() -> $crate::PinnedFuture<$ret> {
            Box::pin(async {
                let ($($name),*) = futures::join!($($name ()),*);
                $body
            })
        }
    };
    ($fn_name:tt : $ret:ty = |$($name:tt),*| $body:block) => {
        fn $fn_name() -> $crate::PinnedFuture<$ret> {
            Box::pin(async {
                let ($($name),*) = futures::join!($($name ()),*);
                $body
            })
        }
    };
    (pub $fn_name:tt : $ret:ty = $body:block) => {
        pub fn $fn_name() -> $crate::PinnedFuture<$ret> {
            Box::pin(async {
                $body
            })
        }
    };
    ($fn_name:tt : $ret:ty = $body:block) => {
        fn $fn_name() -> $crate::PinnedFuture<$ret> {
            Box::pin(async {
                $body
            })
        }
    };
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    mod flush {
        salmon!(pub one: u32 = { 1 });
    }

    use flush::one;

    salmon!(three: u32 = { 3 });

    salmon!(
        pub four: u32 = |one, three| {
            one + three
        }
    );

    #[test]
    fn salmon_test_1() {
        let f = block_on(four());
        assert_eq!(f, 4);
    }
}
