#[macro_export]
macro_rules! vecstring{
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
    ($($x:expr,)*) => (vecstring![$($x:expr),*])
}

#[macro_export]
macro_rules! vecnd {
    ($([$($inner:tt)*]),+ $(,)?) => {
        vec![$(
            vecnd![$($inner)*]
        ),+]
    };
    ($($t:tt)*) => {
        vec![$($t)*]
    };
}

#[macro_export]
macro_rules! vecndstring {
    ($([$($inner:tt)*]),+ $(,)?) => {
        vec![$(
            vecndstring![$($inner)*]
        ),+]
    };
    ($($t:tt)*) => {
        vecstring![$($t)*]
    };
}
