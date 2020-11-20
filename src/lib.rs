/*!
# `::dyn_safe`

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/danielhenrymantilla/dyn_safe.rs)
[![Latest version](https://img.shields.io/crates/v/dyn_safe.svg)](https://crates.io/crates/dyn_safe)
[![Documentation](https://docs.rs/dyn_safe/badge.svg)](https://docs.rs/dyn_safe)
[![MSRV](https://img.shields.io/badge/MSRV-1.42.0-white)](https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/dyn_safe.svg)](https://github.com/danielhenrymantilla/dyn_safe.rs/blob/master/LICENSE-ZLIB)
<!-- [![CI](https://github.com/danielhenrymantilla/dyn_safe.rs/workflows/CI/badge.svg)](https://github.com/danielhenrymantilla/dyn_safe.rs/actions) -->

### Take control of the Semver hazard of the `dyn` safety of your traits!

##### Usage

 1. `cargo add dyn_safe`, or add the following to your `Cargo.toml` file:

    ```toml
    [dependencies]
    dyn_safe = "x.y.z"
    ```

      - where you can find the version using `cargo search dyn_safe`

 1. Add the following to your `lib.rs` file:

    ```rust,ignore
    #[macro_use]
    extern crate dyn_safe;
    ```

 1. Use `#[dyn_safe(true)]` or `#[dyn_safe(false)]` to, respectively,
    assert that the trait object is `dyn`-safe or that the trait
    object should not be `dyn`-safe.

      - ```rust,compile_fail
        # use ::dyn_safe::dyn_safe; macro_rules! ignore {($($t:tt)*) => ()} ignore! {
        #[macro_use]
        extern crate dyn_safe;
        # }

        #[dyn_safe(true)]
        trait Foo {
            fn whoops ();
        }
        ```

      - ```rust,compile_fail
        # use ::dyn_safe::dyn_safe; macro_rules! ignore {($($t:tt)*) => ()} ignore! {
        #[macro_use]
        extern crate dyn_safe;
        # }

        #[dyn_safe(false)]
        trait Foo {
            // …
        }

        let _: dyn Foo; // Whoops
        ```
*/

#![no_std]
#![forbid(unsafe_code)]

/// For a bug when cross-compiling
extern crate proc_macros;

pub use ::proc_macros::dyn_safe;

mod __ { pub struct Hidden(()); }

/// "Marker" (super-)trait used to opt-out of object safety.
pub
trait NotObjectSafe {
    #[doc(hidden)]
    fn __opted_out_of_object_safety<T>(_: __::Hidden) {
        impl<T : ?::core::marker::Sized> NotObjectSafe for T {}
    }
}
