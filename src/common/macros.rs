#[macro_export]
macro_rules! vec_concat {
    ($a: ident, $b: ident) => (
        $a.into_iter().chain($b.into_iter())
    )
}

pub use vec_concat;
