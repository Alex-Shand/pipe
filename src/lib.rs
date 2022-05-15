//! Postfix function application helpers

#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(dead_code)]
#![warn(clippy::pedantic)]

/// Postfix function application helpers
pub mod prelude {
    /// For functions that consume their arguments
    pub trait ValuePipe: Sized {
        #[allow(missing_docs)]
        fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
            f(self)
        }
    }

    impl<T> ValuePipe for T {}

    /// For functions that accept immutable references
    pub trait RefPipe: Sized {
        #[allow(missing_docs)]
        fn pipe<T>(&self, f: impl FnOnce(&Self) -> T) -> T {
            f(self)
        }
    }

    impl<T> RefPipe for T {}

    /// For functions that accept mutable references
    pub trait MutRefPipe: Sized {
        #[allow(missing_docs)]
        fn pipe<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
            f(self)
        }
    }

    impl<T> MutRefPipe for T {}
}
