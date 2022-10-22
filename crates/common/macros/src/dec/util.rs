#[macro_export]
macro_rules! into_fn {
    ($i:ident: $t:ty => $v:expr) => {
        fn $i() -> $t {
            ($v).into()
        }
    };

    ($i:ident: const $t:ty => $v:expr) => {
        const fn $i() -> $t {
            $v
        }
    };
}

pub use into_fn;
