// This is a simple macro named `say_hello`.
#[macro_export] macro_rules! Str {
    ($str_slice:expr) => {
        stringify!($str_slice)
    };
}