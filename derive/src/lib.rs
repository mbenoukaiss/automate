#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

macro_rules! compile_error {
    (err $tokens:expr, $msg:literal) => {
        return Err(::syn::Error::new_spanned($tokens, $msg)
                .to_compile_error()
                .into());
    };
    (err $msg:literal) => {
        return Err(::syn::Error::new(::proc_macro2::Span::call_site(), $msg)
                .to_compile_error()
                .into());
    };
    ($tokens:expr, $msg:literal) => {
        return ::syn::Error::new_spanned($tokens, $msg)
                .to_compile_error()
                .into();
    };
    ($msg:literal) => {
        return ::syn::Error::new(::proc_macro2::Span::call_site(), $msg)
                .to_compile_error()
                .into();
    };
}

macro_rules! unwrap {
    ($input:expr) => {
        match $input {
            Ok(v) => v,
            Err(ts) => return ts
        }
    }
}

mod attribute;
mod derive;
mod macros;
mod utils;

//doc in automate's lib.rs
#[proc_macro_derive(State)]
pub fn state(input: TokenStream) -> TokenStream {
    derive::state(input)
}

//doc in automate's lib.rs
#[proc_macro_derive(Stored, attributes(storage))]
pub fn stored(input: TokenStream) -> TokenStream {
    derive::stored(input)
}

//doc in automate's lib.rs
#[proc_macro_derive(Storage)]
pub fn storage(input: TokenStream) -> TokenStream {
    derive::storage(input)
}

#[proc_macro_attribute]
pub fn object(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attribute::object(metadata, item)
}

#[proc_macro_attribute]
pub fn payload(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attribute::payload(metadata, item)
}

#[proc_macro_attribute]
pub fn convert(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attribute::convert(metadata, item)
}

#[proc_macro_attribute]
pub fn stringify(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attribute::stringify(metadata, item)
}

#[proc_macro_attribute]
pub fn endpoint(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attribute::endpoint(metadata, item)
}

/// An event listener function.
/// The function takes two arguments, the first being the
/// session which contains data about the bot and methods
/// to send instructions to discord. The second argument
/// is the event dispatch which contains data about the
/// event.
/// The library will call this function each time it
/// receives an event of the type of the second argument.
///
/// # Example
/// ```ignore
/// use automate::{Session, Error, listener};
/// use automate::gateway::MessageCreateDispatch;
///
/// #[listener]
/// async fn hello(_: &Context, _: &MessageCreateDispatch) -> Result<(), Error> {
///     println!("Hello!");
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn listener(metadata: TokenStream, item: TokenStream) -> TokenStream {
    attribute::listener(metadata, item)
}

//doc in automate's lib.rs
#[proc_macro]
pub fn functions(input: TokenStream) -> TokenStream {
    macros::functions(input)
}

//doc in automate's lib.rs
#[proc_macro]
pub fn stateless(input: TokenStream) -> TokenStream {
    macros::functions(input)
}

//doc in automate's lib.rs
#[proc_macro]
pub fn methods(input: TokenStream) -> TokenStream {
    macros::methods(input)
}
