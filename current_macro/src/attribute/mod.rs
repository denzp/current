macro_rules! fallback {
    ($input:expr) => {
        return ::proc_macro2::TokenStream::from($input).into_tokens();
    };
}

macro_rules! call_site_error {
    ($msg:expr) => {
        ::proc_macro::Diagnostic::spanned(
            ::proc_macro::Span::call_site(),
            ::proc_macro::Level::Error,
            $msg,
        )
    };
}

mod kernel;
pub use self::kernel::*;

mod current;
pub use self::current::*;
