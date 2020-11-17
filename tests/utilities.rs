// -- utilities.rs --

#[allow(dead_code)]
pub(crate) fn print_type<T: std::any::Any>() {
    println!(
        "{}: {{\n  {:?},\n  size: {}\n}}",
        std::any::type_name::<T>(),
        std::any::TypeId::of::<T>(),
        std::mem::size_of::<T>(),
    )
}

#[allow(unused_macros)]
macro_rules! show_type {
    ($t:ty) => { utilities::print_type::<$t>(); };
    ($t:ty,) => { show_type!($t) };
    ($($t:ty),+ $(,)?) => {
        ($(show_type!($t)),+,)
    };
}
