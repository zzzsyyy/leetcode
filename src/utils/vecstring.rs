#[macro_export]
macro_rules! vecstring{
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
    ($($x:expr,)*) => (vecstring![$($x:expr),*])
}
