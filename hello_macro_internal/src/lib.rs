pub trait HelloMacro {
    fn hello_macro();
}

pub use hello_macro_impl::HelloMacro;
