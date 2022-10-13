#[macro_export]
macro_rules! key_type {
    ($e:ty) => {
        <$e as BasicEntity>::Key
    };
}

pub use key_type;
